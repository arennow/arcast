use crate::config::Config;
use crate::download::download_to_file;
use crate::feed::Episode;
use crate::filesystem;
use std::collections::HashSet;
use std::error::Error;
use std::io::Write;

fn missing_episodes_from_set(
	all_episodes: &[Episode],
	existing_files: HashSet<String>,
) -> impl DoubleEndedIterator<Item = &Episode> {
	all_episodes
		.iter()
		.filter(move |e| !existing_files.contains(&e.filename()))
}

pub fn missing_episodes<'a>(
	all_episodes: &'a [Episode],
	config: &Config,
) -> std::io::Result<impl DoubleEndedIterator<Item = &'a Episode>> {
	let existing_files = filesystem::list_files(config.destination())?;
	let filtered_eps = missing_episodes_from_set(all_episodes, existing_files);

	Ok(filtered_eps)
}

pub fn download_episode(episode: &Episode, config: &Config) -> Result<(), Box<dyn Error>> {
	let mut file_dest_path = config.destination().to_path_buf();
	file_dest_path.push(episode.filename());

	print!("Will download {}…", episode.filename());
	std::io::stdout().flush()?;
	download_to_file(&episode.enclosure_url(), file_dest_path)?;
	println!(" Finished!");

	Ok(())
}
