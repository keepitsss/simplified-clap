use std::ffi::OsStr;

// Despite our design philosophy being to support completion generation, we aren't considering `-`
// the start of a long because there is no valid value to return.
#[test]
fn to_long_stdio() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-"]);
    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(!next.is_long());

    assert_eq!(next.to_long(), None);
}

#[test]
fn to_long_no_escape() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "--"]);
    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(!next.is_long());

    assert_eq!(next.to_long(), None);
}

#[test]
fn to_long_no_value() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "--long"]);
    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok("long"));
    assert_eq!(value, None);
}

#[test]
fn to_long_with_empty_value() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "--long="]);
    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok("long"));
    assert_eq!(value, Some(OsStr::new("")));
}

#[test]
fn to_long_with_value() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "--long=hello"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok("long"));
    assert_eq!(value, Some(OsStr::new("hello")));
}

#[test]
fn to_short_stdio() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(!next.is_short());

    assert!(next.to_short().is_none());
}

#[test]
fn to_short_escape() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "--"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(!next.is_short());

    assert!(next.to_short().is_none());
}

#[test]
fn to_short_long() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "--long"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(!next.is_short());

    assert!(next.to_short().is_none());
}

#[test]
fn to_short() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-short"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(next.is_short());

    let shorts = next.to_short().unwrap();
    let actual: String = shorts.map(|s| s.unwrap()).collect();
    assert_eq!(actual, "short");
}

#[test]
fn is_negative_number() {
    for number in ["-10.0", "-1", "-100", "-3.5", "-1e10", "-1.3e10", "-1E10"] {
        let mut raw = clap_lex::RawArgs::leaking_from(["bin", number]);

        assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
        let next = raw.next_arg().unwrap();

        assert!(next.is_negative_number());
    }
}

#[test]
fn is_positive_number() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "10.0"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(!next.is_negative_number());
}

#[test]
fn is_not_number() {
    for number in [
        "--10.0", "-..", "-2..", "-e", "-1e", "-1e10.2", "-.2", "-E", "-1E", "-1E10.2",
    ] {
        let mut raw = clap_lex::RawArgs::leaking_from(["bin", number]);

        assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
        let next = raw.next_arg().unwrap();

        assert!(
            !next.is_negative_number(),
            "`{number}` is mistakenly classified as a number"
        );
    }
}

#[test]
fn is_stdio() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(next.is_stdio());
}

#[test]
fn is_not_stdio() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "--"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(!next.is_stdio());
}

#[test]
fn is_escape() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "--"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(next.is_escape());
}

#[test]
fn is_not_escape() {
    let mut raw = clap_lex::RawArgs::leaking_from(["bin", "-"]);

    assert_eq!(raw.next_os_str(), Some(OsStr::new("bin")));
    let next = raw.next_arg().unwrap();

    assert!(!next.is_escape());
}
