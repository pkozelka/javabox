use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

pub fn run_javabox() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Download {..} => {
            todo!("download")
        }
        Commands::Select {..} => {
            todo!("select")
        }
    }
}

