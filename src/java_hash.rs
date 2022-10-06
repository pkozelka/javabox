/// hash string like Java String::hashCode
pub fn java_string_hash(text: &str) -> i32 {
    let mut h: i32 = 0;
    for &ch in text.as_bytes() {
        h = h.wrapping_mul(31).wrapping_add(ch as i32);
    }
    h
}

pub fn java_uri_hash(uri: &url::Url) -> i32 {
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
    use super::{hash_ignoring_case, java_string_hash, java_uri_hash};

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