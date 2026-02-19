//!
//! Reflag: print `lint-levels.txt` for a command line,
//!   ignoring unchanged values via `default-lint-levels.txt`.
//!

mod relint;

use rust_template::{is_stable, parse, LintLevel};

fn main() {
    let defaults = parse("default-lint-levels.txt");
    let overrides = parse("lint-levels.txt");
    
    // RustcPrinter, CargoConfigPrinter
    let mut printer = CargoConfigPrinter::default();
    
    let mut skipped = Vec::new();
    
    for ((default_lint, default_level), (overriden_lint, overriden_level)) in defaults.into_iter().zip(overrides) {
        debug_assert_eq!(default_lint, overriden_lint);
        
        if default_level != overriden_level {
//          assert!(overriden_level > default_level, "`{overriden_lint}` was lowered from {default_level:?} to {overriden_level:?}");
            
            if !is_stable(&overriden_lint) {
                skipped.push(overriden_lint);
            }
            else {
                printer.print(&overriden_lint, overriden_level);
            }
        }
    }
    
    println!();
    
    if !skipped.is_empty() {
        eprintln!();
        eprintln!("warn: skipped {} unstable lint{}:", skipped.len(), if skipped.len() == 1 { "" } else { "s" });
        eprintln!("{}", skipped.join(", "));
    }
}

trait Printer {
    fn print(&mut self, lint: &str, level: LintLevel);
}

#[allow(dead_code)]
struct CargoConfigPrinter {
    max_line_len: u16,
    col: u16,
}

#[allow(dead_code)]
#[derive(Default)]
struct RustcPrinter;

impl Default for CargoConfigPrinter {
    fn default() -> Self {
        CargoConfigPrinter {
            max_line_len: 120,
            col: 0,
        }
    }
}

impl Printer for CargoConfigPrinter {
    fn print(&mut self, lint: &str, level: LintLevel) {
        let len = u16::try_from(match level.letter() {
            Some(_) => "-W".len(),
            None => "--".len() + level.as_str().len() + '='.len_utf8(),
        } + 2 * '"'.len_utf8() + lint.len() + ", ".len()).unwrap();
        
        if self.col + len > self.max_line_len {
            println!();
            self.col = 0;
        }
        self.col += len;
        
        print!("{:?}, ", level.as_arg(lint));
    }
}

impl Printer for RustcPrinter {
    fn print(&mut self, lint: &str, level: LintLevel) {
        print!("{} ", level.as_arg(lint));
    }
}
