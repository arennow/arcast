use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Cache<V> {
	inner: RefCell<Option<Rc<V>>>,
}

impl<V> Cache<V> {
	pub fn get<F: Fn() -> V>(&self, generator: F) -> Rc<V> {
		if let Some(existing) = &*self.inner.borrow() {
			return Rc::clone(existing);
		}

		let new = Rc::new(generator());
		self.inner.replace(Some(Rc::clone(&new)));

		new
	}
}

impl<V> Default for Cache<V> {
	fn default() -> Self {
		Self {
			inner: Default::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::cell::Cell;

	#[derive(Default)]
	struct InteriorlyMutableCounter(Cell<u32>);

	impl InteriorlyMutableCounter {
		fn increment(&self) {
			let mut val = self.0.get();
			val += 1;
			self.0.replace(val);
		}

		fn get(&self) -> u32 {
			self.0.get()
		}
	}

	#[test]
	fn test_single_generator_call() {
		let cached = Cache::<String>::default();
		let generator_count = InteriorlyMutableCounter::default();

		let getter = || {
			cached.get(|| {
				generator_count.increment();
				String::from("Foobar")
			})
		};
		let a = getter();
		let b = getter();
		let c = getter();

		assert_eq!(a, b);
		assert_eq!(b, c);
		assert_eq!(generator_count.get(), 1);
	}
}
