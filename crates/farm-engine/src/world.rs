use ndarray::Array2;
use std::collections::HashMap;

use crate::error::EngineError;

pub mod entities;
pub mod tiles;

use entities::{Entity, EntityType};
use tiles::{GroundType, Tile, TileBuilder, TileId};

pub struct World {
    width: usize,
    height: usize,
    tile_refs: Array2<Option<TileId>>,
    tiles: HashMap<TileId, Tile>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Result<Self, EngineError> {
        let mut tile_refs = Array2::from_elem((width, height), None);

        let mut tiles = HashMap::new();
        let mut next_tile_id = 0;

        for x in 0..width {
            for y in 0..height {
                let tile = TileBuilder::new(next_tile_id)
                    .ground_type(GroundType::Grassland)
                    .entity(None)
                    .build()?;

                tiles.insert(next_tile_id, tile);
                tile_refs[(x, y)] = Some(next_tile_id);
                next_tile_id += 1;
            }
        }

        Ok(Self {
            width,
            height,
            tile_refs,
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
        if x >= self.width || y >= self.height {
            return Err(EngineError::DroneOutOfBounds { x, y });
        }

        let tile_id = self.tile_refs[(x, y)].ok_or(EngineError::DroneOutOfBounds { x, y })?;

        self.tiles
            .get(&tile_id)
            .ok_or(EngineError::DroneOutOfBounds { x, y })
    }

    pub fn mut_tile(&mut self, x: usize, y: usize) -> Result<&mut Tile, EngineError> {
        if x >= self.width || y >= self.height {
            return Err(EngineError::DroneOutOfBounds { x, y });
        }

        let tile_id = self.tile_refs[(x, y)].ok_or(EngineError::DroneOutOfBounds { x, y })?;

        self.tiles
            .get_mut(&tile_id)
            .ok_or(EngineError::DroneOutOfBounds { x, y })
    }

    pub fn clear(&mut self) -> Result<(), EngineError> {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(tile_id) = self.tile_refs[(x, y)] {
                    if let Some(tile) = self.tiles.get_mut(&tile_id) {
                        tile.set_ground_type(GroundType::Grassland);
                        tile.set_entity(Some(Entity::from(EntityType::Grass)))?;
                    }
                }
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

        for x in 0..world.width() {
            for y in 0..world.height() {
                let tile = world.get_tile(x, y).unwrap();
                assert_eq!(tile.ground_type(), &GroundType::Grassland);
                assert!(tile.entity().is_some());
                assert_eq!(tile.entity().unwrap().entity_type(), &EntityType::Grass);
            }
        }
    }
}
