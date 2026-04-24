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

	/// Pretend (don't download or delete anything)
	#[arg(short, long)]
	pretend: bool,

	/// Print existing episodes
	#[arg(short = 'e', long)]
	print_existing_episodes: bool,

	/// Limit number of episodes
	#[arg(short, long)]
	number_to_download: Option<usize>,

	/// Remove files in the destination directory that are byte-for-byte duplicates of a
	/// current-feed episode but whose filename is not what the feed would produce today.
	/// (Respects --pretend)
	// The motivating case: an episode originally downloaded as "X" that was later renamed
	// to "Y" in the feed. Running with this flag removes the stale "X" copy.
	#[arg(long)]
	prune_renamed_duplicates: bool,
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

	pub fn prune_renamed_duplicates(&self) -> bool {
		self.prune_renamed_duplicates
	}
}
