#!/bin/bash

set -e

# Check same count of lines
diff <(wc -l < lint-levels.txt) <(wc -l < default-lint-levels.txt)

# Update lint store
clippy-driver +nightly -Z unstable-options --print=crate-root-lint-levels --edition=2024 /dev/null | sort > default-lint-levels.stdout
cd tools
cargo run --bin relint
cd ..
mv default-lint-levels.stdout default-lint-levels.txt
