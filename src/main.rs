use std::env;


fn main() {
    // Continued program logic goes here...
    let exe = env::args().next().unwrap();
    // println!("first arg is '{exe}'");
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
        "javabox" => javabox::run_javabox().unwrap(),
        _ => panic!("Unsupported alias name: {exe}")
    }
}

mod mvn;
mod gradle;
mod javabox;
