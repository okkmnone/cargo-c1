#![cfg_attr(false, no_std)]
#![deny(missing_docs)]

//! A tiny crate that provides a simple command as an alternative to `cargo clean`.

use std::path::Path;

/// Remove only one directory. Remove the `target` directory only if `Cargo.toml` exists within the specified directory.
pub fn cargo_clean1<P: AsRef<Path>>(dir: P) -> std::io::Result<()> {
    let _cargo_toml = dir
        .as_ref()
        .join(rlitrs::file_cargo_toml!())
        .canonicalize()?;
    let target_dir = dir.as_ref().join(rlitrs::dir_target!()).canonicalize()?;
    std::fs::remove_dir_all(target_dir)
}
