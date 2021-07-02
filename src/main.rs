mod download;
mod feed;

use std::error::Error;

fn do_work() -> Result<(), Box<dyn Error>> {
	let reader = download::download_to_reader("https://feeds.fireside.fm/worldsgreatestcon/rss")?;
	let items = feed::items_from_reader(reader)?;

	for item in items {
		println!("{}", item.title().unwrap_or("NO TITLE"));
	}

	Ok(())
}

fn main() {
	if let Err(e) = do_work() {
		eprintln!("Error: {}", e);
	}
}
