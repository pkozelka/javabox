use url::Url;

pub(crate) fn download_jdk(version: &str) -> anyhow::Result<()> {
    let adoptium = AdoptiumApi::new();
    adoptium.info_binary_latest(version)?;
    Ok(())
}


// implementation of Adoptium REST API
// https://api.adoptium.net/q/swagger-ui/

struct AdoptiumApi {
    client: ureq::Agent,
}

const ADOPTIUM_OS: &str = if cfg!(target_os = "macos") {
    "mac"
} else {
    std::env::consts::OS
};

const ADOPTIUM_ARCH: &str = if cfg!(target_arch = "x86_64") {
    "x64"
} else if cfg!(target_arch = "powerpc") {
    "ppc"
} else if cfg!(target_arch = "powerpc64") {
    "ppc64"
} else {
    std::env::consts::ARCH
};

impl AdoptiumApi {
    fn new() -> Self {
        Self {
            client: ureq::AgentBuilder::new()
                .timeout_read(std::time::Duration::from_secs(30))
                .timeout_write(std::time::Duration::from_secs(30))
                .redirects(0)
                .build(),
        }
    }

    pub fn info_binary_latest(&self, version: &str) -> anyhow::Result<Url> {
        let image_type = "jdk";
        let jvm_impl = "hotspot";
        let heap_size = "normal";
        let vendor = "eclipse";
        let os = ADOPTIUM_OS;
        let arch = ADOPTIUM_ARCH;
        self.get_info_binary_latest(version, os, arch, image_type, jvm_impl, heap_size, vendor)
    }

    /// GET /v3/binary/version/{release_name}/{os}/{arch}/{image_type}/{jvm_impl}/{heap_size}/{vendor}
    /// Redirects to the binary that matches your current query.
    /// Matching CURL example:
    /// ```shell
    /// curl -v 'https://api.adoptium.net/v3/binary/latest/17/ga/mac/x64/jdk/hotspot/normal/eclipse'
    /// ```
    ///
    /// Params:
    /// - `feature_version`: The version of the JDK you want to download. This can be a major version (e.g. `8`) or a release name (e.g. `8u212-b03`).
    /// - `os`: The operating system you want to download. This can be:
    ///   * `linux`
    ///   * `mac`
    ///   * `windows`
    ///   * `aix`
    ///   * `solaris`
    /// - `arch`: The architecture you want to download. This can be:
    ///   * `x64`
    ///   * `x32`
    ///   * `x86`
    ///   * `ppc64`
    ///   * `ppc64le`
    ///   * `s390x`
    ///   * `aarch64`
    ///   * `arm`
    ///   * `sparcv9`
    ///   * `riscv64`
    /// - `image_type`: The type of image you want to download. This can be:
    ///   * `jdk`
    ///   * `jre`
    ///   * `testimage`
    ///   * `debugimage`
    ///   * `staticlibs`
    ///   * `source`
    ///   * `sbom`
    /// - `jvm_impl`: The JVM implementation you want to download. This can be:
    ///   * `hotspot`
    ///   * `openj9`
    /// - `heap_size`: The heap size you want to download. This can be:
    ///   * `normal`
    ///   * `large`
    /// - `vendor`: The vendor you want to download. This can be:
    ///   * `eclipse`
    ///
    fn get_info_binary_latest(&self, feature_version: &str, os: &str, arch: &str, image_type: &str, jvm_impl: &str, heap_size: &str, vendor: &str) -> anyhow::Result<Url> {
        let release_type = "ga";
        let url = format!("https://api.adoptium.net/v3/binary/latest/{}/{release_type}/{}/{}/{}/{}/{}/{}", feature_version, os, arch, image_type, jvm_impl, heap_size, vendor);
        log::info!("GET {url}");
        let response = self.client.get(&url)
            .call()?;
        if response.status() != 307 {
            anyhow::bail!("Expected 307, got {}", response.status());
        }
        // read the Location header
        let location = response.header("Location").unwrap();
        log::info!("Location: {}", location);
        let location: Url = location.parse()?;
        let file_name = location
            .path_segments().ok_or(anyhow::anyhow!("No path segments in URL"))?
            .last().ok_or(anyhow::anyhow!("No file name in URL"))?;
        log::info!("file_name: {}", file_name);
        Ok(location.to_owned())
    }
}
