use std::io::ErrorKind;
use std::path::Path;
use std::process::Stdio;

/// Seeks file `what` in current directory and all its parents
pub fn find_containing_dir<'a>(dir: &'a Path, what: &str) -> Option<&'a Path> {
    let mut dir = dir;
    while !dir.join(what).exists() {
        dir = match dir.parent() {
            None => { return None; }
            Some(dir) => dir
        };
    }
    Some(dir)
}

/// Runs the specified tool from project directory with working directory changed to specified module
pub fn execute_tool(project: &Path, tool: &str, module: &Path) -> std::io::Result<()> {
    println!("Running {tool} for project {} in module {}", project.display(), module.display());
    let mut command = std::process::Command::new(project.join(tool));
    command.current_dir(module);
    command.args(std::env::args().skip(1));
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    let status = command.status()?;
    match status.code() {
        None => Ok(()),
        Some(0) => Ok(()),
        Some(code) => {
            eprintln!("Process returned code {code}");
            Err(std::io::Error::new(ErrorKind::Other, format!("error: {code}")))
        }
    }
}
