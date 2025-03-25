use std::process::Command;
use std::process::Stdio;

use config::Config;
use config::ConfigStruct;
use dioxus::events::Key;
use fs::files;
use fs::Collector;
use fs::Files;
use fs::Metadata;
use nucleo_matcher::pattern::CaseMatching;
use nucleo_matcher::pattern::Normalization;
use nucleo_matcher::pattern::Pattern;
use nucleo_matcher::Matcher;

use config::config;
use rayon::prelude::*;

pub fn load() -> anyhow::Result<Config> {
    let home = env!("HOME");
    let path = format!("{home}/.config/zord/config.toml");
    let config: Config = config!(path)?;
    if !config.healthcheck() {
        return Err(anyhow::anyhow!("Healthcheck failed"));
    }
    Ok(config)
}

pub fn load_list(dirs: Vec<String>) -> Vec<fs::Metadata> {
    dirs.par_iter()
        .map(|entry| {
            let files = files!(entry => "**/*.mkv", "**/*.avi", "**/.mp4");
            Files::from(files).inner()
        })
        .flatten()
        .collect()
}

#[inline]
pub fn sort(items: Vec<fs::Metadata>, input: String) -> Vec<Metadata> {
    let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);
    let matches = Pattern::parse(input.as_str(), CaseMatching::Ignore, Normalization::Smart)
        .match_list(items, &mut matcher);
    matches.into_iter().map(|(i, _)| i).collect()
}

#[inline]
pub fn handle_keyboard(e: Key) {
    if e == Key::Escape {
        std::process::exit(0);
    }
}

#[inline]
pub fn handle_click(path: String, command: String) {
    let _ = Command::new(command)
        .arg(&path)
        .stdout(Stdio::null())
        .spawn();
    std::process::exit(0);
}
