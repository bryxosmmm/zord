use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use serde::Deserialize;

use crate::traits::ConfigStruct;

#[derive(Deserialize)]
pub struct Volumes {
    dirs: Vec<String>,
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
        self.dirs.iter().all(|item| Path::new(item).is_dir())
    }
}

impl From<File> for Volumes {
    fn from(value: File) -> Self {
        let mut buf = String::new();
        let mut reader = BufReader::new(value);
        let _ = reader
            .read_to_string(&mut buf)
            .expect("ERROR: while trying to read config");
        toml::from_str(&buf).expect("ERROR: while trying to parse config")
    }
}
