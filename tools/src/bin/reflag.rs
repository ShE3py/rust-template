use rust_template::{parse, LintLevel};

fn main() {
    let defaults = parse("default-lint-levels.txt");
    let overrides = parse("lint-levels.txt");
    
    // RustcPrinter, CargoConfigPrinter
    let mut printer = CargoConfigPrinter::default();
    
    for ((default_lint, default_level), (overriden_lint, overriden_level)) in defaults.into_iter().zip(overrides) {
        assert_eq!(default_lint, overriden_lint);
        
        if default_level != overriden_level {
            assert!(overriden_level > default_level, "`{overriden_lint}` was lowered from {default_level:?} to {overriden_level:?}");
            
            printer.print(&overriden_lint, overriden_level);
        }
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
        let len = (r#""-W", "#.len() + lint.len()) as u16;
        
        if self.col + len > self.max_line_len {
            println!();
            self.col = 0;
        }
        self.col += len;
        
        print!(r#""-{letter}{lint}", "#, letter = level.letter());
    }
}

impl Printer for RustcPrinter {
    fn print(&mut self, lint: &str, level: LintLevel) {
        print!(r#"-{letter}{lint} "#, letter = level.letter());
    }
}
