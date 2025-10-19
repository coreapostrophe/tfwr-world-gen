use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Tile does not have a ground type. Please set it using the `ground_type` method.")]
    TileWithoutGroundType,
    #[error("Failed to create world. {0}")]
    FailedToCreateWorld(#[from] ndarray::ShapeError),
}
