//!
//! Relint: interactively update outdated `default-lint-levels.txt`
//!

use rust_template::{parse, save, LintLevel};
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let old = parse("default-lint-levels.txt");
    let new = parse("default-lint-levels.stdout");
    let mut cur = parse("lint-levels.txt");
    
    let mut keys = Vec::from_iter(old.keys().chain(new.keys()));
    keys.sort_unstable(); // should be Θ(n) instead of O(n log n), but good enough
    keys.dedup();
    
    for key in keys {
        let Some(old_value) = old.get(key).copied() else {
            let new_value = new[key];
            eprintln!("info: lint {} was added with default value `{}`", hyperlint(key, new_value), new_value);
            ask(cur.entry(key.clone()).or_insert(new_value));
            continue
        };
        
        let Some(new_value) = new.get(key).copied() else {
            match cur.remove(key) {
                Some(cur_value) if cur_value != old_value => eprintln!("warn: lint `{key}` was deleted with default value `{old_value}` changed to `{cur_value}`"),
                _ => {}, // no need to inform about deletion of default settings
            }
            continue
        };
        
        let Some(cur_value) = cur.get_mut(key) else {
            eprintln!("error: lint {lint} with default value `{}` was neither added nor deleted, but is missing", new_value, lint = hyperlint(key, new_value));
            ask(cur.entry(key.clone()).or_insert(new_value));
            continue
        };
        
        if old_value != new_value {
            eprintln!(
                "warn: lint {lint} default was {verb} from `{old_value}` to `{new_value}`, while current value is `{cur_value}`",
                lint = hyperlint(key, new_value),
                verb = if new_value < old_value { "relaxed" } else { "upgraded" }
            );
            
            ask(cur_value);
        }
    }
    
    if let Err(e) = save(&cur, "lint-levels.txt") {
        eprintln!("failed to save: {e}");
    }
}

/// Asks for a new level
fn ask(v: &mut LintLevel) {
    let mut buf = String::new();
    
    loop {
        print!("new value: ");
        _ = io::stdout().flush();
        io::stdin().read_line(&mut buf).unwrap();
        let Ok(parsed) = LintLevel::from_str(buf.trim_ascii_end());
        *v = parsed;
        return;
    }
}

/// Prints lint name as an hyperlink.
fn hyperlint(k: &str, v: LintLevel) -> String {
    let url = match k.split_once("::") {
        None => format!(
            "https://doc.rust-lang.org/nightly/rustc/lints/listing/{}-by-default.html#{}",
            match v {
                LintLevel::Allow => "allowed",
                LintLevel::Warn => "warn",
                LintLevel::Deny => "deny",
                LintLevel::Forbid => unimplemented!(),
            },
            k.replace('_', "-"),
        ),
        
        Some((namespace, lint)) if namespace == "clippy" => format!("https://rust-lang.github.io/rust-clippy/master/index.html#{lint}"),
        
        _ => format!("`{k}`"),
    };
    
    format!("\u{1b}]8;;{url}\u{1b}\\`{k}`\u{1b}]8;;\u{1b}\\")
}
