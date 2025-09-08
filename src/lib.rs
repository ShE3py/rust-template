use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LintLevel {
    Allow,
    Warn,
    Deny,
    Forbid,
}

impl LintLevel {
    pub fn letter(self) -> char {
        match self {
            LintLevel::Allow => 'A',
            LintLevel::Warn => 'W',
            LintLevel::Deny => 'D',
            LintLevel::Forbid => 'F',
        }
    }
}

impl FromStr for LintLevel {
    type Err = Infallible;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "allow" => LintLevel::Allow,
            "warn" => LintLevel::Warn,
            "deny" => LintLevel::Deny,
            "forbid" => LintLevel::Forbid,
            _ => panic!("unknown lint level: {s}")
        })
    }
}

pub type LintStore = BTreeMap<String, LintLevel>;

pub fn parse(path: impl AsRef<Path>) -> LintStore {
    let mut store = LintStore::new();
    
    for ln in BufReader::new(File::open(path).expect("fopen")).lines().map(|ln| ln.expect("fget")) {
        let Some((lint, level)) = ln.split_once('=') else {
            panic!("malformed line: {ln}");
        };
        
        store.insert(lint.to_owned(), LintLevel::from_str(level).unwrap());
    }
    
    store
}
