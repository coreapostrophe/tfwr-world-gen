use crate::engine::{
    error::EngineError,
    game::drone::Drone,
    world::{
        entities::{EntityBuilder, EntityType},
        tiles::GroundType,
        World,
    },
};

pub(crate) mod drone;

pub struct Game {
    world: World,
    drone: Drone,
}

impl Game {
    pub fn new(world: World, drone: Drone) -> Self {
        Self { world, drone }
    }

    pub fn get_world_size(&self) -> (usize, usize) {
        (self.world.width(), self.world.height())
    }

    pub fn plant(
        &mut self,
        x: usize,
        y: usize,
        entity_type: EntityType,
    ) -> Result<(), EngineError> {
        let tile = self.world.mut_tile(x, y)?;
        tile.set_entity(Some(EntityBuilder::new().entity_type(entity_type).build()?));
        Ok(())
    }

    pub fn harvest(&mut self, x: usize, y: usize) -> Result<(), EngineError> {
        let tile = self.world.mut_tile(x, y)?;
        tile.set_entity(None);
        Ok(())
    }

    pub fn get_entity_type(&self, x: usize, y: usize) -> Result<Option<&EntityType>, EngineError> {
        let tile = self.world.get_tile(x, y)?;
        if let Some(entity) = tile.entity() {
            Ok(Some(entity.entity_type()))
        } else {
            Ok(None)
        }
    }

    pub fn get_ground_type(&self, x: usize, y: usize) -> Result<&GroundType, EngineError> {
        let tile = self.world.get_tile(x, y)?;
        Ok(tile.ground_type())
    }

    pub fn till(&mut self, x: usize, y: usize) -> Result<(), EngineError> {
        let tile = self.world.mut_tile(x, y)?;
        tile.set_ground_type(GroundType::Soil);
        Ok(())
    }

    pub fn move_drone(&mut self, x: usize, y: usize) -> Result<(), EngineError> {
        if x >= self.world.width() || y >= self.world.height() {
            return Err(EngineError::DroneOutOfBounds { x, y });
        }
        self.drone.set_position(x, y);
        Ok(())
    }

    pub fn get_pos_x(&self) -> usize {
        self.drone.x()
    }

    pub fn get_pos_y(&self) -> usize {
        self.drone.y()
    }
}
