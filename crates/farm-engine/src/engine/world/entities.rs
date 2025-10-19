use crate::engine::error::EngineError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntityType {
    Grass,
    Bush,
    Carrot,
    Pumpkin,
    Tree,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entity {
    entity_type: EntityType,
}

impl Entity {
    pub fn entity_type(&self) -> &EntityType {
        &self.entity_type
    }
    pub fn set_entity_type(&mut self, entity_type: EntityType) -> &mut Self {
        self.entity_type = entity_type;
        self
    }
}

#[derive(Default)]
pub struct EntityBuilder {
    entity_type: Option<EntityType>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entity_type(mut self, entity_type: EntityType) -> Self {
        self.entity_type = Some(entity_type);
        self
    }

    pub fn build(self) -> Result<Entity, EngineError> {
        Ok(Entity {
            entity_type: self.entity_type.ok_or(EngineError::EntityWithoutType)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_entity() {
        let entity = EntityBuilder::new()
            .entity_type(EntityType::Grass)
            .build()
            .unwrap();
        assert_eq!(entity.entity_type(), &EntityType::Grass);
    }
}
