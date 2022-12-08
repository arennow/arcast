use crate::config::Config;
use crate::feed::{Clusions, Episode, Show};
use crate::filesystem;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum EpisodeStatus {
	Need,
	Have,
	ShouldSkip,
}

#[derive(Debug)]
pub struct ClassifiedEpisode<'a> {
	status: EpisodeStatus,
	episode: &'a Episode,
}

impl<'a> ClassifiedEpisode<'a> {
	pub fn take(self) -> (EpisodeStatus, &'a Episode) {
		(self.status, self.episode)
	}
}

fn any_match(regexes: &[Regex], string: &str) -> bool {
	for r in regexes {
		if r.is_match(string) {
			return true;
		}
	}

	false
}

fn classified_episodes_from_set<'a>(
	show: &Show,
	all_episodes: &'a [Episode],
	existing_files: HashSet<String>,
) -> impl Iterator<Item = ClassifiedEpisode<'a>> {
	let clusions = show.regex_container().clusions().clone();

	let show_nb4d8 = show.not_before_date();

	all_episodes.iter().rev().map(move |episode| {
		use Clusions::*;
		use EpisodeStatus::*;

		let get_status = || -> EpisodeStatus {
			if let Some(clusions) = &clusions {
				match clusions {
					Inclusion(regs) => {
						if !any_match(regs, episode.episode_name()) {
							return ShouldSkip;
						}
					}
					Exclusion(regs) => {
						if any_match(regs, episode.episode_name()) {
							return ShouldSkip;
						}
					}
				}
			}

			let already_have = existing_files.contains(episode.filename());
			if already_have {
				Have
			} else {
				Need
			}
		};

		let mut status = get_status();

		if status == EpisodeStatus::Need
			&& matches!(show_nb4d8, Some(not_before_date) if not_before_date > episode.pub_date())
		{
			status = EpisodeStatus::ShouldSkip;
		}

		ClassifiedEpisode { status, episode }
	})
}

pub fn classified_episodes<'a>(
	show: &Show,
	all_episodes: &'a [Episode],
	config: &Config,
) -> Result<impl Iterator<Item = ClassifiedEpisode<'a>>, filesystem::FilesystemError> {
	let existing_files = filesystem::list_files(config.destination())?;
	let filtered_eps = classified_episodes_from_set(show, all_episodes, existing_files);

	Ok(filtered_eps)
}
