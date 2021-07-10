use crate::config::Config;
use crate::helpers;
use std::error::Error;

pub fn process_classified_episodes<'a>(
	episodes: impl Iterator<Item = helpers::ClassifiedEpisode<'a>>,
	config: &Config,
) -> Result<usize, Box<dyn Error>> {
	let mut missing_processed = 0;
	for classified_episode in episodes {
		let (status, episode) = classified_episode.take();
		if missing_processed >= config.number_to_download() {
			break;
		}

		match status {
			helpers::EpisodeStatus::Have => {
				if config.print_existing_episodes() {
					println!("{} already exists", episode.filename())
				}
			}
			helpers::EpisodeStatus::Need => {
				helpers::download_episode(episode, &config)?;
				missing_processed += 1;
			}
		}
	}

	Ok(missing_processed)
}
