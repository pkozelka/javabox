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
pub fn javabox_install(bin: Option<PathBuf>, force_overwrite: bool) -> std::io::Result<()>{

    let bin = match bin {
        None => dir::home_dir().unwrap().join("bin"), // TODO probably not very good
        Some(bin) => bin
    };
    let javabox = env::current_exe().unwrap();
    log::info!("Creating symlinks for {}", javabox.display());
    for alias in ALIASES {
        let symlink = bin.join(alias);
        log::debug!("* {}", symlink.display());
        if symlink.exists() {
            if !force_overwrite {
                log::warn!("File already exists, use '--force' to overwrite: {}", symlink.display());
                continue;
            }
            log::warn!("File already exists, overwriting: {}", symlink.display());
            std::fs::remove_file(&symlink)?;
        }
        symlink_file(&javabox, &symlink)?;
    }
    Ok(())
}

/// Remove any changes performed by [javabox_install]; only javabox itself remains
pub fn javabox_uninstall(bin: Option<PathBuf>) -> std::io::Result<()>{

    let bin = match bin {
        None => dir::home_dir().unwrap().join("bin"), // TODO probably not very good
        Some(bin) => bin
    };
    let javabox = env::current_exe().unwrap();
    log::info!("Removing symlinks for {}", javabox.display());
    for alias in ALIASES {
        let symlink = bin.join(alias);
        log::debug!("* {}", symlink.display());
        if !symlink.exists() {
            log::warn!("File does not exist: {}", symlink.display());
            continue;
        }
        if !symlink.is_symlink() {
            log::warn!("Not a symlink: {}", symlink.display());
            continue;
        }
        let symlink_target = std::fs::read_link(&symlink)?;
        if symlink_target != javabox {
            log::warn!("Not my symlink, skipping: {} - points to {}", symlink.display(), symlink_target.display());
            continue;
        }
        remove_symlink_file(symlink)?;
    }
    Ok(())
}
