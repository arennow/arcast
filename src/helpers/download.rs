use super::progress_bars::TitledBar;
use crate::config::Config;
use crate::download::{download_to_file, DownloadError};
use crate::feed::Episode;
use derive_getters::Getters;
use std::boxed::Box;
use std::fmt::Display;
use std::io::Write;
use std::path::PathBuf;
use thiserror::Error as TError;

#[derive(TError, Debug, Getters)]
pub struct DownloadClientError {
	download_path: PathBuf,
	source: DownloadError,
}

impl DownloadClientError {
	fn new(source: DownloadError, download_path: PathBuf) -> Self {
		DownloadClientError {
			download_path,
			source,
		}
	}
}

impl Display for DownloadClientError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		Display::fmt(&self.source, f)
	}
}

pub fn download_episode(episode: &Episode, config: &Config) -> Result<(), DownloadClientError> {
	let mut file_dest_path = config.destination().to_path_buf();
	file_dest_path.push(episode.filename());

	#[allow(clippy::option_if_let_else)]
	let progress_function: Box<dyn FnMut(f64)> =
		if let Some(terminal_size) = terminal_size::terminal_size() {
			let mut stdout = termion::cursor::HideCursor::from(std::io::stdout());

			let terminal_width = terminal_size.0 .0;
			let label = episode.filename();

			let mut bar = TitledBar::new(label, terminal_width);

			Box::new(move |prog: f64| {
				bar.set(prog);
				let _ = write!(stdout, "\r{}", bar);
			})
		} else {
			Box::new(|_| {})
		};

	if let Err(e) = download_to_file(episode.enclosure_url(), &file_dest_path, progress_function) {
		let new_error = DownloadClientError::new(e, file_dest_path);
		return Err(new_error);
	}

	println!();

	Ok(())
}
