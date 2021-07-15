#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate lazy_static;

mod config;
mod download;
mod feed;
mod filesystem;
mod helpers;

use std::rc::Rc;
use structopt::StructOpt;

fn do_work() -> Result<(), Box<dyn std::error::Error>> {
	let config = config::Config::from_args();
	let show: feed::Show = {
		let config_file_handle = std::fs::File::open(config.config_file_path())?;
		serde_json::from_reader(config_file_handle)
	}?;
	let show = Rc::new(show);

	let reader = download::download_to_reader(show.url())?;
	let episodes = feed::episodes_from_reader(reader, show)?;
	let classified_eps = helpers::classified_episodes(&episodes, &config)?;

	helpers::process_classified_episodes(classified_eps, &config)?;
	Ok(())
}

fn main() {
	if let Err(e) = do_work() {
		eprintln!("Error: {}", e);
	}
}
