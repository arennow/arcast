mod config;
mod download;
mod feed;
mod filesystem;
mod helpers;

use structopt::StructOpt;

fn do_work() -> Result<(), Box<dyn std::error::Error>> {
	let config = config::Config::from_args();

	dbg!(&config);

	let reader = download::download_to_reader("https://newrustacean.com/feed.xml")?;
	let episodes = feed::episodes_from_reader(reader)?;
	let missing_eps = helpers::missing_episodes(&episodes, &config)?
		.rev()
		.take(config.number_to_download());

	for episode in missing_eps {
		helpers::download_episode(episode, &config)?;
	}

	Ok(())
}

fn main() {
	if let Err(e) = do_work() {
		eprintln!("Error: {}", e);
	}
}
