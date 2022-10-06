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

/// ```shell
/// hash_string() {
///   str="${1:-}" h=0
///   while [ -n "$str" ]; do
///     h=$(( ( h * 31 + $(LC_CTYPE=C printf %d "'$str") ) % 4294967296 ))
///     str="${str#?}"
///   done
///   printf %x\\n $h
/// }
/// ```
/// hash string like Java String::hashCode
fn java_string_hash(text: &str) -> i32 {
    let mut h: i32 = 0;
    for &ch in text.as_bytes() {
        h = h.wrapping_mul(31).wrapping_add(ch as i32);
    }
    h
}

fn java_uri_hash(uri: url::Url) -> i32 {
    eprintln!("uri : {uri:?}");
    let opaque = match uri.scheme() {
        "mailto" | "news" | "urn" => true,
        _ => false
    };
    let h = hash_ignoring_case(0, uri.scheme());
    let h = match uri.fragment() {
        None => h,
        Some(fragment) => hash(h, fragment)
    };
    if opaque {
        let mut scheme_specific_part = uri.path().to_string();
        if let Some(query) = uri.query() {
            scheme_specific_part.push_str(query);
        }
        hash(h, &scheme_specific_part);
        //TODO a lot of hideous magic belongs here
        todo!()
    } else {
        let mut h = hash(h, uri.path());
        if let Some(query) = uri.query() {
            h = hash(h, query);
        }
        match uri.host() {
            Some(host) => {
                //TODO hash userinfo
                h = hash_ignoring_case(h, &host.to_string());
                let port = match uri.port() {
                    None => -1,
                    Some(port) => port as i32
                };
                h = h.wrapping_add(port.wrapping_mul(1949))
            }
            None => {
                //TODO hash authority
                todo!()
            },
        }
        h
    }
}

fn hash_ignoring_case(hash:i32, s: &str) -> i32 {
    let mut h = hash;
    for c in s.chars() {
        let ch = c.to_ascii_lowercase();
        h = h.wrapping_mul(31).wrapping_add(ch as i32);
    }
    h
}

fn normalized_hash(hash:i32, s: &str) -> i32 {
    let mut h: i32 = 0;
    let mut up = 0;
    for ch in s.chars() {
        let ch = if up > 0 {
            up -= 1;
            ch.to_ascii_uppercase()
        } else {
            ch
        };
        h = h.wrapping_mul(31).wrapping_add(ch as i32);
        if ch == '%' {
            up = 2;
        }
    }
    hash * 127 + h
}

fn hash(hash:i32, s: &str) -> i32 {
    if s.contains('%') {
        normalized_hash(hash, s)
    } else {
        hash.wrapping_mul(127).wrapping_add(java_string_hash(s))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use url::Url;
    use crate::mvn::{hash_ignoring_case, java_string_hash, java_uri_hash};

    #[test]
    fn test_mvn_hasher() {
        let url = Url::from_str("https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.8.6/apache-maven-3.8.6-bin.zip").unwrap();
        let h = java_uri_hash(url);
        println!("{}", h);
        println!("{:x}", h);
        assert_eq!(1733723188, h);
    }

    #[test]
    fn test_parts() {
        assert_eq!(99617003, hash_ignoring_case(0, "https"));
        assert_eq!(-1920266189, java_string_hash("seznam.cz"));
        assert_eq!(47, java_string_hash("/"));
        assert_eq!(0, java_string_hash(""));
        assert_eq!(-851661731, java_string_hash("/maven2/org/apache/maven/apache-maven/3.8.6/apache-maven-3.8.6-bin.zip"));
    }

    #[test]
    fn test_seznam_hasher() {
        assert_eq!(-1242908590, java_uri_hash(Url::from_str("https://seznam.cz/").unwrap()));
        assert_eq!(-150014690, java_uri_hash(Url::from_str("https://seznam.cz/Hello").unwrap()));
        // assert_eq!(-596708447, java_uri_hash(Url::from_str("https://seznam.cz").unwrap())); //bug: url::Url cannot represent empty path
    }
}