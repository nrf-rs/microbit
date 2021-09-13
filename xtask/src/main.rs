use std::env;

use xtask::{bump_versions, ci, publish};

fn main() {
    let mut args = env::args().skip(1);
    let subcommand = args.next();
    match subcommand.as_deref() {
        Some("bump") => {
            let new_version = args.next().expect("missing <semver> argument");
            bump_versions(&new_version, false);
        }
        Some("ci") => ci(),
        Some("publish") => publish(),
        _ => {
            eprintln!("usage: cargo xtask <subcommand>");
            eprintln!();
            eprintln!("subcommands:");
            eprintln!("  ci      - run continuous integration checks (build and clippy)");
            eprintln!("  bump    - bump the crate version and update docs and changelog");
            eprintln!("  publish - publish all crates to crates.io");
        }
    }
}
