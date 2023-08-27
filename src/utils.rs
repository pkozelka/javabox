use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crypto::digest::Digest;
use crypto::md5::Md5;
use indicatif::{ProgressBar, ProgressStyle};
use ureq::Response;
use url::Url;

/// Runs the specified tool from project directory with working directory changed to specified module
pub fn execute_tool(project: &Path, tool: &str, module: &Path) -> std::io::Result<i32> {
    log::info!("Running {tool} for project {} in module {}", project.display(), module.display());
    let mut command = std::process::Command::new(project.join(tool));
    command.current_dir(module);
    command.args(std::env::args().skip(1));
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    let status = command.status()?;
    match status.code() {
        None => Err(std::io::Error::new(ErrorKind::BrokenPipe, "Interrupted")),
        Some(code) => Ok(code)
    }
}

/// Read property file into a HashMap.
/// Empty lines and comments are ignored.
/// Lines in form `KEY=VALUE` are read.
/// Other lines are reported to stderr.
pub fn read_properties(properties: &mut HashMap<String,String>, path: &Path) -> std::io::Result<()> {
    log::trace!("read_properties({})", path.display());
    let text = std::fs::read_to_string(path)?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match line.find('=') {
            None => {
                log::warn!("ERROR: Bad line: {line}");
            }
            Some(n) => {
                let key = &line[0..n];
                let value = &line[n + 1..].replace("\\:", ":");
                properties.insert(key.to_string(), value.to_string());
            }
        }
    }
    Ok(())
}

/// ureq doesn't have this
fn get_content_length(response: &Response) -> Option<u64> {
    match response.header("content-length") {
        None => None,
        Some(sz) => sz.parse::<u64>().ok()
    }
}

/// Downloads a file from given URL.
/// It is safe - the new file exists only if it was read successfully; download is pointed to a different file.
pub fn download(url: &Url, path: &Path) -> std::io::Result<()> {
    log::info!("Downloading {} from {}", path.display(), url.as_str());
    let request = ureq::get(url.as_str());
    let response = request.call()
        .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("Problem with request: {url} :: {e:?}")))?;
    if response.status() != 200 {
        return Err(Error::new(ErrorKind::Other, format!("HTTP Status {}:{} on {}", response.status(), response.status_text(), response.get_url())));
    }
    let total_size = get_content_length(&response).ok_or(std::io::Error::new(ErrorKind::Other, "Cannot get content length"))?;

    let mut tmp_path = path.display().to_string();
    if path.exists() {
        tmp_path.push_str(".upd");
        // for existing file, we use its current mtime in the tmp suffix
        let stat = std::fs::metadata(path)?;
        let time = stat.modified()
            .or(stat.created())
            .unwrap_or(SystemTime::now());
        let time = time.duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs();
        if time > 0 {
            tmp_path.push_str(&format!(".{time}"));
        }
    } else {
        tmp_path.push_str(".new");
    }
    let tmp_path = PathBuf::from(&tmp_path);
    std::fs::create_dir_all(tmp_path.parent().unwrap())?;
    let mut br = BufReader::new(response.into_reader());
    let mut buf = [0; 8192];
    let mut wr = File::options()
        .create(true)
        .write(true)
        .open(&tmp_path)?;
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n[{wide_bar:.cyan/blue}] {bytes}/{total_bytes} [{elapsed_precise}] ({bytes_per_sec}, {eta})")
        .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("ERROR: {e:?}")))?
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", path.file_name().unwrap().to_str().unwrap()));
    let mut downloaded = 0;
    loop {
        let sz = br.read(&mut buf)?;
        downloaded += sz;
        pb.set_position(downloaded as u64);
        if sz == 0 {
            wr.flush()?;
            break;
        }
        wr.write(&buf[0..sz])?;
    }
    pb.finish_and_clear();
    // TODO: verify checksum
    // give it the proper name
    std::fs::rename(tmp_path, path)?;
    Ok(())
}

/// Downloads a file only if it is older than provided age.
/// The file is reused otherwise, and also if troubles occur during age check or download.
/// Useful only when being totally up-to-date is not critical.
pub fn download_or_reuse(url: &Url, path: &Path, max_age: Duration) -> std::io::Result<()> {
    match std::fs::metadata(path) {
        Ok(stat) => {
            let mut needs_update = true;
            match stat.modified().or(stat.created()) {
                Ok(time) => {
                    let max_age_secs = max_age.as_secs();
                    // if too fresh, avoid updating
                    let age_secs = time.elapsed()
                        .unwrap_or(Duration::from_secs(max_age_secs + 1))
                        .as_secs();
                    needs_update = age_secs > max_age_secs;
                }
                Err(e) => {
                    log::warn!("Cannot read modification time of '{}', file will be updated. Error is: {e:?}", path.display());
                }
            }
            if needs_update {
                // try to update, but don't die if you can't
                if let Err(e) = download(&url, &path) {
                    log::warn!("Failed to update file '{}', let's assume that the latest version didn't change. Error is: {e:?}", path.display());
                }
            }
            Ok(())
        }
        Err(e) => {
            if path.exists() {
                log::warn!("Cannot read file information: '{}'. Error is: {e:?}", path.display())
            }
            download(&url, &path)
        }
    }
}

/// Decides if provided directory is the root of SCM working copy, by examining the presence of metadata.
/// Returns:
/// - true if it _surely is_
/// - `false` if it _surely is not_
/// There is currently no way to indicate 'not sure'.
pub fn is_scm_wc_root(d: &Path) -> bool {
    d.join(".git/config").is_file()
        || d.join(".svn").is_dir()
        || d.join(".hg").is_dir()
}

/// This is used in Gradle Wrapper for distributionUrl hashing.
/// See https://github.com/gradle/gradle/blob/master/subprojects/wrapper-shared/src/main/java/org/gradle/wrapper/PathAssembler.java#L62-L71
///
pub fn md5decimal(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input(s.as_bytes());
    let mut output = [0; 16]; // An MD5 is 16 bytes
    hasher.result(&mut output);
    let dec = bigdecimal::num_bigint::BigUint::from_bytes_be(&output);
    dec.to_str_radix(36)
}

#[cfg(test)]
mod tests {
    use crate::utils::md5decimal;

    #[test]
    fn test_md5radix36() {
        assert_eq!("260hg96vuh6ex27h9vo47iv4d", md5decimal("https://services.gradle.org/distributions/gradle-7.2-all.zip"))
    }
}
