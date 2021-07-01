mod download;

fn main() {
	let mut args = std::env::args().skip(1);
	let source_url = args.next().unwrap();
	let dest_path = args.next().unwrap();

	if let Err(e) = download::download_to_file(&source_url, dest_path) {
		eprintln!("Error: {}", e);
	}
}
