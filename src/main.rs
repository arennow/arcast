mod download;

fn main() -> Result<(), ureq::Error> {
	let mut args = std::env::args().skip(1);
	let source_url = args.next().unwrap();
	let dest_path = args.next().unwrap();

	download::download_to_file(&source_url, dest_path)?;

	Ok(())
}
