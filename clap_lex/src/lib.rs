//! Minimal, flexible command-line parser
//!
//! As opposed to a declarative parser, this processes arguments as a stream of tokens.  As lexing
//! a command-line is not context-free, we rely on the caller to decide how to interpret the
//! arguments.
//!
//! # Examples
//!
//! ```rust
//! use std::path::PathBuf;
//! use std::ffi::OsStr;
//!
//! type BoxedError = Box<dyn std::error::Error + Send + Sync>;
//!
//! #[derive(Debug)]
//! struct Args {
//!     paths: Vec<PathBuf>,
//!     color: Color,
//!     verbosity: usize,
//! }
//!
//! #[derive(Debug)]
//! enum Color {
//!     Always,
//!     Auto,
//!     Never,
//! }
//!
//! impl Color {
//!     fn parse(s: Option<&OsStr>) -> Result<Self, BoxedError> {
//!         let s = s.map(|s| s.to_str().ok_or(s));
//!         match s {
//!             Some(Ok("always")) | Some(Ok("")) | None => {
//!                 Ok(Color::Always)
//!             }
//!             Some(Ok("auto")) => {
//!                 Ok(Color::Auto)
//!             }
//!             Some(Ok("never")) => {
//!                 Ok(Color::Never)
//!             }
//!             Some(invalid) => {
//!                 Err(format!("Invalid value for `--color`, {invalid:?}").into())
//!             }
//!         }
//!     }
//! }
//!
//! fn parse_args(
//!     raw: impl IntoIterator<Item=impl Into<std::ffi::OsString>>
//! ) -> Result<Args, BoxedError> {
//!     let mut args = Args {
//!         paths: Vec::new(),
//!         color: Color::Auto,
//!         verbosity: 0,
//!     };
//!
//!     let raw_args = raw.into_iter().map(|x| x.into()).collect::<Vec<_>>();
//!     let raw = raw_args.iter().map(|x| x.as_os_str()).collect::<RawArgs>();
//!     let mut cursor = 0;
//!     raw.next(&mut cursor);  // Skip the bin
//!     while let Some(arg) = raw.next(&mut cursor) {
//!         if arg.is_escape() {
//!             args.paths.extend(raw.remaining(&mut cursor).map(PathBuf::from));
//!         } else if arg.is_stdio() {
//!             args.paths.push(PathBuf::from("-"));
//!         } else if let Some((long, value)) = arg.to_long() {
//!             match long {
//!                 Ok("verbose") => {
//!                     if let Some(value) = value {
//!                         return Err(format!("`--verbose` does not take a value, got `{value:?}`").into());
//!                     }
//!                     args.verbosity += 1;
//!                 }
//!                 Ok("color") => {
//!                     args.color = Color::parse(value)?;
//!                 }
//!                 _ => {
//!                     return Err(
//!                         format!("Unexpected flag: --{}", arg.display()).into()
//!                     );
//!                 }
//!             }
//!         } else if let Some(mut shorts) = arg.to_short() {
//!             while let Some(short) = shorts.next_flag() {
//!                 match short {
//!                     Ok('v') => {
//!                         args.verbosity += 1;
//!                     }
//!                     Ok('c') => {
//!                         let value = shorts.next_value_os();
//!                         args.color = Color::parse(value)?;
//!                     }
//!                     Ok(c) => {
//!                         return Err(format!("Unexpected flag: -{c}").into());
//!                     }
//!                     Err(e) => {
//!                         return Err(format!("Unexpected flag: -{}", e.to_string_lossy()).into());
//!                     }
//!                 }
//!             }
//!         } else {
//!             args.paths.push(PathBuf::from(arg.to_value_os().to_owned()));
//!         }
//!     }
//!
//!     Ok(args)
//! }
//!
//! let args = parse_args(["bin", "--hello", "world"]);
//! println!("{args:?}");
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod ext;

use std::ffi::{OsStr, OsString};
pub use std::io::SeekFrom;

pub use ext::OsStrExt;

/// Command-line arguments wrapper
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct RawArgs<'a> {
    /// some args, mb not all
    pub items: Vec<&'a OsStr>,
    /// points to current item
    pub cursor: usize,
}

impl<'a> RawArgs<'a> {
    /// Advance the cursor, returning the next [`ParsedArg`]
    pub fn next_arg(&mut self) -> Option<ParsedArg<'a>> {
        self.next_os_str().map(ParsedArg::new)
    }

    /// Advance the cursor, returning a raw argument value.
    pub fn next_os_str(&mut self) -> Option<&'a OsStr> {
        let next = self.items.get_mut(self.cursor).copied();
        self.cursor = self.cursor.saturating_add(1);
        next
    }

    /// Return the next [`ParsedArg`]
    pub fn peek_arg(&self) -> Option<ParsedArg<'_>> {
        self.peek_os_str().map(ParsedArg::new)
    }

    /// Return a raw argument value.
    pub fn peek_os_str(&self) -> Option<&OsStr> {
        self.items.get(self.cursor).copied()
    }

    /// Return all remaining raw arguments, advancing the cursor to the end
    pub fn remaining(self) -> Vec<OsString> {
        self.items[self.cursor..]
            .iter()
            .map(|x| x.to_os_string())
            .collect()
    }
}

impl<'a> RawArgs<'a> {
    #[doc(hidden)]
    pub fn leaking_from(itr: impl IntoIterator<Item = impl Into<OsString>>) -> Self {
        let args = itr.into_iter().map(|x| x.into()).collect::<Vec<OsString>>();
        let args = Box::leak(args.into_boxed_slice());
        let items = args.iter().map(|x| x.as_os_str()).collect::<Vec<_>>();
        Self { items, cursor: 0 }
    }
}

impl<'a> FromIterator<&'a OsStr> for RawArgs<'a> {
    fn from_iter<T: IntoIterator<Item = &'a OsStr>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
            cursor: 0,
        }
    }
}

/// Command-line Argument
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedArg<'s> {
    inner: &'s OsStr,
}

impl<'s> ParsedArg<'s> {
    fn new(inner: &'s OsStr) -> Self {
        Self { inner }
    }

    /// Argument is length of 0
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Does the argument look like a stdio argument (`-`)
    pub fn is_stdio(&self) -> bool {
        self.inner == "-"
    }

    /// Does the argument look like an argument escape (`--`)
    pub fn is_escape(&self) -> bool {
        self.inner == "--"
    }

    /// Does the argument look like a negative number?
    ///
    /// This won't parse the number in full but attempts to see if this looks
    /// like something along the lines of `-3`, `-0.3`, or `-33.03`
    pub fn is_negative_number(&self) -> bool {
        self.to_value()
            .ok()
            .and_then(|s| Some(is_number(s.strip_prefix('-')?)))
            .unwrap_or_default()
    }

    /// Treat as a long-flag
    pub fn to_long(&self) -> Option<(Result<&str, &OsStr>, Option<&OsStr>)> {
        let raw = self.inner;
        let remainder = raw.strip_prefix("--")?;
        if remainder.is_empty() {
            debug_assert!(self.is_escape());
            return None;
        }

        let (flag, value) = if let Some((p0, p1)) = remainder.split_once("=") {
            (p0, Some(p1))
        } else {
            (remainder, None)
        };
        let flag = flag.to_str().ok_or(flag);
        Some((flag, value))
    }

    /// Can treat as a long-flag
    pub fn is_long(&self) -> bool {
        self.inner.starts_with("--") && !self.is_escape()
    }

    /// Treat as a short-flag
    pub fn to_short(&self) -> Option<ShortFlags<'_>> {
        if let Some(remainder_os) = self.inner.strip_prefix("-") {
            if remainder_os.starts_with("-") {
                None
            } else if remainder_os.is_empty() {
                debug_assert!(self.is_stdio());
                None
            } else {
                Some(ShortFlags::new(remainder_os))
            }
        } else {
            None
        }
    }

    /// Can treat as a short-flag
    pub fn is_short(&self) -> bool {
        self.inner.starts_with("-") && !self.is_stdio() && !self.inner.starts_with("--")
    }

    /// Treat as a value
    ///
    /// <div class="warning">
    ///
    /// **NOTE:** May return a flag or an escape.
    ///
    /// </div>
    pub fn to_value_os(&self) -> &OsStr {
        self.inner
    }

    /// Treat as a value
    ///
    /// <div class="warning">
    ///
    /// **NOTE:** May return a flag or an escape.
    ///
    /// </div>
    pub fn to_value(&self) -> Result<&str, &OsStr> {
        self.inner.to_str().ok_or(self.inner)
    }

    /// Safely print an argument that may contain non-UTF8 content
    ///
    /// This may perform lossy conversion, depending on the platform. If you would like an implementation which escapes the path please use Debug instead.
    pub fn display(&self) -> impl std::fmt::Display + use<'_> {
        self.inner.to_string_lossy()
    }
}

/// Walk through short flags within a [`ParsedArg`]
#[derive(Clone, Debug)]
pub struct ShortFlags<'s> {
    inner: &'s OsStr,
    utf8_prefix: std::str::CharIndices<'s>,
    invalid_suffix: Option<&'s OsStr>,
}

impl<'s> ShortFlags<'s> {
    fn new(inner: &'s OsStr) -> Self {
        let (utf8_prefix, invalid_suffix) = split_nonutf8_once(inner);
        let utf8_prefix = utf8_prefix.char_indices();
        Self {
            inner,
            utf8_prefix,
            invalid_suffix,
        }
    }

    /// Move the iterator forward by `n` short flags
    pub fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        for i in 0..n {
            self.next().ok_or(i)?.map_err(|_| i)?;
        }
        Ok(())
    }

    /// No short flags left
    pub fn is_empty(&self) -> bool {
        self.invalid_suffix.is_none() && self.utf8_prefix.as_str().is_empty()
    }

    /// Does the short flag look like a number
    ///
    /// Ideally call this before doing any iterator
    pub fn is_negative_number(&self) -> bool {
        self.invalid_suffix.is_none() && is_number(self.utf8_prefix.as_str())
    }

    /// Advance the iterator, returning the next short flag on success
    ///
    /// On error, returns the invalid-UTF8 value
    pub fn next_flag(&mut self) -> Option<Result<char, &'s OsStr>> {
        if let Some((_, flag)) = self.utf8_prefix.next() {
            return Some(Ok(flag));
        }

        if let Some(suffix) = self.invalid_suffix {
            self.invalid_suffix = None;
            return Some(Err(suffix));
        }

        None
    }

    /// Advance the iterator, returning everything left as a value
    pub fn next_value_os(&mut self) -> Option<&'s OsStr> {
        if let Some((index, _)) = self.utf8_prefix.next() {
            self.utf8_prefix = "".char_indices();
            self.invalid_suffix = None;
            // SAFETY: `char_indices` ensures `index` is at a valid UTF-8 boundary
            let remainder = unsafe { ext::split_at(self.inner, index).1 };
            return Some(remainder);
        }

        if let Some(suffix) = self.invalid_suffix {
            self.invalid_suffix = None;
            return Some(suffix);
        }

        None
    }
}

impl<'s> Iterator for ShortFlags<'s> {
    type Item = Result<char, &'s OsStr>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_flag()
    }
}

fn split_nonutf8_once(b: &OsStr) -> (&str, Option<&OsStr>) {
    match b.try_str() {
        Ok(s) => (s, None),
        Err(err) => {
            // SAFETY: `err.valid_up_to()`, which came from str::from_utf8(), is guaranteed
            // to be a valid UTF8 boundary
            let (valid, after_valid) = unsafe { ext::split_at(b, err.valid_up_to()) };
            let valid = valid.try_str().unwrap();
            (valid, Some(after_valid))
        }
    }
}

fn is_number(arg: &str) -> bool {
    // Return true if this looks like an integer or a float where it's all
    // digits plus an optional single dot after some digits.
    //
    // For floats allow forms such as `1.`, `1.2`, `1.2e10`, etc.
    let mut seen_dot = false;
    let mut position_of_e = None;
    for (i, c) in arg.as_bytes().iter().enumerate() {
        match c {
            // Digits are always valid
            b'0'..=b'9' => {}

            // Allow a `.`, but only one, only if it comes before an
            // optional exponent, and only if it's not the first character.
            b'.' if !seen_dot && position_of_e.is_none() && i > 0 => seen_dot = true,

            // Allow an exponent `e`/`E` but only at most one after the first
            // character.
            b'e' | b'E' if position_of_e.is_none() && i > 0 => position_of_e = Some(i),

            _ => return false,
        }
    }

    // Disallow `-1e` which isn't a valid float since it doesn't actually have
    // an exponent.
    match position_of_e {
        Some(i) => i != arg.len() - 1,
        None => true,
    }
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
