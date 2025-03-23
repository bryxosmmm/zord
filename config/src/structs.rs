use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use serde::Deserialize;

use crate::traits::ConfigStruct;

#[derive(Deserialize, Default, PartialEq, Clone)]
pub struct Config {
    pub volumes: Volumes,
    pub options: Opts,
}

#[derive(Deserialize, Default, PartialEq, Clone)]
pub struct Opts {
    command: String,
}

#[derive(Deserialize, Default, PartialEq, Clone)]
pub struct Volumes {
    dirs: Vec<String>,
}

impl Volumes {
    pub fn dirs(&self) -> Vec<String> {
        self.dirs.clone()
    }
}

impl Opts {
    pub fn command(&self) -> String {
        self.command.clone()
    }
}

impl ConfigStruct for Volumes {
    fn load(path: String) -> anyhow::Result<Self> {
        let mut buf = String::new();
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let _ = reader.read_to_string(&mut buf);
        Ok(toml::from_str(&buf)?)
    }

    fn healthcheck(&self) -> bool {
        self.dirs.iter().all(|item| {
            let p = Path::new(item);
            p.is_dir() && p.is_absolute()
        })
    }
}

impl ConfigStruct for Opts {
    fn healthcheck(&self) -> bool {
        let p = Path::new(&self.command);
        p.is_absolute() && p.is_file()
    }
}

impl ConfigStruct for Config {
    fn load(path: String) -> anyhow::Result<Self> {
        let mut buf = String::new();
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let _ = reader.read_to_string(&mut buf);

        Ok(toml::from_str(&buf)?)
    }
    fn healthcheck(&self) -> bool {
        self.volumes.healthcheck()
    }
}
