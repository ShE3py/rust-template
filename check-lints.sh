#!/bin/bash

clippy-driver +nightly -Z unstable-options --print=crate-root-lint-levels --edition=2024 /dev/null | sort | diff - default-lint-levels.txt
