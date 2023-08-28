use std::io::ErrorKind;
use std::str::FromStr;
use std::time::Duration;

use serde_derive::Deserialize;
use url::Url;

use crate::utils::download_or_reuse;

pub const APACHE_MAVEN_DIST_URL_BASE: &str = "https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven";
const APACHE_MAVEN_DIST_METADATA_URL: &str = "https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/maven-metadata.xml";

pub fn find_latest_maven_version() -> std::io::Result<String> {
    log::trace!("find_latest_maven_distribution");
    let metadata_xml = load_known_versions()?;
    let version = &metadata_xml.versioning.latest;
    log::debug!("Latest Maven release: {}", version);
    Ok(version.to_owned())
}

fn load_known_versions() -> std::io::Result<MavenMetadataXml> {
    let user_home = dir::home_dir().unwrap();
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
    Ok(meta)
}

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
