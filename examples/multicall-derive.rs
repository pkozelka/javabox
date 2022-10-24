use std::process::exit;

use clap::{Parser, Subcommand};

/// An attempt to use multicall with derive
#[derive(Parser)] // requires `derive` feature
// #[command(multicall = true)] // this does not work
#[command(name = "multicall-derive", about = "Multi-call derive example", long_about = None)]
struct Cli {
    #[command(subcommand)]
    applet: Applets,
}

#[derive(Subcommand)]
enum Applets {
    #[command(name="multicall-derive")]
    MulticallDerive,
    #[command(name="true")]
    AppletTrue,
    #[command(name="false")]
    AppletFalse,
}

fn main() {
    let args: Cli = Cli::parse();
    match args.applet {
        Applets::AppletTrue => {
            println!("running true");
            exit(0);
        }
        Applets::AppletFalse => {
            println!("running false");
            exit(1);
        }
        Applets::MulticallDerive => {
            println!("Hello");
        }
    }
}
