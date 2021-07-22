use progressing::clamping::Bar;
use progressing::Baring;

struct BarSizes {
	title: u16,
	bar: u16,
}

impl BarSizes {
	fn percent() -> u16 {
		5
	}
}

pub struct TitledBar {
	title: String,
	bar: Bar,
	sizes: BarSizes,
}

impl TitledBar {
	pub fn new<S: Into<String>>(title: S, width: u16) -> Self {
		let title = title.into();
		let sizes = Self::sizes(&title, width);

		let mut bar = Bar::new();
		bar.set_style("[##-]");
		bar.set_len(sizes.bar as usize);

		Self { title, bar, sizes }
	}

	pub fn set<P: Into<f64>>(&mut self, prog: P) {
		self.bar.set(prog)
	}

	fn sizes(title: &str, full_width: u16) -> BarSizes {
		let title = Self::width_of_title(title, full_width);
		let bar = Self::width_of_bar(title, full_width);

		BarSizes { title, bar }
	}

	fn width_of_title(title: &str, full_width: u16) -> u16 {
		let title_len = title.len() as f32 as u16; // Casting through float causes a saturating cast

		title_len.min(full_width / 2)
	}

	fn width_of_bar(title_width: u16, full_width: u16) -> u16 {
		assert!(full_width > title_width);

		full_width - title_width - 1 - BarSizes::percent()
	}

	fn percent_string(&self) -> String {
		let int_percent = (self.bar.progress() * 100.0) as i32;
		let string = format!("{:3}%", int_percent);

		assert!(string.len() < BarSizes::percent() as usize);

		string
	}
}

impl std::fmt::Display for TitledBar {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		let title_width = self.sizes.title as usize;
		let sub_title = &self.title[..title_width];
		let percent = self.percent_string();

		write!(f, "{} {} {}", sub_title, self.bar, percent)
	}
}
