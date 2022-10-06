use std::env;
use log::LevelFilter;


fn main() {
    pretty_env_logger::formatted_timed_builder()
        .format_timestamp_millis()
        .filter_level(LevelFilter::Trace)
        .filter_module("serde_xml_rs", LevelFilter::Info)
        .filter_module("rustls", LevelFilter::Info)
        .filter_module("zip_extract", LevelFilter::Debug)
        .init();

    let exe = env::args().next().unwrap();
    let exe = match exe.rfind(std::path::MAIN_SEPARATOR) {
        None => exe.as_str(),
        Some(n) => &exe[n+1..]
    };
    // println!("executable name is '{exe}'");
    match exe {
        "mvnw" |
        "mvn" => mvn::run_mvn(),
        "gradlew" |
        "gradle" => gradle::run_gradle(),
        "javabox" => javabox::run_javabox(),
        _ => panic!("Unsupported alias name: {exe}")
    }.unwrap()
}

mod mvn;
mod gradle;
mod javabox;
mod utils;
mod java_hash;