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
    },
    /// remove symlinks to javabox
    Uninstall {
        /// where to create the symlinks - if other than `~/bin/`
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

pub fn run_javabox() -> std::io::Result<()>{
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::Install { bin } => {
            javabox_install(bin)?;
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
    Ok(())
}

const ALIASES: [&str;4] = ["mvn", "mvnw", "gradle", "gradlew"];

fn javabox_install(bin: Option<PathBuf>) -> std::io::Result<()>{

    let bin = match bin {
        None => dir::home_dir().unwrap().join("bin"), // TODO probably not very good
        Some(bin) => bin
    };
    let javabox = env::current_exe().unwrap();
    eprintln!("Creating symlinks for {}", javabox.display());
    for alias in ALIASES {
        let symlink = bin.join(alias);
        eprintln!("* {}", symlink.display());
        if symlink.exists() {
            eprintln!("WARNING: ^ File already exists!");
            continue;
        }
        symlink_file(&javabox, symlink)?;
    }
    Ok(())
}

fn javabox_uninstall(bin: Option<PathBuf>) -> std::io::Result<()>{

    let bin = match bin {
        None => dir::home_dir().unwrap().join("bin"), // TODO probably not very good
        Some(bin) => bin
    };
    let javabox = env::current_exe().unwrap();
    eprintln!("Removing symlinks for {}", javabox.display());
    for alias in ALIASES {
        let symlink = bin.join(alias);
        eprintln!("* {}", symlink.display());
        if !symlink.exists() {
            eprintln!("WARNING: ^ File does not exist!");
            continue;
        }
        // TODO: somehow, check if it is "my" link, skip otherwise
        remove_symlink_file(symlink)?;
    }
    Ok(())
}
