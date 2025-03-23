use std::path::Path;

use ffprobe::Stream;

pub fn to_streams(p: &Path) -> anyhow::Result<Vec<Stream>> {
    Ok(ffprobe::ffprobe(p)?.streams)
}
