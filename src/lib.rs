use ansi_term::Colour::RGB;
use image::{ImageBuffer, Rgb};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::boxed::Box;
use std::fmt;
use std::path::Path;

const WIGGLE: i16 = 32;
const TILES: usize = 512;
const SIZE: usize = TILES * 2 + 1;

pub fn generate() {
    let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(SIZE as u32, SIZE as u32);

    let mut map: Map = Map::new();

    map.init();

    let steps = TILES.trailing_zeros() + 1;

    for s in 0..steps {
        map.squares(1 << s);
        map.diamonds(1 << (steps - s - 1));
    }

    for x in 0..SIZE {
        for y in 0..SIZE {
            img_buffer.put_pixel(x as u32, y as u32, Rgb([map.terrain[x][y] as u8, 13, 50]));
        }
    }

    img_buffer.save(Path::new("map.png")).unwrap();
}

struct Map {
    pub terrain: Box<[[u32; SIZE as usize]; SIZE as usize]>,
    rng: SmallRng,
}

impl Map {
    fn new() -> Map {
        Map {
            rng: SmallRng::from_entropy(),
            terrain: Box::new([[0; SIZE as usize]; SIZE as usize]),
        }
    }

    fn init(&mut self) {
        let a = self.rng.gen::<u8>() as u32;
        let b = self.rng.gen::<u8>() as u32;
        let c = self.rng.gen::<u8>() as u32;
        let d = self.rng.gen::<u8>() as u32;

        self.terrain[0][0] = a;
        self.terrain[SIZE - 1][0] = b;
        self.terrain[0][SIZE - 1] = c;
        self.terrain[SIZE - 1][SIZE - 1] = d;
    }

    fn square(&mut self, x: usize, y: usize, radius: usize) {
        let height = self.wiggle((
                self.terrain[x - radius][y - radius] +
                self.terrain[x - radius][y + radius] +
                self.terrain[x + radius][y - radius] +
                self.terrain[x + radius][y + radius] )
                / 4,
            WIGGLE,
        );

        self.terrain[x][y] = height;
    }

    fn diamond(&mut self, x: usize, y: usize, radius: usize) {
        let mut spread = 0;
        let mut t = 0;

        if radius <= x {
            spread += 1;
            t += self.terrain[x - radius][y];
        }

        if x + radius < SIZE {
            spread += 1;
            t += self.terrain[x + radius][y];
        }

        if radius <= y {
            spread += 1;
            t += self.terrain[x][y - radius];
        }

        if y + radius < SIZE {
            spread += 1;
            t += self.terrain[x][y + radius];
        }

        let height = self.wiggle(t / spread, WIGGLE);
        self.terrain[x][y] = height;
    }

    fn squares(&mut self, step: usize) {
        for x in 0..step {
            for y in 0..step {
                self.square(
                    SIZE / (step * 2) + (x * SIZE / step),
                    SIZE / (step * 2) + (y * SIZE / step),
                    SIZE / (step * 2),
                );
            }
        }
    }

    fn diamonds(&mut self, radius: usize) {
        for x in (0..SIZE).step_by(radius) {
            let start: usize = if (x / (radius)) % 2 == 0 { radius } else { 0 };

            for y in (start..SIZE).step_by(radius * 2) {
                self.diamond(x, y, radius);
            }
        }
    }

    fn wiggle(&mut self, value: u32, range: i16) -> u32 {
        let min = range.min(value as i16);

        (value as i16 + self.rng.gen_range(-min, range)) as u32
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..SIZE {
            for y in 0..SIZE {
                let color = self.terrain[x][y] as u8;
                let styled = RGB(color, 70, 130).paint("███");
                let _ = write!(f, "{}", styled);
            }
            let _ = write!(f, "\n");
        }
        write!(f, "")
    }
}
