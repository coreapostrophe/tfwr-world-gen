use ndarray::Array2;

use crate::error::EngineError;

pub mod entities;
pub mod tiles;

use entities::{Entity, EntityType};
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
                    .ground_type(GroundType::Grassland)
                    .entity(None)
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

    pub fn get_tile(&self, x: usize, y: usize) -> Result<&Tile, EngineError> {
        Ok(&self.tiles[(x, y)])
    }

    pub fn mut_tile(&mut self, x: usize, y: usize) -> Result<&mut Tile, EngineError> {
        Ok(&mut self.tiles[(x, y)])
    }

    pub fn clear(&mut self) -> Result<(), EngineError> {
        for row in 0..self.width {
            for column in 0..self.height {
                let tile = &mut self.tiles[(row, column)];
                tile.set_ground_type(GroundType::Grassland);
                tile.set_entity(Some(Entity::from(EntityType::Grass)))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_world() {
        let world = World::new(10, 10).unwrap();
        assert_eq!(world.width(), 10);
        assert_eq!(world.height(), 10);
    }

    #[test]
    fn can_get_tile() {
        let world = World::new(10, 10).unwrap();
        let tile = world.get_tile(0, 0).unwrap();
        assert_eq!(tile.ground_type(), &GroundType::Grassland);
    }

    #[test]
    fn clear_resets_all_tiles_to_grassland_with_grass() {
        let mut world = World::new(5, 5).unwrap();

        world
            .mut_tile(1, 1)
            .unwrap()
            .set_ground_type(GroundType::Soil);
        world
            .mut_tile(1, 1)
            .unwrap()
            .set_entity(Some(Entity::from(EntityType::Carrot)))
            .unwrap();

        world
            .mut_tile(2, 3)
            .unwrap()
            .set_ground_type(GroundType::Soil);
        world
            .mut_tile(2, 3)
            .unwrap()
            .set_entity(Some(Entity::from(EntityType::Pumpkin)))
            .unwrap();

        world
            .mut_tile(4, 4)
            .unwrap()
            .set_entity(Some(Entity::from(EntityType::Tree)))
            .unwrap();

        world.clear().unwrap();

        for row in 0..world.width() {
            for column in 0..world.height() {
                let tile = world.get_tile(row, column).unwrap();
                assert_eq!(tile.ground_type(), &GroundType::Grassland);
                assert!(tile.entity().is_some());
                assert_eq!(tile.entity().unwrap().entity_type(), &EntityType::Grass);
            }
        }
    }

    #[test]
    fn clear_preserves_tile_coordinates() {
        let mut world = World::new(3, 3).unwrap();

        world
            .mut_tile(0, 0)
            .unwrap()
            .set_ground_type(GroundType::Soil);
        world
            .mut_tile(1, 2)
            .unwrap()
            .set_ground_type(GroundType::Soil);

        world.clear().unwrap();

        for row in 0..world.width() {
            for column in 0..world.height() {
                let tile = world.get_tile(row, column).unwrap();
                assert_eq!(tile.ground_type(), &GroundType::Grassland);
                assert_eq!(tile.entity().unwrap().entity_type(), &EntityType::Grass);
            }
        }
    }
}
