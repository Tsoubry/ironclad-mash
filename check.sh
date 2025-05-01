#!/bin/bash
set -e

cargo check

cargo +nightly fmt --all -- --check

cargo clippy -- -D warnings

cargo test

cargo deny check advisories bans sources
