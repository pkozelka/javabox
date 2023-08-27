use std::env;
use log::LevelFilter;


fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .format_timestamp_millis()
        .filter_level(LevelFilter::Trace)
        .filter_module("serde_xml_rs", LevelFilter::Info)
        .filter_module("rustls", LevelFilter::Info)
        .filter_module("zip_extract", LevelFilter::Debug)
        .init();

    let exe = env::args().next().unwrap();
    // println!("executable full name is '{exe}'");
    let exe = match exe.rfind(std::path::MAIN_SEPARATOR) {
        None => exe.as_str(),
        Some(n) => &exe[n+1..]
    };
    // println!("executable name is '{exe}'");
    let exit_code = match exe {
        "mvnw" |
        "mvn" => mvn::run_mvn()?,
        "gradlew" |
        "gradle" => gradle::run_gradle()?,
        "javabox.exe" |
        "javabox" => javabox::run_javabox()?,
        _ => panic!("Unsupported alias name: {exe}")
    };
    if exit_code != 0 {
        log::error!("Returning with exit code {exit_code}");
        std::process::exit(exit_code);
    }
    Ok(())
}

mod mvn;
mod gradle;
mod javabox;
mod utils;
mod java_hash;

mod config;
