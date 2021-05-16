//use ffprobe::{ffprobe, FfProbe, Stream};
use ffprobe::{ffprobe};
use std::path::Path;

fn main() {
	let probe = ffprobe(Path::new("nogit/video.mp4")).unwrap();
	for (i, stream) in probe.streams.into_iter().enumerate() {
			println!("Stream #{} codec_name: {}", i, stream.codec_name);
	}
}
