use super::progress_bars::TitledBar;
use crate::config::Config;
use crate::download::download_to_file;
use crate::feed::Episode;
use std::boxed::Box;
use std::error::Error;
use std::io::Write;

pub fn download_episode(episode: &Episode, config: &Config) -> Result<(), Box<dyn Error>> {
	let mut file_dest_path = config.destination().to_path_buf();
	file_dest_path.push(&*episode.filename());

	let progress_function: Box<dyn FnMut(f64)> =
		if let Some(terminal_size) = terminal_size::terminal_size() {
			let mut stdout = termion::cursor::HideCursor::from(std::io::stdout());

			let terminal_width = terminal_size.0 .0;
			let label = episode.title().clone();

			let mut bar = TitledBar::new(label, terminal_width);

			Box::new(move |prog: f64| {
				bar.set(prog);
				let _ = write!(stdout, "\r{}", bar);
			})
		} else {
			Box::new(|_| {})
		};

	download_to_file(&episode.enclosure_url(), file_dest_path, progress_function)?;

	println!();

	Ok(())
}
