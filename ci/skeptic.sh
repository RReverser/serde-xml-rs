#!/bin/sh

sed -i  '/\[dev-dependencies\]/a skeptic = "0.13"' Cargo.toml

echo "[build-dependencies]
skeptic = \"0.13\"" >> Cargo.toml

cat <<EOT >> build.rs
extern crate skeptic;

fn main() {
    // generates doc tests for `README.md`.
    skeptic::generate_doc_tests(&["README.md"]);
}
EOT

echo 'include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));' > tests/skeptic.rs
