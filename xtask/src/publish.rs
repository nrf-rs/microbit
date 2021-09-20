use std::process::Command;

use crate::CRATES;

pub fn publish() {
    publish_package("microbit-common", "thumbv7em-none-eabihf", Some("v2"));

    for (name, target, _) in CRATES {
        publish_package(name, target, None);
    }
}

fn publish_package(package: &str, target: &str, feature: Option<&str>) {
    let mut cargo = Command::new("cargo");
    cargo.args(&["publish", "--target", target, "--package", package]);
    if let Some(feature) = feature {
        cargo.args(&["--features", feature]);
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
