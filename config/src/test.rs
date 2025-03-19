use std::fs::File;

use crate::{config, structs::Volumes, traits::ConfigStruct};

#[test]
fn test_volumes_basic() {
    let volume = Volumes::load("./sample.toml".into()).unwrap();
    assert!(volume.healthcheck())
}

#[test]
fn test_volumes_from() {
    let file = File::open("./sample.toml").unwrap();
    let volume = Volumes::from(file);
    assert!(volume.healthcheck())
}

#[test]
fn test_macros() {
    let volume = config!("./sample.toml").unwrap();
    assert!(volume.healthcheck())
}
