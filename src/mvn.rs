//! # Maven Wrapper
//!
use std::collections::HashMap;
use std::env::current_dir;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::str::FromStr;
use dir::home_dir;
use url::Url;

use crate::{java_hash, utils};
use crate::config::{JavaboxConfig, JavaConfig, MavenConfig};
use crate::mvn::centralrepo::APACHE_MAVEN_DIST_URL_BASE;
use crate::utils::download;

pub fn run_mvn_here() -> anyhow::Result<i32> {
    run_mvn(&current_dir()?)
}

pub fn run_mvn(cwd: &Path) -> anyhow::Result<i32> {
    log::trace!("run_mvn({})", cwd.display());
    let mvn_env = MavenEnv::load_or_infer(cwd)?;
    let exit_code = mvn_env.execute(cwd)?;
    Ok(exit_code)
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
                    let _ = std::fs::create_dir_all(maven_base);
                    download(&distribution_url, &zip_path)?;
                }
                let zip = std::fs::File::open(&zip_path)?;
                log::trace!("Extracting {} to {}", zip_path.to_string_lossy(), maven_home.to_string_lossy());
                zip_extract::extract(zip, &maven_home, true)
                    .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, format!("Failed to extract file: {} :: {e:?}", zip_path.display())))?;
            }
            log::debug!("maven_home={}", maven_home.display());
            Ok(maven_home)
        }
    }
}

struct MavenEnv {
    maven_bin: PathBuf,
    java_home: PathBuf,
    //TODO env, properties etc
}

impl MavenEnv {
    /// Finds directory with root pom.xml file
    pub fn load_or_infer(cwd: &Path) -> anyhow::Result<MavenEnv> {
        let user_home = home_dir().unwrap();
        let config = if JavaboxConfig::is_inside(cwd) {
            JavaboxConfig::load(cwd)?
        } else {
            infer_config(cwd)?
        };
        let maven = config.maven.as_ref().unwrap();
        // maven_version -> distributionUrl
        // maven_version -> MAVEN_HOME

        let maven_home = get_maven_home(&user_home, &maven.download_url)?;

        // determine maven_home directory based on maven_version and customizations
        // if empty:
        // - download maven if not downloaded yet
        // - expand downloaded to maven_home

        // download java if needed, pass it to JAVA_HOME and PATH
        // maybe other required tooling
        Ok(MavenEnv {
            maven_bin: maven_home.join("bin/mvn"),
            java_home: Default::default(),
        })
    }

    pub fn execute(&self, cwd: &Path) -> std::io::Result<i32> {
        log::info!("Running {} in project {}", self.maven_bin.display(), cwd.display());

        let mut command = std::process::Command::new(&self.maven_bin);
        command.current_dir(cwd);
        command.args(std::env::args().skip(1));
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        command.env("JAVA_HOME", self.java_home.display().to_string());
        let status = command.status()?;
        match status.code() {
            None => Err(std::io::Error::new(ErrorKind::BrokenPipe, "Interrupted")),
            Some(code) => Ok(code)
        }
    }
}

pub fn infer_config(cwd: &Path) -> anyhow::Result<JavaboxConfig> {
    let mut props = HashMap::new();
    let mwp = cwd.join(".mvn/wrapper/maven-wrapper.properties");
    if mwp.is_file() {
        utils::read_properties(&mut props, &mwp)?;
    }
    // config was not yet persisted
    let pom = cwd.join("pom.xml");
    if !pom.exists() {
        anyhow::bail!("No pom.xml file in {}", cwd.display());
    }
    // maven version: from wrapper or default
    // TODO: consider reading properties and compiler plugin config from pom.xml
    let maven_version = match maven_version_from_wrapper(props) {
        None => centralrepo::maven_last_stable_version()?,
        Some(maven_version) => maven_version
    };
    let download_url = format!("{APACHE_MAVEN_DIST_URL_BASE}/{maven_version}/apache-maven-{maven_version}-bin.zip").parse()?;
    let maven = MavenConfig {
        version: maven_version.to_string(),
        download_url,
    };
    let java_version = "1.8".to_string();
    Ok(JavaboxConfig {
        java: Some(JavaConfig { version: java_version }),
        maven: Some(maven),
        ..Default::default()
    })
}

fn maven_version_from_wrapper<'a>(props: HashMap<String, String>) -> Option<String> {
    match props.get("distributionUrl") {
        None => None,
        Some(dist) => {
            log::warn!("dist={dist}");
            match dist[APACHE_MAVEN_DIST_URL_BASE.len()+1..].split('/')
                // .skip(1)
                .next() {
                None => None,
                Some(version) => {
                    log::debug!("{dist} --> '{version}'");
                    Some(version.to_string())
                }
            }
        }
    }
}

mod centralrepo;
