use std::env;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use symlink::{remove_symlink_file, symlink_file};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create symlinks to javabox into a directory present in PATH
    Install {
        /// where to create the symlinks - if other than `~/bin/`
        bin: Option<PathBuf>,
        /// overwrite existing symlinks
        #[arg(short, long)]
        force: bool,
    },
    /// remove symlinks to javabox
    Uninstall {
        /// where to remove the symlinks - if other than `~/bin/`
        bin: Option<PathBuf>,
    },
    /// download java, maven, gradle etc of given version
    Download {
        #[arg(value_name = "TOOL")]
        tool: String,
        #[arg(value_name = "VERSION")]
        version: String,
        /// ensure that the downloaded version is the default for given tool
        #[arg(short, long)]
        select: bool
    },
    /// select default versions for tools
    Select {
        #[arg(value_name = "TOOL")]
        tool: String,
        /// for which major version is this the default (if relevant)
        #[arg(short, long)]
        major: String,
        #[arg(value_name = "VERSION")]
        version: String,
    },
}

pub fn run_javabox() -> std::io::Result<i32>{
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::Install { bin, force } => {
            javabox_install(bin, force)?;
        }
        Commands::Uninstall { bin } => {
            javabox_uninstall(bin)?;
        }
        Commands::Download {..} => {
            todo!("download")
        }
        Commands::Select {..} => {
            todo!("select")
        }
    }
    Ok(0)
}

const ALIASES: [&str;5] = ["mvn", "mvnw", "gradle", "gradlew", "javabox"];

fn javabox_install(bin: Option<PathBuf>, force_overwrite: bool) -> std::io::Result<()>{

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

fn javabox_uninstall(bin: Option<PathBuf>) -> std::io::Result<()>{

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
