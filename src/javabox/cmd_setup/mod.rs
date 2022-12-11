use std::env;
use std::path::PathBuf;

use symlink::{remove_symlink_file, symlink_file};

const ALIASES: [&str;5] = ["mvn", "mvnw", "gradle", "gradlew", "javabox"];

/// Prepare javabox tools for convenient use.
/// This includes:
/// - TODO make myself (arg#0) executable if it is not (linux, mac)
/// - TODO put myself into a directory on PATH
///     a) use an existing accessible directory which alraedy is on PATH, and move itself there
///     b) select a good directory and convince user to put it on PATH
///         - _linux_: help him by adding stuff to .bashrc
///         - _mac_: ?
///         - _windows_: ?
/// - (re)configure symlinks/shortcuts/scripts for running each tool
pub fn javabox_install(javabox_bin_dir: PathBuf, force_overwrite: bool) -> std::io::Result<()>{
    let javabox_exe = env::current_exe()?;
    log::info!("Creating symlinks for {}", javabox_exe.display());

    #[cfg(target_os = "linux")]
    {
        let mut perms = std::fs::metadata(&javabox_exe)?.permissions();
        use std::os::unix::fs::PermissionsExt;
        perms.set_mode(0x755);
    }

    log::info!("Creating symlinks for {}", javabox_exe.display());

    for alias in ALIASES {
        let symlink = javabox_bin_dir.join(alias);
        log::debug!("* {}", symlink.display());
        if symlink.exists() {
            if symlink == javabox_exe {
                log::debug!("Not replacing myself with symlink: {}", symlink.display());
                continue;
            }
            if !force_overwrite {
                log::warn!("File already exists, use '--force' to overwrite: {}", symlink.display());
                continue;
            }
            log::warn!("File already exists, overwriting: {}", symlink.display());
            std::fs::remove_file(&symlink)?;
        }
        symlink_file(&javabox_exe, &symlink)?;
    }
    Ok(())
}

/// Remove any changes performed by [javabox_install]; only javabox itself remains
pub fn javabox_uninstall(javabox_bin_dir: PathBuf) -> std::io::Result<()>{
    log::info!("Removing symlinks from {}", javabox_bin_dir.display());
    let javabox_exe = env::current_exe()?;
    for alias in ALIASES {
        let symlink = javabox_bin_dir.join(alias);
        log::debug!("* {}", symlink.display());
        if !symlink.exists() {
            log::warn!("File does not exist: {}", symlink.display());
            continue;
        }
        if symlink == javabox_exe {
            log::debug!("Not removing myself: {}", symlink.display());
            continue;
        }
        if !symlink.is_symlink() {
            log::warn!("Not a symlink: {}", symlink.display());
            continue;
        }
        let symlink_target = std::fs::read_link(&symlink)?;
        if symlink_target != javabox_exe {
            log::warn!("Not my symlink, skipping: {} - points to {}", symlink.display(), symlink_target.display());
            continue;
        }
        remove_symlink_file(symlink)?;
    }
    Ok(())
}
