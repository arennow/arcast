#![warn(clippy::all)]
#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::map_flatten)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::redundant_closure_for_method_calls)]
#![warn(clippy::inconsistent_struct_constructor)]
#![warn(clippy::option_if_let_else)]

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate lazy_static;

mod cache;
mod config;
mod download;
mod feed;
mod filesystem;
mod helpers;

use clap::Parser;

fn do_work() -> Result<(), Box<dyn std::error::Error>> {
	let config = config::Config::parse();
	let show: feed::Show = {
		let config_file_path_string = config.config_file_path().to_string_lossy();
		let config_file_handle =
			filesystem::FilesystemError::handling_io_error_in(config_file_path_string, || {
				std::fs::File::open(config.config_file_path())
			})?;

		serde_json::from_reader(config_file_handle)
	}?;

	let (reader, _) = download::download_to_reader(show.url())?;
	let episodes = feed::episodes_from_reader(reader, &show)?;
	let classified_eps = helpers::classified_episodes(&show, &episodes, &config)?;

	helpers::process_classified_episodes(classified_eps, &config)?;

	if config.prune_renamed_duplicates() {
		let canonical_filenames: std::collections::HashSet<String> =
			episodes.iter().map(|e| e.filename().to_owned()).collect();
		let to_prune = helpers::find_files_to_prune(config.destination(), &canonical_filenames)?;
		for path in to_prune {
			if config.pretend() {
				println!("{} would be removed (renamed duplicate)", path.display());
			} else {
				filesystem::FilesystemError::handling_io_error_in(path.to_string_lossy(), || {
					std::fs::remove_file(&path)
				})?;
			}
		}
	}

	Ok(())
}

fn main() {
	if let Err(e) = do_work() {
		eprintln!("Error: {}", e);
	}
}
