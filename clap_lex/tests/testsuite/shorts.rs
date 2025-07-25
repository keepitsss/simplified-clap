#[test]
fn iter() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-short"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let shorts = next.to_short().unwrap();

    let actual: String = shorts.map(|s| s.unwrap()).collect();
    assert_eq!(actual, "short");
}

#[test]
fn next_flag() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-short"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let mut shorts = next.to_short().unwrap();

    let mut actual = String::new();
    actual.push(shorts.next_flag().unwrap().unwrap());
    actual.push(shorts.next_flag().unwrap().unwrap());
    actual.push(shorts.next_flag().unwrap().unwrap());
    actual.push(shorts.next_flag().unwrap().unwrap());
    actual.push(shorts.next_flag().unwrap().unwrap());
    assert_eq!(shorts.next_flag(), None);

    assert_eq!(actual, "short");
}

#[test]
fn next_value_os() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-short"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let mut shorts = next.to_short().unwrap();

    let actual = shorts.next_value_os().unwrap().to_string_lossy();

    assert_eq!(actual, "short");
}

#[test]
fn next_flag_with_value() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-short"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let mut shorts = next.to_short().unwrap();

    assert_eq!(shorts.next_flag().unwrap().unwrap(), 's');
    let actual = shorts.next_value_os().unwrap().to_string_lossy();

    assert_eq!(actual, "hort");
}

#[test]
fn next_flag_with_no_value() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-short"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let mut shorts = next.to_short().unwrap();

    assert_eq!(shorts.next_flag().unwrap().unwrap(), 's');
    assert_eq!(shorts.next_flag().unwrap().unwrap(), 'h');
    assert_eq!(shorts.next_flag().unwrap().unwrap(), 'o');
    assert_eq!(shorts.next_flag().unwrap().unwrap(), 'r');
    assert_eq!(shorts.next_flag().unwrap().unwrap(), 't');

    assert_eq!(shorts.next_value_os(), None);
}

#[test]
fn advance_by_nothing() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-short"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let mut shorts = next.to_short().unwrap();

    assert_eq!(shorts.advance_by(0), Ok(()));

    let actual: String = shorts.map(|s| s.unwrap()).collect();
    assert_eq!(actual, "short");
}

#[test]
fn advance_by_something() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-short"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let mut shorts = next.to_short().unwrap();

    assert_eq!(shorts.advance_by(2), Ok(()));

    let actual: String = shorts.map(|s| s.unwrap()).collect();
    assert_eq!(actual, "ort");
}

#[test]
fn advance_by_out_of_bounds() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-short"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let mut shorts = next.to_short().unwrap();

    assert_eq!(shorts.advance_by(2000), Err(5));

    let actual: String = shorts.map(|s| s.unwrap()).collect();
    assert_eq!(actual, "");
}

#[test]
fn is_not_empty() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-hello"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let shorts = next.to_short().unwrap();

    assert!(!shorts.is_empty());
}

#[test]
fn is_partial_not_empty() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-hello"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let mut shorts = next.to_short().unwrap();
    shorts.advance_by(1).unwrap();

    assert!(!shorts.is_empty());
}

#[test]
fn is_exhausted_empty() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-hello"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let mut shorts = next.to_short().unwrap();
    shorts.advance_by(20000).unwrap_err();

    assert!(shorts.is_empty());
}

#[test]
fn is_negative_number() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-1.0"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let shorts = next.to_short().unwrap();

    assert!(shorts.is_negative_number());
}

#[test]
fn is_not_negaitve_number() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-hello"]);

    assert_eq!(raw.next_os_str(), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next_arg().unwrap();
    let shorts = next.to_short().unwrap();

    assert!(!shorts.is_negative_number());
}
