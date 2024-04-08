//! This has been copied pretty much wholesale from https://github.com/nrf-rs/nrf-hal/blob/master/xtask/src/lib.rs
use super::CRATES;
use chrono::Local;
use std::fs;

fn file_replace(path: &str, from: &str, to: &str, dry_run: bool) {
    let old_contents = fs::read_to_string(path).unwrap();
    let new_contents = old_contents.replacen(from, to, 1);
    if old_contents == new_contents {
        panic!("failed to replace `{}` -> `{}` in `{}`", from, to, path);
    }

    if !dry_run {
        fs::write(path, new_contents).unwrap();
    }
}

/// Bumps the versions of all crates and the changelog to `new_version`.
///
/// Dependency declarations are updated automatically. `html_root_url` is updated automatically.
pub fn bump_versions(new_version: &str, dry_run: bool) {
    let common_toml_path = "microbit-common/Cargo.toml";
    let toml = fs::read_to_string(common_toml_path).unwrap();

    let needle = "version = \"";
    let version_pos = toml.find(needle).unwrap() + needle.len();
    let version_rest = &toml[version_pos..];
    let end_pos = version_rest.find('"').unwrap();
    let old_version = &version_rest[..end_pos];

    {
        // Bump the changelog first, also check that it isn't empty.
        let changelog_path = "CHANGELOG.md";
        let changelog = fs::read_to_string(changelog_path).unwrap();
        // (ignore empty changelog when this is a dry_run, since that runs in normal CI)
        assert!(
            dry_run || !changelog.contains("(no changes)"),
            "changelog contains `(no changes)`; please fill it"
        );

        // Prepend empty "[Unreleased]" section, promote the current one.
        let today = Local::now().date_naive().format("%Y-%m-%d").to_string();
        let from = String::from("## [Unreleased]");
        let to = format!(
            "## [Unreleased]\n\n(no changes)\n\n## [{}] - {}",
            new_version, today
        );
        file_replace(changelog_path, &from, &to, dry_run);

        // Replace the Unreleased link
        let from = format!(
            r#"[Unreleased]: https://github.com/nrf-rs/microbit/compare/v{old_version}...HEAD"#,
            old_version = old_version,
        );
        let to = format!(
            "[Unreleased]: https://github.com/nrf-rs/microbit/compare/v{new_version}...HEAD\n\
             [{new_version}]: https://github.com/nrf-rs/microbit/compare/v{old_version}...v{new_version}",
            new_version = new_version,
            old_version = old_version,
        );
        file_replace(changelog_path, &from, &to, dry_run);
    }

    {
        println!("microbit-common: {} -> {}", old_version, new_version);

        // Bump `microbit-common`'s version.
        let from = format!(r#"version = "{}""#, old_version);
        let to = format!(r#"version = "{}""#, new_version);
        file_replace("microbit-common/Cargo.toml", &from, &to, dry_run);

        // Bump the `html_root_url`.
        let from = format!(
            r#"#![doc(html_root_url = "https://docs.rs/microbit-common/{old_version}")]"#,
            old_version = old_version
        );
        let to = format!(
            r#"#![doc(html_root_url = "https://docs.rs/microbit-common/{new_version}")]"#,
            new_version = new_version
        );
        let librs_path = "microbit-common/src/lib.rs";
        file_replace(librs_path, &from, &to, dry_run);
    }

    for (crate_name, _, _) in CRATES {
        println!("{}: {} -> {}", crate_name, old_version, new_version);
        let toml_path = format!("{}/Cargo.toml", crate_name);

        // Bump the crate's version.
        let from = format!(r#"version = "{}""#, old_version);
        let to = format!(r#"version = "{}""#, new_version);
        file_replace(&toml_path, &from, &to, dry_run);

        // Bump the crate's dependency on `microbit-common`.
        let from = format!(r#"version = "={}""#, old_version);
        let to = format!(r#"version = "={}""#, new_version);
        file_replace(&toml_path, &from, &to, dry_run);

        // Bump the crate's `html_root_url`.
        let from = format!(
            r#"#![doc(html_root_url = "https://docs.rs/{crate}/{old_version}")]"#,
            crate = crate_name,
            old_version = old_version
        );
        let to = format!(
            r#"#![doc(html_root_url = "https://docs.rs/{crate}/{new_version}")]"#,
            crate = crate_name,
            new_version = new_version
        );
        let librs_path = format!("{}/src/lib.rs", crate_name);
        file_replace(&librs_path, &from, &to, dry_run);
    }
}
