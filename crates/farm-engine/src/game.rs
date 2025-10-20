use crate::{
    error::EngineError,
    game::drone::Drone,
    world::{entities::EntityType, tiles::GroundType, World},
};

pub(crate) mod drone;

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct GameOptions {
    pub world_width: usize,
    pub world_height: usize,
}

pub struct Game {
    world: World,
    drone: Drone,
}

impl Game {
    pub fn new(options: GameOptions) -> Self {
        let world = World::new(options.world_width, options.world_height).unwrap();
        let drone = Drone::new(0, 0);
        Self { world, drone }
    }

    pub fn get_world_size(&self) -> (usize, usize) {
        (self.world.width(), self.world.height())
    }

    pub fn plant(&mut self, entity_type: EntityType) -> Result<(), EngineError> {
        let x = self.drone.x();
        let y = self.drone.y();
        let tile = self.world.mut_tile(x, y)?;
        tile.set_entity(Some(entity_type.into()))?;
        Ok(())
    }

    pub fn harvest(&mut self) -> Result<(), EngineError> {
        let x = self.drone.x();
        let y = self.drone.y();
        if self.can_harvest()? {
            let tile = self.world.mut_tile(x, y)?;
            tile.set_entity(None)?;
            Ok(())
        } else {
            Err(EngineError::EntityNotGrown)
        }
    }

    pub fn can_harvest(&self) -> Result<bool, EngineError> {
        let x = self.drone.x();
        let y = self.drone.y();
        let tile = self.world.get_tile(x, y)?;
        if let Some(entity) = tile.entity() {
            Ok(entity.is_grown())
        } else {
            Ok(false)
        }
    }

    pub fn get_entity_type(&self) -> Result<Option<&EntityType>, EngineError> {
        let x = self.drone.x();
        let y = self.drone.y();
        let tile = self.world.get_tile(x, y)?;
        if let Some(entity) = tile.entity() {
            Ok(Some(entity.entity_type()))
        } else {
            Ok(None)
        }
    }

    pub fn get_ground_type(&self) -> Result<&GroundType, EngineError> {
        let x = self.drone.x();
        let y = self.drone.y();
        let tile = self.world.get_tile(x, y)?;
        Ok(tile.ground_type())
    }

    pub fn till(&mut self) -> Result<(), EngineError> {
        let x = self.drone.x();
        let y = self.drone.y();
        let tile = self.world.mut_tile(x, y)?;
        if tile.ground_type() == &GroundType::Grassland {
            tile.set_ground_type(GroundType::Soil);
        } else if tile.ground_type() == &GroundType::Soil {
            tile.set_ground_type(GroundType::Grassland);
        }
        Ok(())
    }

    pub fn move_drone(&mut self, direction: Direction) -> Result<(), EngineError> {
        let width = self.world.width();
        let height = self.world.height();

        let (x, y) = match direction {
            Direction::North => {
                let new_y = if self.drone.y() == 0 {
                    height - 1
                } else {
                    self.drone.y() - 1
                };
                (self.drone.x(), new_y)
            }
            Direction::East => {
                let new_x = (self.drone.x() + 1) % width;
                (new_x, self.drone.y())
            }
            Direction::South => {
                let new_y = (self.drone.y() + 1) % height;
                (self.drone.x(), new_y)
            }
            Direction::West => {
                let new_x = if self.drone.x() == 0 {
                    width - 1
                } else {
                    self.drone.x() - 1
                };
                (new_x, self.drone.y())
            }
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game() -> Game {
        Game::new(GameOptions {
            world_width: 5,
            world_height: 5,
        })
    }

    #[test]
    fn can_create_game_with_correct_initial_state() {
        let game = create_test_game();
        assert_eq!(game.get_world_size(), (5, 5));
        assert_eq!(game.get_pos_x(), 0);
        assert_eq!(game.get_pos_y(), 0);
    }

    #[test]
    fn can_get_world_dimensions() {
        let game = Game::new(GameOptions {
            world_width: 10,
            world_height: 15,
        });
        assert_eq!(game.get_world_size(), (10, 15));
    }

    #[test]
    fn can_plant_different_entity_types() {
        let mut game = create_test_game();

        game.plant(EntityType::Grass).unwrap();
        let entity_type = game.get_entity_type().unwrap();
        assert_eq!(entity_type, Some(&EntityType::Grass));

        game.move_drone(Direction::East).unwrap();
        game.plant(EntityType::Bush).unwrap();
        assert_eq!(game.get_entity_type().unwrap(), Some(&EntityType::Bush));

        game.move_drone(Direction::East).unwrap();
        game.till().unwrap();
        game.plant(EntityType::Carrot).unwrap();
        assert_eq!(game.get_entity_type().unwrap(), Some(&EntityType::Carrot));

        game.move_drone(Direction::East).unwrap();
        game.till().unwrap();
        game.plant(EntityType::Pumpkin).unwrap();
        assert_eq!(game.get_entity_type().unwrap(), Some(&EntityType::Pumpkin));
    }

    #[test]
    fn cannot_plant_carrot_on_grassland() {
        let mut game = create_test_game();

        let result = game.plant(EntityType::Carrot);
        assert!(matches!(
            result.unwrap_err(),
            EngineError::EntityNotAllowedOnGroundType(EntityType::Carrot, GroundType::Grassland)
        ));
    }

    #[test]
    fn cannot_plant_pumpkin_on_grassland() {
        let mut game = create_test_game();

        let result = game.plant(EntityType::Pumpkin);
        assert!(matches!(
            result.unwrap_err(),
            EngineError::EntityNotAllowedOnGroundType(EntityType::Pumpkin, GroundType::Grassland)
        ));
    }

    #[test]
    fn cannot_plant_grass_on_soil() {
        let mut game = create_test_game();

        game.till().unwrap();

        let result = game.plant(EntityType::Grass);
        assert!(matches!(
            result.unwrap_err(),
            EngineError::EntityNotAllowedOnGroundType(EntityType::Grass, GroundType::Soil)
        ));
    }

    #[test]
    fn can_harvest_planted_entities_after_growth() {
        let mut game = create_test_game();

        game.plant(EntityType::Grass).unwrap();
        assert!(game.get_entity_type().unwrap().is_some());

        let tile = game.world.get_tile(0, 0).unwrap();
        let entity = tile.entity().unwrap();
        let now = chrono::Local::now().time();
        assert!(!entity.is_grown_at(now));
        assert!(!game.can_harvest().unwrap());

        let grown_time = now + entity.entity_type().growth_time();
        assert!(entity.is_grown_at(grown_time));
    }

    #[test]
    fn cannot_harvest_empty_tile() {
        let mut game = create_test_game();

        assert!(!game.can_harvest().unwrap());

        let result = game.harvest();
        assert!(matches!(result.unwrap_err(), EngineError::EntityNotGrown));
        assert!(game.get_entity_type().unwrap().is_none());
    }

    #[test]
    fn can_get_entity_type_from_tiles() {
        let mut game = create_test_game();

        assert!(game.get_entity_type().unwrap().is_none());

        game.plant(EntityType::Tree).unwrap();
        assert_eq!(game.get_entity_type().unwrap(), Some(&EntityType::Tree));
    }

    #[test]
    fn can_get_ground_type() {
        let game = create_test_game();

        let ground_type = game.get_ground_type().unwrap();
        assert_eq!(ground_type, &GroundType::Grassland);
    }

    #[test]
    fn can_toggle_ground_type_between_grassland_and_soil() {
        let mut game = create_test_game();

        assert_eq!(game.get_ground_type().unwrap(), &GroundType::Grassland);

        game.till().unwrap();
        assert_eq!(game.get_ground_type().unwrap(), &GroundType::Soil);

        game.till().unwrap();
        assert_eq!(game.get_ground_type().unwrap(), &GroundType::Grassland);
    }

    #[test]
    fn can_move_drone_east() {
        let mut game = create_test_game();

        assert_eq!(game.get_pos_x(), 0);
        assert_eq!(game.get_pos_y(), 0);

        game.move_drone(Direction::East).unwrap();
        assert_eq!(game.get_pos_x(), 1);
        assert_eq!(game.get_pos_y(), 0);
    }

    #[test]
    fn can_move_drone_west() {
        let mut game = create_test_game();

        game.move_drone(Direction::East).unwrap();
        assert_eq!(game.get_pos_x(), 1);

        game.move_drone(Direction::West).unwrap();
        assert_eq!(game.get_pos_x(), 0);
        assert_eq!(game.get_pos_y(), 0);
    }

    #[test]
    fn can_move_drone_south() {
        let mut game = create_test_game();

        assert_eq!(game.get_pos_y(), 0);

        game.move_drone(Direction::South).unwrap();
        assert_eq!(game.get_pos_x(), 0);
        assert_eq!(game.get_pos_y(), 1);
    }

    #[test]
    fn can_move_drone_north() {
        let mut game = create_test_game();

        game.move_drone(Direction::South).unwrap();
        assert_eq!(game.get_pos_y(), 1);

        game.move_drone(Direction::North).unwrap();
        assert_eq!(game.get_pos_x(), 0);
        assert_eq!(game.get_pos_y(), 0);
    }

    #[test]
    fn can_wrap_drone_east_at_world_boundary() {
        let mut game = create_test_game();

        for _ in 0..4 {
            game.move_drone(Direction::East).unwrap();
        }
        assert_eq!(game.get_pos_x(), 4);

        game.move_drone(Direction::East).unwrap();
        assert_eq!(game.get_pos_x(), 0);
        assert_eq!(game.get_pos_y(), 0);
    }

    #[test]
    fn can_wrap_drone_west_at_world_boundary() {
        let mut game = create_test_game();

        assert_eq!(game.get_pos_x(), 0);

        game.move_drone(Direction::West).unwrap();
        assert_eq!(game.get_pos_x(), 4);
        assert_eq!(game.get_pos_y(), 0);
    }

    #[test]
    fn can_wrap_drone_south_at_world_boundary() {
        let mut game = create_test_game();

        for _ in 0..4 {
            game.move_drone(Direction::South).unwrap();
        }
        assert_eq!(game.get_pos_y(), 4);

        game.move_drone(Direction::South).unwrap();
        assert_eq!(game.get_pos_x(), 0);
        assert_eq!(game.get_pos_y(), 0);
    }

    #[test]
    fn can_wrap_drone_north_at_world_boundary() {
        let mut game = create_test_game();

        assert_eq!(game.get_pos_y(), 0);

        game.move_drone(Direction::North).unwrap();
        assert_eq!(game.get_pos_x(), 0);
        assert_eq!(game.get_pos_y(), 4);
    }

    #[test]
    fn can_get_drone_x_position() {
        let mut game = create_test_game();
        assert_eq!(game.get_pos_x(), 0);

        game.move_drone(Direction::East).unwrap();
        assert_eq!(game.get_pos_x(), 1);

        game.move_drone(Direction::East).unwrap();
        assert_eq!(game.get_pos_x(), 2);
    }

    #[test]
    fn can_get_drone_y_position() {
        let mut game = create_test_game();
        assert_eq!(game.get_pos_y(), 0);

        game.move_drone(Direction::South).unwrap();
        assert_eq!(game.get_pos_y(), 1);

        game.move_drone(Direction::South).unwrap();
        assert_eq!(game.get_pos_y(), 2);
    }

    #[test]
    fn can_perform_complex_drone_movement_sequence() {
        let mut game = create_test_game();

        assert_eq!((game.get_pos_x(), game.get_pos_y()), (0, 0));

        game.move_drone(Direction::East).unwrap();
        assert_eq!((game.get_pos_x(), game.get_pos_y()), (1, 0));

        game.move_drone(Direction::South).unwrap();
        assert_eq!((game.get_pos_x(), game.get_pos_y()), (1, 1));

        game.move_drone(Direction::West).unwrap();
        assert_eq!((game.get_pos_x(), game.get_pos_y()), (0, 1));

        game.move_drone(Direction::North).unwrap();
        assert_eq!((game.get_pos_x(), game.get_pos_y()), (0, 0));
    }

    #[test]
    fn can_plant_at_drone_position() {
        let mut game = create_test_game();

        game.plant(EntityType::Grass).unwrap();

        assert_eq!(game.get_entity_type().unwrap(), Some(&EntityType::Grass));
    }

    #[test]
    fn can_harvest_at_drone_position_after_growth() {
        let mut game = create_test_game();

        game.plant(EntityType::Grass).unwrap();

        assert!(!game.can_harvest().unwrap());

        let tile = game.world.get_tile(0, 0).unwrap();
        let entity = tile.entity().unwrap();
        let now = chrono::Local::now().time();
        let grown_time = now + entity.entity_type().growth_time();
        assert!(entity.is_grown_at(grown_time));
    }

    #[test]
    fn can_harvest_planted_entity_after_growth() {
        let mut game = create_test_game();

        game.plant(EntityType::Tree).unwrap();

        assert!(!game.can_harvest().unwrap());

        let tile = game.world.get_tile(0, 0).unwrap();
        let entity = tile.entity().unwrap();
        let now = chrono::Local::now().time();
        let grown_time = now + entity.entity_type().growth_time();
        assert!(entity.is_grown_at(grown_time));
    }

    #[test]
    fn cannot_harvest_planted_entity_before_growth() {
        let mut game = create_test_game();

        game.till().unwrap();
        game.plant(EntityType::Carrot).unwrap();

        let result = game.harvest();
        assert!(matches!(result.unwrap_err(), EngineError::EntityNotGrown));
    }

    #[test]
    fn can_till_at_drone_position() {
        let mut game = create_test_game();

        game.till().unwrap();

        assert_eq!(game.get_ground_type().unwrap(), &GroundType::Soil);
    }
}
