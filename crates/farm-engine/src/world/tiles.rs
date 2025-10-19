use crate::error::EngineError;
use crate::world::entities::{Entity, EntityType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GroundType {
    Grassland,
    Soil,
}

impl GroundType {
    pub fn can_have_entity(&self, entity: &Entity) -> bool {
        match self {
            GroundType::Grassland => [EntityType::Grass, EntityType::Bush, EntityType::Tree]
                .contains(entity.entity_type()),
            GroundType::Soil => {
                [EntityType::Carrot, EntityType::Pumpkin].contains(entity.entity_type())
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tile {
    x: usize,
    y: usize,
    ground_type: GroundType,
    entity: Option<Entity>,
}

impl Tile {
    pub fn ground_type(&self) -> &GroundType {
        &self.ground_type
    }
    pub fn set_ground_type(&mut self, ground_type: GroundType) -> &mut Self {
        self.ground_type = ground_type;
        self
    }
    pub fn entity(&self) -> Option<&Entity> {
        self.entity.as_ref()
    }
    pub fn set_entity(&mut self, entity: Option<Entity>) -> &mut Self {
        self.entity = entity;
        self
    }
}

pub struct TileBuilder {
    x: usize,
    y: usize,
    ground_type: Option<GroundType>,
    entity: Option<Entity>,
}

impl TileBuilder {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            ground_type: Default::default(),
            entity: Default::default(),
        }
    }

    pub fn ground_type(mut self, ground_type: GroundType) -> Self {
        self.ground_type = Some(ground_type);
        self
    }

    pub fn entity(mut self, entity: Option<Entity>) -> Self {
        self.entity = entity;
        self
    }

    pub fn build(self) -> Result<Tile, EngineError> {
        let ground_type = self.ground_type.ok_or(EngineError::TileWithoutGroundType)?;
        let entity = if let Some(entity) = self.entity {
            if !ground_type.can_have_entity(&entity) {
                return Err(EngineError::EntityNotAllowedOnGroundType(
                    entity.entity_type().clone(),
                    ground_type.clone(),
                ));
            }
            Some(entity)
        } else {
            None
        };

        Ok(Tile {
            x: self.x,
            y: self.y,
            ground_type,
            entity,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::world::entities::{EntityBuilder, EntityType};

    use super::*;

    #[test]
    fn can_create_tile() {
        let tile = TileBuilder::new(0, 0)
            .ground_type(GroundType::Grassland)
            .build()
            .unwrap();
        assert_eq!(tile.ground_type(), &GroundType::Grassland);
        assert_eq!(tile.entity(), None);
    }

    #[test]
    fn can_create_tile_with_entity() {
        let tile = TileBuilder::new(0, 0)
            .ground_type(GroundType::Grassland)
            .entity(Some(
                EntityBuilder::new()
                    .entity_type(EntityType::Grass)
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();
        assert_eq!(
            tile.entity(),
            Some(
                EntityBuilder::new()
                    .entity_type(EntityType::Grass)
                    .build()
                    .unwrap()
            )
            .as_ref()
        );
    }

    #[test]
    fn cannot_create_tile_with_entity_not_allowed_on_ground_type() {
        let tile = TileBuilder::new(0, 0)
            .ground_type(GroundType::Grassland)
            .entity(Some(
                EntityBuilder::new()
                    .entity_type(EntityType::Carrot)
                    .build()
                    .unwrap(),
            ))
            .build();
        assert!(matches!(
            tile.unwrap_err(),
            EngineError::EntityNotAllowedOnGroundType(EntityType::Carrot, GroundType::Grassland)
        ));
    }
}
