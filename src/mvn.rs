//! # Maven Wrapper
//!
use std::env::current_dir;

use crate::utils;

pub fn run_mvn() -> std::io::Result<()> {
    let current = current_dir()?;
    // all ancestors containing pom.xml
    let mut modules = Vec::new();
    // top of the SCM repository
    let mut scm_repo_root = None;
    let mut wrapper = None;
    for d in current.ancestors() {
        if scm_repo_root.is_none() {
            // we only care about these files _within_ scm repo, if one exists
            // ... and also _within_ wrapper, if one exists
            if wrapper.is_none() && d.join("pom.xml").is_file() {
                modules.push(d);
                eprintln!("POM: {}", d.display());
            }
            if d.join("mvnw").is_file()
                || d.join("mvnw.bat").is_file()
                || d.join(".mvn").is_dir()
            {
                wrapper = Some(d);
                eprintln!("WRAPPER: {}", d.display());
            }
            //
        }
        if d.join(".git/config").is_file()
            || d.join(".svn").is_dir()
        {
            scm_repo_root = Some(d);
            eprintln!("SCM WORKING COPY: {}", d.display());
        }

        //TODO: think about seeking
        // - .repository/ and .m2/repository/
        // - settings.xml and .m2/settings.xml
        // - .m2/
    }
    let project_dir = *modules.last().expect("Not inside a maven module with pom.xml file found");
    let module_dir = modules[0];

    // TODO: consider delegating to the existing wrapper, if it isn't myself
    // TODO: when on a nested module, we should perhaps go from project root and add options `-pl` and `--also-make` or `--also-make-dependents`
    // TODO: estimate maven version and use it; default=latest
    // TODO: estimate JDK version and use it
    utils::execute_tool(&project_dir, "/home/pk/opt/maven/bin/mvn", &module_dir)
}
