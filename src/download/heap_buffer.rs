use std::ops::{Deref, DerefMut};

pub struct HeapBuffer<const S: usize> {
	storage: Vec<u8>,
}

impl<const S: usize> HeapBuffer<S> {
	pub fn new() -> Self {
		HeapBuffer {
			storage: vec![0; S],
		}
	}
}

impl<const S: usize> Default for HeapBuffer<S> {
	fn default() -> Self {
		Self::new()
	}
}

impl<const S: usize> Deref for HeapBuffer<S> {
	type Target = [u8];

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.storage[..]
	}
}

impl<const S: usize> DerefMut for HeapBuffer<S> {
	fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
		&mut self.storage[..]
	}
}
