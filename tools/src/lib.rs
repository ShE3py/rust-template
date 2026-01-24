use std::io::Write;
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::{fmt, io};
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum LintLevel {
    Allow,
    Warn,
    Deny,
    Forbid,
}

impl LintLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            LintLevel::Allow => "allow",
            LintLevel::Warn => "warn",
            LintLevel::Deny => "deny",
            LintLevel::Forbid => "forbid",
        }
    }
    
    pub fn letter(self) -> char {
        match self {
            LintLevel::Allow => 'A',
            LintLevel::Warn => 'W',
            LintLevel::Deny => 'D',
            LintLevel::Forbid => 'F',
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
