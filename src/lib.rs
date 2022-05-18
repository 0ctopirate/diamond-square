use ansi_term::Colour::RGB;
use image::{ImageBuffer, Rgb};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::fmt;
use std::path::Path;

pub struct Map {
    pub terrain: Vec<Vec<u32>>,
    pub wiggle_range: i16,
    pub tiles: usize,
    pub size: usize
}

impl Map {
    pub fn new(wiggle_range: i16, tiles: usize) -> Self {
        let size: usize = tiles * 2 + 1;

        let mut terrain = vec![vec![0; size]; size];

        let mut rng = SmallRng::from_entropy();
        terrain[0][0]               = rng.gen::<u8>() as u32;
        terrain[size - 1][0]        = rng.gen::<u8>() as u32;
        terrain[0][size - 1]        = rng.gen::<u8>() as u32;
        terrain[size - 1][size - 1] = rng.gen::<u8>() as u32;

        Map {
            terrain,
            wiggle_range,
            tiles,
            size
        }
    }

    pub fn generate(&mut self) {
        let steps = self.tiles.trailing_zeros() + 1;

        for s in 0..steps {
            self.squares(1 << s);
            self.diamonds(1 << (steps - s - 1));
        }
    }

    pub fn save(&self) {
        let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(self.size as u32, self.size as u32);

        for x in 0..self.size {
            for y in 0..self.size {
                img_buffer.put_pixel(x as u32, y as u32, Rgb([self.terrain[x][y] as u8, 13, 50]));
            }
        }
    
        img_buffer.save(Path::new("map.png")).unwrap();
    }

    fn square(&mut self, x: usize, y: usize, radius: usize) {
        let height = self.wiggle((
                self.terrain[x - radius][y - radius] +
                self.terrain[x - radius][y + radius] +
                self.terrain[x + radius][y - radius] +
                self.terrain[x + radius][y + radius] )
                / 4,
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

        if x + radius < self.size {
            spread += 1;
            t += self.terrain[x + radius][y];
        }

        if radius <= y {
            spread += 1;
            t += self.terrain[x][y - radius];
        }

        if y + radius < self.size {
            spread += 1;
            t += self.terrain[x][y + radius];
        }

        let height = self.wiggle(t / spread);
        self.terrain[x][y] = height;
    }

    fn squares(&mut self, step: usize) {
        for x in 0..step {
            for y in 0..step {
                self.square(
                    self.size / (step * 2) + (x * self.size / step),
                    self.size / (step * 2) + (y * self.size / step),
                    self.size / (step * 2),
                );
            }
        }
    }

    fn diamonds(&mut self, radius: usize) {
        for x in (0..self.size).step_by(radius) {
            let start: usize = if (x / (radius)) % 2 == 0 { radius } else { 0 };

            for y in (start..self.size).step_by(radius * 2) {
                self.diamond(x, y, radius);
            }
        }
    }

    fn wiggle(&mut self, value: u32) -> u32 {
        let min = self.wiggle_range.min(value as i16);

        let mut rng = SmallRng::from_entropy();

        (value as i16 + rng.gen_range(-min, self.wiggle_range)) as u32
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                let color = self.terrain[x][y] as u8;
                let styled = RGB(color, 70, 130).paint("███");
                let _ = write!(f, "{}", styled);
            }
            let _ = write!(f, "\n");
        }
        write!(f, "")
    }
}
