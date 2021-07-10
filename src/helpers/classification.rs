use crate::config::Config;
use crate::feed::Episode;
use crate::filesystem;
use std::collections::HashSet;

pub enum EpisodeStatus {
	Need,
	Have,
}

pub struct ClassifiedEpisode<'a> {
	status: EpisodeStatus,
	episode: &'a Episode,
}

impl<'a> ClassifiedEpisode<'a> {
	pub fn take(self) -> (EpisodeStatus, &'a Episode) {
		(self.status, self.episode)
	}
}

fn classified_episodes_from_set(
	all_episodes: &[Episode],
	existing_files: HashSet<String>,
) -> impl Iterator<Item = ClassifiedEpisode> {
	all_episodes
		.iter()
		.map(move |episode| {
			use EpisodeStatus::*;
			let contains = existing_files.contains(&*episode.filename());
			let status = if contains { Have } else { Need };

			ClassifiedEpisode { status, episode }
		})
		.rev()
}

pub fn classified_episodes<'a>(
	all_episodes: &'a [Episode],
	config: &Config,
) -> Result<impl Iterator<Item = ClassifiedEpisode<'a>>, filesystem::FilesystemError> {
	let existing_files = filesystem::list_files(config.destination())?;
	let filtered_eps = classified_episodes_from_set(all_episodes, existing_files);

	Ok(filtered_eps)
}
