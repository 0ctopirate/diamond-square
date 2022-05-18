use ansi_term::Colour::RGB;
use std::fmt;

extern crate diamond_square;
use diamond_square::Map;

const WIGGLE: i16 = 32;
const TILES: usize = 512;

struct PrintableMap(Map);

impl fmt::Display for PrintableMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..self.0.size {
            for y in 0..self.0.size {
                let color = self.0.terrain[x][y] as u8;
                let styled = RGB(color, 70, 130).paint("███");
                let _ = write!(f, "{}", styled);
            }
            let _ = write!(f, "\n");
        }
        write!(f, "")
    }
}

fn main() {
	let mut map = Map::new(WIGGLE, TILES);
	
	map.generate();

	println!("{}", PrintableMap(map));
}