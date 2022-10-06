use std::env::current_dir;

use crate::utils;

pub fn run_gradle() -> std::io::Result<i32> {
    //TODO lookup top of the repository (.git/, .svn/, .gitignore, .svnignore etc)
    // lookup nearest build.gradle
    // lookup settings.gradle, gradle.properties, topmost build.gradle,
    // determine gradle version and java version
    // (download and) select desired gradle version
    // (download and) select desired java version
    // run gradle in nearest directory
    let current = current_dir()?;
    let module_dir = utils::find_containing_dir(&current, "build.gradle");
    let module_dir = module_dir.expect("No gradle module found above current directory");
    let project_dir = utils::find_containing_dir(&module_dir, "gradlew");
    let project_dir = project_dir.expect("No gradle project found above module directory");
    utils::execute_tool(&project_dir, "gradlew", &module_dir)
}