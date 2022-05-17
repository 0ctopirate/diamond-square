use ansi_term::Colour::RGB;
use image::{ImageBuffer, Rgb};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::boxed::Box;
use std::fmt;
use std::path::Path;

const WIGGLE: i16 = 32;
const TILES: u32 = 512;
const SIZE: u32 = TILES * 2 + 1;
const MSIZE: usize = (SIZE * SIZE) as usize;

pub fn generate() {
    let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(SIZE, SIZE);

    let mut map: Map = Map::new();

    map.init();

    let steps = TILES.trailing_zeros() + 1;

    for s in 0..steps {
        map.squares(1 << s);
        map.diamonds(1 << (steps - s - 1));
    }

    for x in 0..SIZE {
        for y in 0..SIZE {
            img_buffer.put_pixel(x, y, Rgb([map.get(x, y) as u8, 13, 50]));
        }
    }

    img_buffer.save(Path::new("map.png")).unwrap();
}

struct Map {
    rng: SmallRng,
    terrain: Box<[u32; MSIZE]>,
}

impl Map {
    fn new() -> Map {
        Map {
            rng: SmallRng::from_entropy(),
            terrain: Box::new([0; MSIZE]),
        }
    }

    fn get(&self, x: u32, y: u32) -> u32 {
        self.terrain[(x * SIZE + y) as usize]
    }

    fn set(&mut self, x: u32, y: u32, v: u32) {
        self.terrain[(x * SIZE + y) as usize] = v;
    }

    fn init(&mut self) {
        let a = self.rng.gen::<u8>() as u32;
        let b = self.rng.gen::<u8>() as u32;
        let c = self.rng.gen::<u8>() as u32;
        let d = self.rng.gen::<u8>() as u32;

        self.set(0, 0, a);
        self.set(SIZE - 1, 0, b);
        self.set(0, SIZE - 1, c);
        self.set(SIZE - 1, SIZE - 1, d);
    }

    fn square(&mut self, x: u32, y: u32, radius: u32) {
        let height = self.wiggle(
            (self.get(x - radius, y - radius)
                + self.get(x - radius, y + radius)
                + self.get(x + radius, y - radius)
                + self.get(x + radius, y + radius))
                / 4,
            WIGGLE,
        );

        self.set(x, y, height);
    }

    fn diamond(&mut self, x: u32, y: u32, radius: u32) {
        let mut spread = 0;
        let mut t = 0;

        if radius <= x {
            spread += 1;
            t += self.get(x - radius, y);
        }

        if x + radius < SIZE {
            spread += 1;
            t += self.get(x + radius, y);
        }

        if radius <= y {
            spread += 1;
            t += self.get(x, y - radius);
        }

        if y + radius < SIZE {
            spread += 1;
            t += self.get(x, y + radius);
        }

        let height = self.wiggle(t / spread, WIGGLE);
        self.set(x, y, height);
    }

    fn squares(&mut self, step: u32) {
        let step2 = step * 2;
        for x in 0..step {
            for y in 0..step {
                self.square(
                    SIZE / step2 + (x * SIZE / step),
                    SIZE / step2 + (y * SIZE / step),
                    SIZE / step2,
                );
            }
        }
    }

    fn diamonds(&mut self, radius: u32) {
        for x in (0..SIZE).step_by(radius as usize) {
            let y_start: u32 = if (x / (radius)) % 2 == 0 { radius } else { 0 };

            for y in (y_start..SIZE).step_by(radius as usize * 2) {
                self.diamond(x, y, radius);
            }
        }
    }

    fn wiggle(&mut self, value: u32, range: i16) -> u32 {
        let min = if value < range as u32 {
            value as i16
        } else {
            range
        };

        (value as i16 + self.rng.gen_range(-min, range)) as u32
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..SIZE {
            for y in 0..SIZE {
                let color = self.get(x, y) as u8;
                let styled = RGB(color, 70, 130).paint("███");
                let _ = write!(f, "{}", styled);
            }
            let _ = write!(f, "\n");
        }
        write!(f, "")
    }
}
