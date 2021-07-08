use structopt::StructOpt;

use std::path::{Path, PathBuf};

// arcast [-h] [-p] [-e] [-n NUM] -c PATH -d PATH
//    -h	Print help information
//    -p	Pretend (don't download anything)
//    -e	Show existing files
//    -n	Limit number of episodes
//    -c	Path to configuration file
//    -d	Download directory path

#[derive(Debug, StructOpt)]
pub struct Config {
	/// Download directory path
	#[structopt(short, long)]
	destination: PathBuf,
}

impl Config {
	pub fn destination(&self) -> &Path {
		&self.destination
	}
}
