use noise::{NoiseFn, Perlin, Seedable};

pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            tiles: vec![vec![TileType::Empty; width]; height],
            width,
            height,
        }
    }

    pub fn generate(&mut self) {
        let perlin = Perlin::new();
        perlin.set_seed(0);

        for y in 0..self.height {
            for x in 0..self.width {
                let noise_val = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
                self.tiles[y][x] = if noise_val > 0.0 { TileType::Wall } else { TileType::Floor };
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Empty,
    Floor,
    Wall,
}

