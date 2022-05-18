use std::ops::{Index, IndexMut};

pub struct Terrain {
	pub map: Box<[u32]>,
	pub width: usize,
	pub height: usize
}

impl Terrain {
	pub fn new(width: usize, height: usize) -> Self {
		Terrain {
			map: vec![0; width * height].into_boxed_slice(),
			width,
			height
		}
	}
}

impl Index<usize> for Terrain {
	type Output = [u32];

	fn index(&self, i: usize) -> &Self::Output {
		&self.map[(i * self.height)..(i * self.height + self.width)]
	}
}

impl IndexMut<usize> for Terrain {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.map[(i * self.height)..(i * self.height + self.width)]
	}
}