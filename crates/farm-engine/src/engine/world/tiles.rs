use crate::engine::error::EngineError;

#[derive(Debug, PartialEq, Eq)]
pub enum GroundType {
    Grass,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tile {
    x: usize,
    y: usize,
    ground_type: GroundType,
}

impl Tile {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn ground_type(&self) -> &GroundType {
        &self.ground_type
    }
}

pub struct TileBuilder {
    x: usize,
    y: usize,
    ground_type: Option<GroundType>,
}

impl TileBuilder {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            ground_type: None,
        }
    }

    pub fn ground_type(mut self, ground_type: GroundType) -> Self {
        self.ground_type = Some(ground_type);
        self
    }

    pub fn build(self) -> Result<Tile, EngineError> {
        Ok(Tile {
            x: self.x,
            y: self.y,
            ground_type: self.ground_type.ok_or(EngineError::TileWithoutGroundType)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tile = TileBuilder::new(0, 0).build().unwrap();
        assert_eq!(tile.x(), 0);
        assert_eq!(tile.y(), 0);
        assert_eq!(tile.ground_type(), &GroundType::Grass);
    }
}
