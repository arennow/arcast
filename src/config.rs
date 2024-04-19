use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Config {
	/// Download directory path
	#[arg(short, long)]
	destination: PathBuf,

	/// Path to configuration file
	#[arg(short, long)]
	config_file_path: PathBuf,

	/// Pretend (don't download anything)
	#[arg(short, long)]
	pretend: bool,

	/// Print existing episodes
	#[arg(short = 'e', long)]
	print_existing_episodes: bool,

	/// Limit number of episodes
	#[arg(short, long)]
	number_to_download: Option<usize>,
}

impl Config {
	pub fn destination(&self) -> &Path {
		&self.destination
	}

	pub fn config_file_path(&self) -> &Path {
		&self.config_file_path
	}

	pub fn pretend(&self) -> bool {
		self.pretend
	}

	pub fn print_existing_episodes(&self) -> bool {
		self.print_existing_episodes
	}

	pub fn number_to_download(&self) -> usize {
		self.number_to_download.unwrap_or(usize::MAX)
	}
}
