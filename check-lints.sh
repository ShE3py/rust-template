#!/bin/bash

set -ex

diff <(sort lint-levels.txt) lint-levels.txt
diff <(clippy-driver +nightly -Z unstable-options --print=crate-root-lint-levels --edition=2024 /dev/null | sort) default-lint-levels.txt
diff <(wc -l < lint-levels.txt) <(wc -l < default-lint-levels.txt)
