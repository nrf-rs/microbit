mod bump;
mod ci;

pub static CRATES: &[(&str, &str, &str)] = &[
    ("microbit", "thumbv6m-none-eabi", "v1"),
    ("microbit-v2", "thumbv7em-none-eabihf", "v2"),
];

pub use bump::bump_versions;
pub use ci::ci;
