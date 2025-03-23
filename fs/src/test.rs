use crate::{files, structs::Files, traits::Collector};

#[test]
fn test_collect_files() {
    let files = Files::collect(
        "/mnt/diskd/Cartoons/Adventure Time (1080p HEVC)/Season 01/Season 01/**/*.mkv",
    );
    assert!(!files.inner().is_empty());
}

#[test]
fn test_collect_files_macros() {
    let path = "./src";
    let files: Vec<Files> = files![path => "**/*.rs", "**/*.txt"];
    let result = Files::from(files);
    assert!(!result.inner().is_empty());
}
