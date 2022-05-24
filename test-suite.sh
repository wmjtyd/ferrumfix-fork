#!/usr/bin/env bash

git submodule init
git submodule update

mkdir lib/quickfix/config
pushd lib/quickfix/config || exit
cmake ..
make
popd || exit

# Increase number of iteration for QuickCheck.
export QUICKCHECK_TESTS="2500"

# Default features
cargo test
# Test multiple feature combinations.
cargo test --no-default-features
cargo test --no-default-features --features "fix42"
cargo test --no-default-features --features "fixt11"
cargo test --no-default-features --features "fixs"
cargo test --no-default-features --features "utils-bytes, utils-rust-decimal"
cargo test --no-default-features --features "fixs, utils-openssl, fix40"
cargo test --no-default-features --features "derive, fix43"
cargo test --no-default-features --features "full"

RUSTDOCFLAGS="--cfg doc_cfg" cargo +nightly doc --all-features
