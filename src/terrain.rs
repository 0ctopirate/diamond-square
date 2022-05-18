use std::ops::{Index, IndexMut};

pub struct Terrain<T: Default> {
	pub map: Box<[T]>,
	pub width: usize,
	pub height: usize
}

impl<T: Default + Clone> Terrain<T> {
	pub fn new(width: usize, height: usize) -> Self {
		Terrain::<T> {
			map: vec![T::default(); width * height].into_boxed_slice(),
			width,
			height
		}
	}
}

impl<T: Default + Clone> Index<usize> for Terrain<T> {
	type Output = [T];

	fn index(&self, i: usize) -> &Self::Output {
		&self.map[(i * self.height)..(i * self.height + self.width)]
	}
}

impl<T: Default + Clone> IndexMut<usize> for Terrain<T> {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.map[(i * self.height)..(i * self.height + self.width)]
	}
}