use std::env::current_dir;

use crate::utils;

pub fn run_mvn() -> std::io::Result<()> {
    //TODO: improve heuristic, heavily
    let current = current_dir()?;
    let module_dir = utils::find_containing_dir(&current, "pom.xml");
    let module_dir = module_dir.expect("No maven module found above current directory");
    let project_dir = utils::find_containing_dir(&module_dir, "mvnw");
    let project_dir = project_dir.expect("No maven project found above module directory");
    utils::execute_tool(&project_dir, "mvnw", &module_dir)
}
