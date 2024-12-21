#!/bin/bash

cargo fmt && cargo clippy -- -D warnings && cargo test --release --quiet
