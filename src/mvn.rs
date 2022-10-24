//! # Maven Wrapper
//!
use std::collections::HashMap;
use std::env::current_dir;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

use dir::home_dir;
use url::Url;

use crate::{java_hash, utils};
use crate::utils::{download, download_or_reuse};

const APACHE_MAVEN_DIST_URL_BASE: &str     = "https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven";
const APACHE_MAVEN_DIST_METADATA_URL: &str = "https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/maven-metadata.xml";

pub fn run_mvn() -> std::io::Result<i32> {
    let current_dir = current_dir()?;
    let user_home = home_dir().expect("There is no HOME directory?!");
    // all ancestors containing modules
    let mut modules = Vec::new();
    // top of the SCM repository
    let mut scm_repo_root = None;
    let mut wrapper_dir = None;
    let mut wrapper_properties = HashMap::new();
    for d in current_dir.ancestors() {
        if scm_repo_root.is_none() {
            // we only care about these files _within_ scm repo, if one exists
            // ... and also _within_ wrapper, if one exists
            if wrapper_dir.is_none() && d.join("pom.xml").is_file() {
                modules.push(d);
                log::trace!("POM: {}", d.display());
            }
            if d.join("mvnw").is_file()
                || d.join("mvnw.bat").is_file()
                || d.join(".mvn").is_dir()
            {
                wrapper_dir = Some(d);
                log::trace!("WRAPPER: {}", d.display());
            }
            //
        }
        if utils::is_scm_wc_root(d) {
            scm_repo_root = Some(d);
            log::trace!("SCM WORKING COPY: {}", d.display());
        }

        //TODO: think about detecting
        // - .repository/ and .m2/repository/ ... can be passed to maven with -Dmaven.repository.local=XXX
        // - settings.xml and .m2/settings.xml ... can be passed to maven with --settings
        // - .m2/

        // stop scan at user home level
        if d == user_home {
            break;
        }
    }

    // TODO: estimate JDK version and use it as JAVA_HOME and in PATH

    // TODO: consider delegating to the existing wrapper, if it isn't myself
    // TODO: when on a nested module, we should perhaps go from project root and add options `-pl` with rel module and one of (`--also-make`, `--also-make-dependents`)
    // estimate maven version and use it
    let distribution_url = match wrapper_dir {
        Some(wrapper_dir) => {
            let props = wrapper_dir.join(".mvn/wrapper/maven-wrapper.properties");
            if props.exists() {
                utils::read_properties(&mut wrapper_properties, &props)?;
            }
            let url = wrapper_properties.get("distributionUrl")
                .expect("cannot find key 'distributionUrl'") //TODO: here is the place to use default - we don't have properties, but we do have mvnw
                .clone();
            if !url.starts_with(APACHE_MAVEN_DIST_URL_BASE) {
                log::warn!("Suspicious: this is not our known Apache Maven distribution location: {url}");
                // if we ever implement a paranoid mode, this could be a reason to stop
            }
            url
        }
        None => {
            // default=latest if not configured otherwise
            find_latest_maven_distribution(&user_home)?
        }
    };
    let maven_home = get_maven_home(&user_home, &distribution_url)?;
    log::debug!("Maven home: {}", maven_home.display());
    let launcher = maven_home.join("bin/mvn");

    let current_dir = current_dir.as_path(); // for use outside existing modules
    let project_dir = *modules.last().unwrap_or(&current_dir);
    let module_dir = *modules.first().unwrap_or(&current_dir);

    utils::execute_tool(&project_dir, &launcher.display().to_string(), &module_dir)
}

fn get_maven_home(user_home: &Path, distribution_url: &String) -> std::io::Result<PathBuf> {
    let distribution_url = Url::from_str(distribution_url)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidInput, format!("Bad URL: {distribution_url} :: {e:?}")))?;
    let upath = distribution_url.path();

    match upath.rfind('/') {
        None => {
            log::warn!("Invalid distribution URL: {distribution_url}");
            Err(std::io::Error::new(ErrorKind::Unsupported, format!("Strange distribution URL: {distribution_url}")))
        }
        Some(n) => {
            let zip_name = &upath[n + 1..];
            let base_name = zip_name.replace(".zip", "");
            let dist_name = base_name.replace("-bin", "");
            let url_hash = java_hash::java_uri_hash(&distribution_url);
            let maven_base = user_home.join(format!(".m2/wrapper/dists/{base_name}/{url_hash:x}"));
            let maven_home = maven_base.join(dist_name);
            if !maven_home.is_dir() {
                let zip_path = maven_base.join(zip_name);
                // if the zip is missing, download it first (verify checksums!)
                if !zip_path.is_file() {
                    std::fs::create_dir_all(maven_base)?;
                    download(&distribution_url, &zip_path)?;
                }
                let zip = std::fs::File::open(&zip_path)?;
                zip_extract::extract(zip, &maven_home, true)
                    .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, format!("Failed to extract file: {} :: {e:?}", zip_path.display())))?;
            }
            Ok(maven_home)
        }
    }
}

fn find_latest_maven_distribution(user_home: &Path) -> std::io::Result<String> {
    let metadata_xml = user_home.join(".m2/wrapper/dists/maven-metadata.xml");
    let url = APACHE_MAVEN_DIST_METADATA_URL;
    let url = Url::from_str(url)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidInput, format!("Failed to download maven-matadata.xml: {} :: {e:?}", url)))?;
    // reuse the file for some time, they don't release maven every hour
    // one day should be good enough
    download_or_reuse(&url, &metadata_xml, Duration::from_secs(3600 * 24))?;
    // extract the latest version
    let meta = std::fs::File::open(&metadata_xml)?;
    let meta: MavenMetadataXml = serde_xml_rs::from_reader(meta)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, format!("Invalid format of maven-matadata.xml :: {e:?}")))?;
    let version = &meta.versioning.latest;
    log::debug!("Latest Maven release: {}", version);
    Ok(format!("{APACHE_MAVEN_DIST_URL_BASE}/{version}/apache-maven-{version}-bin.zip"))
}

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "metadata")]
#[serde(rename_all = "camelCase")]
struct MavenMetadataXml {
    group_id: String,
    artifact_id: String,
    versioning: MetadataVersioning,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MetadataVersioning {
    latest: String,
    release: String,
    // versions: Vec<String>
}
