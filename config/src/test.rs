use crate::{config, structs::Config, traits::ConfigStruct};

#[test]
fn test_volumes_basic() {
    let volume = Config::load("./sample.toml".into()).unwrap();
    assert!(volume.healthcheck())
}

#[test]
fn test_macros() {
    let volume = config!("./sample.toml").unwrap();
    assert!(volume.healthcheck())
}
