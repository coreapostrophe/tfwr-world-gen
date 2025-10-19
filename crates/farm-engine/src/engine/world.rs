use ndarray::Array2;

use crate::engine::error::EngineError;

pub mod tiles;

use tiles::{GroundType, Tile, TileBuilder};

pub struct World {
    width: usize,
    height: usize,
    tiles: Array2<Tile>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Result<Self, EngineError> {
        let tiles: Vec<Tile> = (0..width)
            .flat_map(|x| (0..height).map(move |y| (x, y)))
            .map(|(x, y)| {
                TileBuilder::new(x, y)
                    .ground_type(GroundType::Grass)
                    .build()
            })
            .collect::<Result<Vec<Tile>, EngineError>>()?;

        let tiles = Array2::from_shape_vec((width, height), tiles)?;

        Ok(Self {
            width,
            height,
            tiles,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tiles(&self) -> &Array2<Tile> {
        &self.tiles
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Result<&Tile, EngineError> {
        Ok(&self.tiles[(x, y)])
    }

    pub fn mut_tile(&mut self, x: usize, y: usize) -> Result<&mut Tile, EngineError> {
        Ok(&mut self.tiles[(x, y)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let world = World::new(10, 10).unwrap();
        assert_eq!(world.width(), 10);
        assert_eq!(world.height(), 10);
    }

    #[test]
    fn test_get_tile() {
        let world = World::new(10, 10).unwrap();
        let tile = world.get_tile(0, 0).unwrap();
        assert_eq!(tile.x(), 0);
        assert_eq!(tile.y(), 0);
        assert_eq!(tile.ground_type(), &GroundType::Grass);
    }
}
