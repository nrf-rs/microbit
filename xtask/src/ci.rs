use super::CRATES;
use std::{collections::HashMap, env, fs, path, process::Command};

pub static DEPENDENCIES: &[&str] = &["flip-link"];

fn install_targets() {
    let targets = CRATES
        .iter()
        .map(|(_, target, _)| *target)
        .collect::<Vec<_>>();

    let mut rustup = Command::new("rustup");
    rustup.args(&["target", "add"]).args(&targets);
    let status = rustup
        .status()
        .map_err(|e| format!("couldn't execute {:?}: {}", rustup, e))
        .unwrap();
    assert!(
        status.success(),
        "failed to install targets with rustup: {:?}",
        rustup
    );
}

/// Install global dependencies
fn install_dependencies() {
    for dependency in DEPENDENCIES {
        let exists = Command::new("which")
            .arg(dependency)
            .output()
            .expect("failed to execute");
        if !exists.status.success() {
            let mut cargo = Command::new("cargo");
            cargo.args(&["install", dependency]);
            let status = cargo
                .status()
                .map_err(|e| format!("couldn't execute {:?}: {}", cargo, e))
                .unwrap();
            assert!(status.success(),);
        }
    }
}

/// Build-test each board support crate
fn build_crates() {
    for (hal, target, _) in CRATES {
        let mut cargo = Command::new("cargo");
        let toml_path = format!("{}/Cargo.toml", hal);
        let status = cargo
            .args(&["build", "--manifest-path", &toml_path, "--target", target])
            .status()
            .map_err(|e| format!("could not execute {:?}: {}", cargo, e))
            .unwrap();
        assert!(
            status.success(),
            "command exited with error status: {:?}",
            cargo
        );
    }
}

/// Build/Run doc-tests in `microbit-common` for each version.
fn build_run_doc_tests() {
    for (_, _, feature) in CRATES {
        let mut cargo = Command::new("cargo");
        let status = cargo
            .current_dir("microbit-common")
            .args(&["test", "--features", feature])
            .status()
            .map_err(|e| format!("could not execute {:?}: {}", cargo, e))
            .unwrap();
        assert!(
            status.success(),
            "command exited with error status: {:?}",
            cargo
        );
    }
}

/// Build all examples with the boards they support
fn build_examples() {
    let feature_targets = CRATES
        .iter()
        .map(|(_, target, feature)| (feature.to_string(), target.to_string()))
        .collect::<HashMap<_, _>>();

    let crate_targets = CRATES
        .iter()
        .map(|(name, target, _)| (name.to_string(), target.to_string()))
        .collect::<HashMap<_, _>>();

    for example in fs::read_dir("examples").unwrap() {
        let dir = example.unwrap();
        let manifest_path = dir.path().join("Cargo.toml");

        // Skip if there is no manifest
        if !manifest_path.exists() {
            continue;
        }

        let manifest = cargo_toml::Manifest::from_path(&manifest_path).unwrap();

        // find features and their targets supported by the example
        let mut features = manifest
            .features
            .keys()
            .filter_map(|feature| {
                feature_targets
                    .get(feature)
                    .map(|target| (Some(feature.to_owned()), target.to_owned()))
            })
            .collect::<Vec<_>>();

        // if there are no features find the target from the dependencies
        if features.is_empty() {
            features = manifest
                .dependencies
                .keys()
                .filter_map(|name| {
                    crate_targets
                        .get(name)
                        .map(|target| (None, target.to_owned()))
                })
                .collect::<Vec<_>>();
            assert_eq!(
                features.len(),
                1,
                "examples must depend on either microbit or microbit-v2"
            );
        }

        for (feature, target) in features {
            build_example(&manifest_path, feature, target);
        }
    }
}

fn build_example(manifest_path: &path::Path, feature: Option<String>, target: String) {
    let mut cargo = Command::new("cargo");
    cargo.args(&[
        "build",
        "--target",
        &target,
        "--manifest-path",
        manifest_path.to_str().unwrap(),
    ]);
    if let Some(feature) = feature {
        cargo.args(&["--features", &feature]);
    }

    let status = cargo
        .status()
        .map_err(|e| format!("could not execute {:?}: {}", cargo, e))
        .unwrap();

    assert!(
        status.success(),
        "command exited with error status: {:?}",
        cargo
    );
}

fn start_group(is_ci: bool, name: &str) {
    if is_ci {
        println!("::group::{}", name);
    }
}

fn end_group(is_ci: bool) {
    if is_ci {
        println!("::endgroup::");
    }
}

fn wrap_in_group(is_ci: bool, name: &str, callable: &dyn Fn()) {
    start_group(is_ci, name);
    callable();
    end_group(is_ci);
}

pub fn ci() {
    let is_ci = env::var("CI").map_or(false, |ci| ci == "true");

    // move up if we're running from inside xtask
    if env::current_dir().unwrap().ends_with("xtask") {
        env::set_current_dir("..").unwrap();
    }

    wrap_in_group(is_ci, "install targets", &install_targets);
    wrap_in_group(is_ci, "install dependencies", &install_dependencies);
    wrap_in_group(is_ci, "build crates", &build_crates);
    wrap_in_group(is_ci, "build examples", &build_examples);
    wrap_in_group(is_ci, "run doc tests", &build_run_doc_tests);
}
