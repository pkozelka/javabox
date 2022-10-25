use std::collections::HashMap;
use std::env::current_dir;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

use dir::home_dir;
use serde_derive::Deserialize;
use url::Url;
use crate::utils;

use crate::utils::{download, download_or_reuse};

const GRADLE_DIST_URL_BASE: &str = "https://services.gradle.org/distributions";
// + '"/gradle-6.5-all.zip"
const GRADLE_DIST_CURRENT_VERSION: &str = "https://services.gradle.org/versions/current"; // JSON

pub fn run_gradle() -> std::io::Result<i32> {
    // TODO lookup settings.gradle, gradle.properties, if useful
    let current_dir = current_dir()?;
    let user_home = home_dir().expect("There is no HOME directory?!");
    // all ancestors containing modules
    let mut modules = Vec::new();
    // top of the SCM repository
    let mut scm_repo_root = None;
    let mut wrapper_base = None; // the dir containing wrapper script
    let mut wrapper_properties = HashMap::new();
    for d in current_dir.ancestors() {
        if scm_repo_root.is_none() {
            // we only care about these files _within_ scm repo, if one exists
            // ... and also _within_ wrapper, if one exists
            if wrapper_base.is_none() {
                if d.join("build.gradle").is_file()
                    || d.join("build.gradle.kts").is_file() {
                    modules.push(d);
                    log::trace!("Module: {}", d.display());
                }
            }
            if d.join("gradlew").is_file()
                || d.join("gradlew.bat").is_file()
                || d.join("gradle/wrapper").is_dir()
            {
                wrapper_base = Some(d);
                log::trace!("WRAPPER: {}", d.display());
            }
            //
        }
        if utils::is_scm_wc_root(d) {
            scm_repo_root = Some(d);
            log::trace!("SCM WORKING COPY: {}", d.display());
        }

        // stop scan at user home level
        if d == user_home {
            break;
        }
    }

    // TODO: estimate JDK version and use it as JAVA_HOME and in PATH

    // TODO: consider delegating to the existing wrapper, if it isn't myself
    // estimate gradle version and use it
    let distribution_url = match wrapper_base {
        Some(wrapper_base) => {
            let props = wrapper_base.join("gradle/wrapper/gradle-wrapper.properties");
            if props.exists() {
                utils::read_properties(&mut wrapper_properties, &props)?;
            }
            let url = wrapper_properties.get("distributionUrl");
            if let Some(url) = url {
                if !url.starts_with(GRADLE_DIST_URL_BASE) {
                    log::warn!("Suspicious: this is not our known Gradle distribution location: {url}");
                    // if we ever implement a paranoid mode, this could be a reason to stop
                }
            }
            url
        }
        None => None
    };
    let distribution_url = match distribution_url {
        None => find_latest_gradle_distribution(&user_home)?, // default=latest if not configured otherwise
        Some(distribution_url) => distribution_url.clone()
    };
    let gradle_home = get_gradle_home(&user_home, &distribution_url)?;
    log::debug!("Gradle home: {}", gradle_home.display());
    let launcher = gradle_home.join("bin/gradle");

    let current_dir = current_dir.as_path(); // for use outside existing modules
    let project_dir = *modules.last().unwrap_or(&current_dir);
    let module_dir = *modules.first().unwrap_or(&current_dir);

    utils::execute_tool(&project_dir, &launcher.display().to_string(), &module_dir)
}

fn get_gradle_home(user_home: &Path, distribution_url: &String) -> std::io::Result<PathBuf> {
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
            let url_hash = utils::md5decimal(distribution_url.as_str());
            let gradle_base = user_home.join(format!(".gradle/wrapper/dists/{base_name}/{url_hash}"));
            let gradle_home = gradle_base.join(dist_name);
            if !gradle_home.is_dir() {
                let zip_path = gradle_base.join(zip_name);
                // if the zip is missing, download it first (verify checksums!)
                if !zip_path.is_file() {
                    std::fs::create_dir_all(gradle_base)?;
                    download(&distribution_url, &zip_path)?;
                }
                let zip = std::fs::File::open(&zip_path)?;
                zip_extract::extract(zip, &gradle_home, true)
                    .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, format!("Failed to extract file: {} :: {e:?}", zip_path.display())))?;
            }
            Ok(gradle_home)
        }
    }
}

fn find_latest_gradle_distribution(user_home: &Path) -> std::io::Result<String> {
    let metadata_xml = user_home.join(".gradle/wrapper/dists/current");
    let url = GRADLE_DIST_CURRENT_VERSION;
    let url = Url::from_str(url)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidInput, format!("Failed to download maven-matadata.xml: {} :: {e:?}", url)))?;
    // reuse the file for some time, they don't release maven every hour
    // one day should be good enough
    download_or_reuse(&url, &metadata_xml, Duration::from_secs(3600 * 24))?;
    // extract the latest version
    let current_gradle = std::fs::File::open(&metadata_xml)?;
    let current_gradle: CurrentVersionJson = serde_json::from_reader(current_gradle)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, format!("Invalid format of current JSON :: {e:?}")))?;
    log::debug!("Latest(Current) Gradle release: {} distribution: {}", current_gradle.version, current_gradle.download_url);
    // format!("{GRADLE_DIST_URL_BASE}/gradle-{version}-bin.zip")
    Ok(current_gradle.download_url)
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "metadata")]
#[serde(rename_all = "camelCase")]
struct CurrentVersionJson {
    version: String,
    current: bool,
    download_url: String,
    checksum_url: String,
}
