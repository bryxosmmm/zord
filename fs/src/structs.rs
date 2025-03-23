use crate::{traits::Collector, utils::to_streams};
use rayon::prelude::*;
use std::path::PathBuf;

pub struct Files(Vec<PathBuf>);

#[derive(Debug, PartialEq, Clone)]
pub struct Metadata {
    pub subtitles: u32,
    pub tracks: u32,
    pub name: String,
    pub path: String,
}

impl Files {
    pub fn new() -> Self {
        Files(vec![])
    }
}

impl Default for Files {
    fn default() -> Self {
        Self::new()
    }
}

impl Collector for Files {
    type Inner = Vec<Metadata>;

    fn inner(&self) -> Self::Inner {
        self.0
            .par_iter() // Use par_iter() for parallel processing
            .map(|p| {
                let streams = to_streams(p).unwrap_or_default();
                let (audio_count, subtitle_count) =
                    streams.iter().fold((0, 0), |(a, s), stream| {
                        match stream.codec_type.as_deref() {
                            Some("audio") => (a + 1, s),
                            Some("subtitle") => (a, s + 1),
                            _ => (a, s),
                        }
                    });

                let name = p
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                let path = p.display().to_string();

                Metadata {
                    tracks: audio_count,
                    subtitles: subtitle_count,
                    name,
                    path,
                }
            })
            .collect()
    }
    fn collect(pattern: &str) -> Self {
        let mathes = glob::glob(pattern);
        let entries: Vec<PathBuf> = match mathes {
            Ok(v) => v.into_iter().filter_map(|v| v.ok()).collect(),
            Err(_) => {
                vec![]
            }
        };
        Files(entries)
    }

    fn mutate(&self, other: &Self) -> Self {
        let mut c = self.0.clone();
        c.extend(other.0.clone());
        Files(c)
    }
}

impl AsRef<str> for Metadata {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl From<Vec<Files>> for Files {
    fn from(value: Vec<Files>) -> Self {
        let vecs: Vec<Vec<PathBuf>> = value.iter().map(|i| i.0.clone()).collect();
        let mut result = vec![];
        for v in vecs {
            result.extend(v);
        }
        Files(result)
    }
}
