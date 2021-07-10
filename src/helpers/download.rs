use crate::config::Config;
use crate::download::download_to_file;
use crate::feed::Episode;
use std::error::Error;
use std::io::Write;

pub fn download_episode(episode: &Episode, config: &Config) -> Result<(), Box<dyn Error>> {
	let mut file_dest_path = config.destination().to_path_buf();
	file_dest_path.push(&*episode.filename());

	print!("{}… ", episode.filename());
	std::io::stdout().flush()?;
	download_to_file(&episode.enclosure_url(), file_dest_path)?;
	println!("Finished!");

	Ok(())
}
