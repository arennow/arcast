mod config;
mod download;
mod feed;
mod filesystem;
mod helpers;

use structopt::StructOpt;

fn do_work() -> Result<(), Box<dyn std::error::Error>> {
	let config = config::Config::from_args();

	let reader = download::download_to_reader("https://newrustacean.com/feed.xml")?;
	let episodes = feed::episodes_from_reader(reader)?;
	let classified_eps = helpers::classified_episodes(&episodes, &config)?;

	helpers::process_classified_episodes(classified_eps, &config)?;
	Ok(())
}

fn main() {
	if let Err(e) = do_work() {
		eprintln!("Error: {}", e);
	}
}
