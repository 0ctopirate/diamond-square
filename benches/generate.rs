#![feature(test)]

extern crate test;

extern crate diamond_square;
use diamond_square::Map;

const WIGGLE: i16 = 32;
const TILES: usize = 512;

fn generate() -> Map {
	let mut map = Map::new(WIGGLE, TILES);
	
	map.generate();

	map
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

	#[test]
	fn generate_produces_terrain() {
		let map = generate();
        assert_ne!(map.terrain[0], map.terrain[map.size]);
    }

    #[bench]
    fn bench_generate(b: &mut Bencher) {
        b.iter(|| generate());
    }
}