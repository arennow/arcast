use std::io::{BufWriter, Read, Result, Write};
use std::path::Path;

use super::heap_buffer::*;

pub fn download_to_file<P: AsRef<Path>>(source_url: &str, destination: P) -> Result<usize> {
	let mut downloader = ureq::get(source_url)
		.call()
		.map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))?
		.into_reader();

	let mut file = std::fs::File::create(destination)?;

	pipe(&mut downloader, &mut file)
}

fn pipe<R: Read, W: Write>(source: &mut R, dest: &mut W) -> Result<usize> {
	let mut dest = BufWriter::new(dest);
	let mut bytes_written = 0;
	let mut buf = HeapBuffer::<{ 1024 * 8 }>::new();
	// 8 KiB is the size of the BufWriter buffer and is also seemingly what we get from ureq's result reader
	// So I guess this function allocates 16 KiB of buffers?

	while let Result::Ok(bytes_read) = source.read(&mut buf) {
		if bytes_read == 0 {
			break;
		}

		dest.write_all(&buf[..bytes_read])?;
		bytes_written += bytes_read;
	}

	dest.flush()?;

	Ok(bytes_written)
}
