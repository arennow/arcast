use std::io::{BufWriter, Read, Write};
use std::path::Path;

use super::error::*;
use super::heap_buffer::*;

pub fn download_to_reader(source_url: &str) -> Result<impl Read, DownloadError> {
	Ok(ureq::get(source_url).call()?.into_reader())
}

pub fn download_to_file<P: AsRef<Path>>(
	source_url: &str,
	dest_path: P,
) -> Result<usize, DownloadError> {
	let dest_path = dest_path.as_ref();
	let mut downloader = download_to_reader(source_url)?;

	let mut file = std::fs::File::create(&dest_path)
		.map_err(|e| DownloadError::from_io_error(e, dest_path.to_string_lossy()))?;

	pipe(&mut downloader, &mut file, dest_path.to_string_lossy())
}

fn pipe<R: Read, W: Write, S: Into<String>>(
	source: &mut R,
	dest: &mut W,
	dest_name: S,
) -> Result<usize, DownloadError> {
	let mut dest = BufWriter::new(dest);
	let mut bytes_written = 0;
	let mut buf = HeapBuffer::<{ 1024 * 8 }>::new();
	// 8 KiB is the size of the BufWriter buffer and is also seemingly what we get from ureq's result reader
	// So I guess this function allocates 16 KiB of buffers?

	DownloadError::handling_io_error_in(dest_name, || {
		loop {
			let bytes_read = source.read(&mut buf)?;
			if bytes_read == 0 {
				break;
			}
			dest.write_all(&buf[..bytes_read])?;

			bytes_written += bytes_read;
		}

		dest.flush()?;

		Ok(bytes_written)
	})
}
