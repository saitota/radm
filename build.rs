use std::env;

fn main() {
    // The release version is sourced from the Git tag (passed via RADM_VERSION
    // by `task release`), with a leading `v` stripped so it matches SemVer. In
    // a plain `cargo build` RADM_VERSION is unset and we fall back to the
    // Cargo.toml version.
    let version = env::var("RADM_VERSION")
        .ok()
        .map(|v| v.trim_start_matches('v').to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| env::var("CARGO_PKG_VERSION").unwrap());

    println!("cargo:rustc-env=RADM_VERSION={version}");
    // Rebuild when the tag-derived version changes.
    println!("cargo:rerun-if-env-changed=RADM_VERSION");
}
