use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None,bin_name="javabox")]
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
            cmd_setup::javabox_install(javabox_home(bin), force)?;
        }
        Commands::Uninstall { bin } => {
            cmd_setup::javabox_uninstall(javabox_home(bin))?;
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

fn javabox_home(javabox_home: Option<PathBuf>) -> PathBuf {
    let javabox_home = match javabox_home {
        None => {
            let user_home = dir::home_dir().unwrap();
            user_home.join("bin")
        }, // TODO probably not very good
        Some(javabox_home) => javabox_home
    };
    if !javabox_home.exists() {
        log::debug!("creating javabox home directory: {}", javabox_home.display());
        let _ = std::fs::create_dir_all(&javabox_home);
    }
    javabox_home
}

mod cmd_setup;
