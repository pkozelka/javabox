use std::path::Path;
use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct JavaboxConfig {
    pub java: Option<JavaConfig>,
    pub maven: Option<MavenConfig>,
    pub gradle: Option<GradleConfig>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct JavaConfig {
    pub version: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MavenConfig {
    pub version: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GradleConfig {
    pub version: String,
}

const CONFIG_NAME: &'static str = "javabox.toml";

impl JavaboxConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config: JavaboxConfig = confy::load_path(CONFIG_NAME)?;
        Ok(config)
    }

    pub fn save(&self, dir: &Path) -> anyhow::Result<()> {
        let dir = dir.canonicalize()?;
        let config_file = dir.join(CONFIG_NAME);
        if let None = self.java {
            // no java tooling found, let's remove the entire config
            log::warn!("Not saving configuration to {} - no known java build tool detected", config_file.display());
            return Ok(());
        }
        confy::store_path(config_file, self)?;
        Ok(())
    }
}
