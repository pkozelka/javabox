use std::collections::HashMap;
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

/// Read property file into a HashMap.
/// Empty lines and comments are ignored.
/// Lines in form `KEY=VALUE` are read.
/// Other lines are reported to stderr.
pub fn read_properties(path: &Path) -> std::io::Result<HashMap<String, String>> {
    let text = std::fs::read_to_string(path)?;
    let mut properties = HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue
        }
        match line.find('=') {
            None => {
                eprintln!("ERROR: Bad line: {line}");
            }
            Some(n) => {
                let key = &line[0..n];
                let value = &line[n+1..];
                properties.insert(key.to_string(), value.to_string());
            }
        }
    }
    Ok(properties)
}