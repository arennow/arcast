use structopt::StructOpt;

use std::path::{Path, PathBuf};

// arcast [-h] [-p] [-e] [-n NUM] -c PATH -d PATH
//done-h	Print help information
//    -p	Pretend (don't download anything)
//done-e	Show existing files
//done-n	Limit number of episodes
//    -c	Path to configuration file
//done-d	Download directory path

#[derive(Debug, StructOpt)]
pub struct Config {
	/// Download directory path
	#[structopt(short, long)]
	destination: PathBuf,

	/// Print existing episodes
	#[structopt(short = "e", long)]
	print_existing_episodes: bool,

	/// Limit number of episodes
	#[structopt(short, long)]
	number_to_download: Option<usize>,
}

impl Config {
	pub fn destination(&self) -> &Path {
		&self.destination
	}

	pub fn print_existing_episodes(&self) -> bool {
		self.print_existing_episodes
	}

	pub fn number_to_download(&self) -> usize {
		self.number_to_download.unwrap_or(usize::MAX)
	}
}
