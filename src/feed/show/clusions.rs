#[derive(Debug, Clone)]
pub enum Clusions<T> {
	Inclusion(Vec<T>),
	Exclusion(Vec<T>),
}

impl<T> Clusions<T> {
	pub fn map<R, F>(&self, f: F) -> Clusions<R>
	where
		F: Fn(&Vec<T>) -> Vec<R>,
	{
		use Clusions::*;
		match self {
			Inclusion(v) => Inclusion(f(v)),
			Exclusion(v) => Exclusion(f(v)),
		}
	}
}
