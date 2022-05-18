extern crate diamond_square;

const WIGGLE: i16 = 32;
const TILES: usize = 512;

fn main() {
	let mut map = diamond_square::Map::new(WIGGLE, TILES);
	
	map.generate();

	map.save();
}