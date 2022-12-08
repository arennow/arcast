use super::error::*;
use super::heap_buffer::*;
use crate::filesystem::FilesystemError;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

pub fn download_to_reader(
	source_url: &str,
) -> Result<(impl Read, Option<usize>), Box<DownloadError>> {
	let agent = ureq::AgentBuilder::new().redirects(10).build();
	let resp = agent.get(source_url).call()?;
	let content_length = resp.header("Content-Length").and_then(|s| s.parse().ok());
	Ok((resp.into_reader(), content_length))
}

pub fn download_to_file<P: AsRef<Path>, PF>(
	source_url: &str,
	dest_path: P,
	mut progress_func: PF,
) -> Result<usize, Box<DownloadError>>
where
	PF: FnMut(f64),
{
	let dest_path = dest_path.as_ref();
	let (mut downloader, content_length) = download_to_reader(source_url)?;

	let mut file = std::fs::File::create(&dest_path)
		.map_err(|e| FilesystemError::from_io_error(e, dest_path.to_string_lossy()))?;

	fn us_div(num: usize, den: Option<usize>) -> f64 {
		den.map(|d| (num as f64) / (d as f64)).unwrap_or_default()
	}

	pipe(
		&mut downloader,
		&mut file,
		dest_path.to_string_lossy(),
		|cur| progress_func(us_div(cur, content_length)),
	)
}

fn pipe<R: Read, W: Write, S: Into<String>, PF>(
	source: &mut R,
	dest: &mut W,
	dest_name: S,
	mut progress_func: PF,
) -> Result<usize, Box<DownloadError>>
where
	PF: FnMut(usize),
{
	let mut dest = BufWriter::new(dest);
	let mut bytes_written = 0;
	let mut buf = HeapBuffer::<{ 1024 * 8 }>::new();
	// 8 KiB is the size of the BufWriter buffer and is also seemingly what we get from ureq's result reader
	// So I guess this function allocates 16 KiB of buffers?

	FilesystemError::handling_io_error_in(dest_name, || {
		loop {
			let bytes_read = source.read(&mut buf)?;
			if bytes_read == 0 {
				break;
			}
			dest.write_all(&buf[..bytes_read])?;
			bytes_written += bytes_read;

			progress_func(bytes_written);
		}

		dest.flush()?;

		Ok(bytes_written)
	})
	.map_err(std::convert::Into::into)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_pipe() {
		let src: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
		let mut src_cur = std::io::Cursor::new(&src);
		let mut dest = [0; 8];
		let mut dest_cur = std::io::Cursor::new(&mut dest[..]);

		pipe(&mut src_cur, &mut dest_cur, "idk", |_| {}).unwrap();

		assert_eq!(src, dest);
	}
}
