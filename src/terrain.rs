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

impl<T: Default + Copy + Into<f64>> Terrain<T> {
	pub fn normalize(self) -> Terrain<f64> {
		let cast_map: Vec<f64> = self.map.iter().map(|&e| e.into()).collect();
		let min = cast_map.iter().fold(f64::INFINITY, |a, &b| a.min(b));
		let max = cast_map.iter().fold(f64::INFINITY, |a, &b| a.max(b));
		let diff = max - min;

		Terrain::<f64> {
			map: cast_map.iter().map(|&e| (e - min) * diff).collect::<Vec<f64>>().into_boxed_slice(),
			width: self.width,
			height: self.height
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