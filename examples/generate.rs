// use ansi_term::Colour::RGB;
use image::{ImageBuffer, Rgb};
// use std::fmt;
use std::path::Path;

extern crate diamond_square;
use diamond_square::Map;

const WIGGLE: i16 = 32;
const TILES: usize = 512;

// impl fmt::Display for Map {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for x in 0..self.size {
//             for y in 0..self.size {
//                 let color = self.terrain[x][y] as u8;
//                 let styled = RGB(color, 70, 130).paint("███");
//                 let _ = write!(f "{}", styled);
//             }
//             let _ = write!(f, "\n");
//         }
//         write!(f, "")
//     }
// }

fn main() {
	let mut map = Map::new(WIGGLE, TILES);
	
	map.generate();

	save_map_as_image(map);
}

fn save_map_as_image(map: Map) {
	let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(map.size as u32, map.size as u32);

	for x in 0..map.size {
		for y in 0..map.size {
			img_buffer.put_pixel(x as u32, y as u32, Rgb([map.terrain[x][y] as u8, 13, 50]));
		}
	}

	img_buffer.save(Path::new("map.png")).unwrap();
}