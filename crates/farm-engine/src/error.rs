use thiserror::Error;

use crate::world::entities::EntityType;
use crate::world::tiles::GroundType;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Tile does not have a ground type. Please set it using the `ground_type` method.")]
    TileWithoutGroundType,
    #[error("Failed to create world. {0}")]
    FailedToCreateWorld(#[from] ndarray::ShapeError),
    #[error("Entity does not have a type. Please set it using the `entity_type` method.")]
    EntityWithoutType,
    #[error("Entity {0:#?} is not allowed on this ground type. {1:#?}")]
    EntityNotAllowedOnGroundType(EntityType, GroundType),
    #[error("Drone is out of bounds. ({x}, {y})")]
    DroneOutOfBounds { x: usize, y: usize },
}
