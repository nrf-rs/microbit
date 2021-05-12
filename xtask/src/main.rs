use std::env;

use xtask::{bump_versions, ci};

fn main() {
    let mut args = env::args().skip(1);
    let subcommand = args.next();
    match subcommand.as_deref() {
        Some("ci") => ci(),
        Some("bump") => {
            let new_version = args.next().expect("missing <semver> argument");
            bump_versions(&new_version, false);
        }
        _ => {
            eprintln!("usage: cargo xtask <subcommand>");
            eprintln!();
            eprintln!("subcommands:");
            eprintln!("  ci - run continuous integration checks (build and clippy)");
        }
    }
}
