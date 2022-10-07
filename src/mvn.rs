//! # Maven Wrapper
//!
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
    let current = current_dir()?;
    let user_home = home_dir().expect("There is no HOME directory?!");
    // all ancestors containing pom.xml
    let mut modules = Vec::new();
    // top of the SCM repository
    let mut scm_repo_root = None;
    let mut wrapper = None;
    let mut wrapper_properties = None;
    for d in current.ancestors() {
        if scm_repo_root.is_none() {
            // we only care about these files _within_ scm repo, if one exists
            // ... and also _within_ wrapper, if one exists
            if wrapper.is_none() && d.join("pom.xml").is_file() {
                modules.push(d);
                log::trace!("POM: {}", d.display());
            }
            if d.join("mvnw").is_file()
                || d.join("mvnw.bat").is_file()
                || d.join(".mvn").is_dir()
            {
                wrapper = Some(d);
                log::trace!("WRAPPER: {}", d.display());
                let props = d.join(".mvn/wrapper/maven-wrapper.properties");
                if props.is_file() {
                    wrapper_properties = Some(utils::read_properties(&props)?);
                }
            }
            //
        }
        if d.join(".git/config").is_file()
            || d.join(".svn").is_dir()
        {
            scm_repo_root = Some(d);
            log::trace!("SCM WORKING COPY: {}", d.display());
        }

        //TODO: think about detecting
        // - .repository/ and .m2/repository/ ... can be passed to maven with -Dmaven.repository.local=XXX
        // - settings.xml and .m2/settings.xml ... can be passed to maven with --settings
        // - .m2/
    }
    let project_dir = *modules.last().expect("Not inside a maven module with pom.xml file found");
    let module_dir = modules[0];

    // TODO: consider delegating to the existing wrapper, if it isn't myself
    // TODO: when on a nested module, we should perhaps go from project root and add options `-pl` with rel module and one of (`--also-make`, `--also-make-dependents`)
    // estimate maven version and use it
    let distribution_url = match wrapper_properties {
        Some(wrapper_properties) => {
            let url = wrapper_properties.get("distributionUrl").expect("cannot find key 'distributionUrl'").clone();
            if !url.starts_with(APACHE_MAVEN_DIST_URL_BASE) {
                log::warn!("Suspicious: this is now our known Apache Maven distribution location: {url}");
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

    // TODO: estimate JDK version and use it as JAVA_HOME and in PATH
    utils::execute_tool(&project_dir, &launcher.display().to_string(), &module_dir)
}

fn get_maven_home(user_home: &Path, distribution_url: &String) -> std::io::Result<PathBuf> {
    let distribution_url = Url::from_str(distribution_url)
        .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("Bad URL: {distribution_url} :: {e:?}")))?;
    let upath = distribution_url.path();

    match upath.rfind('/') {
        None => {
            log::warn!("Strange distribution URL: {distribution_url}");
            Err(std::io::Error::new(ErrorKind::Other, format!("Strange distribution URL: {distribution_url}")))
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
                    .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("Failed to extract file: {} :: {e:?}", zip_path.display())))?;
            }
            Ok(maven_home)
        }
    }
}

fn find_latest_maven_distribution(user_home: &Path) -> std::io::Result<String> {
    let metadata_xml = user_home.join(".m2/wrapper/dists/maven-metadata.xml");
    let url = APACHE_MAVEN_DIST_METADATA_URL;
    let url = Url::from_str(url)
        .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("Failed to download maven-matadata.xml: {} :: {e:?}", url)))?;
    // reuse the file for some time, they don't release maven every hour
    // one day should be good enough
    download_or_reuse(&url, &metadata_xml, Duration::from_secs(3600 * 24))?;
    // extract the latest version
    let meta = std::fs::File::open(&metadata_xml)?;
    let meta: MavenMetadataXml = serde_xml_rs::from_reader(meta)
        .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("Invalid format of maven-matadata.xml :: {e:?}")))?;
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