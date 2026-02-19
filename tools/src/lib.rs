use std::io::Write;
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::{fmt, io};
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum LintLevel {
    Allow,
    Warn,
    ForceWarn,
    Deny,
    Forbid,
}

impl PartialOrd for LintLevel {
    /// [`ForceWarn`] and [`Deny`] are incomparable (as [`ForceWarn`] cannot be `#[allow]`'ed)
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use LintLevel::*;
        match (*self, *other) {
            (Allow, Allow) => Some(Ordering::Equal),
            (Allow, _) => Some(Ordering::Less),
            
            (Warn, Allow) => Some(Ordering::Greater),
            (Warn, Warn) => Some(Ordering::Equal),
            (Warn, _) => Some(Ordering::Less),
            
            (ForceWarn, Allow | Warn) => Some(Ordering::Greater),
            (ForceWarn, ForceWarn) => Some(Ordering::Equal),
            (ForceWarn, Deny) => None,
            (ForceWarn, Forbid) => Some(Ordering::Less),
            
            (Deny, Allow | Warn) => Some(Ordering::Greater),
            (Deny, ForceWarn) => None,
            (Deny, Deny) => Some(Ordering::Equal),
            (Deny, Forbid) => Some(Ordering::Less),
            
            (Forbid, Forbid) => Some(Ordering::Equal),
            (Forbid, _) => Some(Ordering::Less)
        }
    }
}

impl LintLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            LintLevel::Allow => "allow",
            LintLevel::Warn => "warn",
            LintLevel::ForceWarn => "force-warn",
            LintLevel::Deny => "deny",
            LintLevel::Forbid => "forbid",
        }
    }
    
    pub fn letter(self) -> Option<char> {
        match self {
            LintLevel::Allow => Some('A'),
            LintLevel::Warn => Some('W'),
            LintLevel::ForceWarn => None,
            LintLevel::Deny => Some('D'),
            LintLevel::Forbid => Some('F'),
        }
    }
    
    /// Relaxes [`Forbid`] to [`Deny`], and [`ForceWarn`]
    /// to [`Warn`].
    pub fn overridable(self) -> Self {
        match self {
            LintLevel::Allow => LintLevel::Allow,
            LintLevel::Warn => LintLevel::Warn,
            LintLevel::ForceWarn => LintLevel::Warn,
            LintLevel::Deny => LintLevel::Deny,
            LintLevel::Forbid => LintLevel::Deny,
        }
    }
    
    pub fn as_arg(self, lint: &str) -> String {
        match self.letter() {
            Some(letter) => format!("-{letter}{lint}"),
            None => format!("--{}={lint}", self.as_str()),
        }
    }
}

impl fmt::Display for LintLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for LintLevel {
    type Err = Infallible;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "allow" | "A" => LintLevel::Allow,
            "warn" | "W" => LintLevel::Warn,
            "force-warn" => LintLevel::ForceWarn,
            "deny" | "D" => LintLevel::Deny,
            "forbid" | "F" => LintLevel::Forbid,
            _ => panic!("unknown lint level: {s}")
        })
    }
}

pub type LintStore = BTreeMap<String, LintLevel>;

pub fn parse(path: impl AsRef<Path>) -> LintStore {
    let mut store = LintStore::new();
    
    for ln in BufReader::new(File::open(Path::new("../").join(path)).expect("fopen")).lines().map(|ln| ln.expect("fget")) {
        let Some((lint, level)) = ln.split_once('=') else {
            panic!("malformed line: {ln}");
        };
        
        store.insert(lint.to_owned(), LintLevel::from_str(level).unwrap());
    }
    
    store
}

pub fn save(store: &LintStore, path: impl AsRef<Path>) -> io::Result<()> {
    let mut w = BufWriter::new(File::create(Path::new("../").join(path))?);
    for (k, v) in store {
        writeln!(w, "{k}={v}")?;
    }
    Ok(())
}

/// Includes a trailing newline.
pub fn version() -> String {
    String::from_utf8(
        Command::new("rustc")
            .arg("+nightly")
            .arg("-V")
            .output()
            .expect("could not spawn clippy-driver")
            .stdout
    ).expect("invalid utf8")
}

pub fn is_stable(lint: &str) -> bool {
    Command::new("clippy-driver")
        .arg("+stable")
        .arg("-Funknown_lints")
        .arg(format!("-F{lint}"))
        .arg("--crate-type=lib")
        .arg("-o-")
        .arg("-")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .expect("could not spawn clippy-driver")
        .success()
}
