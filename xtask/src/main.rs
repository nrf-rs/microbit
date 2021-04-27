use std::env;

use xtask::ci;

fn main() {
    let mut args = env::args().skip(1);
    let subcommand = args.next();
    match subcommand.as_deref() {
        Some("ci") => ci(),
        _ => {
            eprintln!("usage: cargo xtask <subcommand>");
            eprintln!();
            eprintln!("subcommands:");
            eprintln!("  ci - run continuous integration checks (build and clippy)");
        }
    }
}
