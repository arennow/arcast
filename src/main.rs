mod download;
mod feed;

fn do_work() -> Result<(), Box<dyn std::error::Error>> {
	let reader = download::download_to_reader("https://newrustacean.com/feed.xml")?;
	let items = feed::items_from_reader(reader)?;

	for item in items {
		println!("{}", item.filename());
	}

	Ok(())
}

fn main() {
	if let Err(e) = do_work() {
		eprintln!("Error: {}", e);
	}
}
