use crate::{
    consts::*,
    game::Game,
    world::{entities::EntityType, tiles::GroundType},
};

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "World Size: {}x{}",
            self.get_world_size().0,
            self.get_world_size().1
        )?;
        writeln!(
            f,
            "Drone Position: ({}, {})",
            self.get_pos_x(),
            self.get_pos_y()
        )?;

        if let Ok(ground_type) = self.get_ground_type() {
            writeln!(f, "Current Ground: {:?}", ground_type)?;
        }

        if let Ok(Some(entity_type)) = self.get_entity_type() {
            writeln!(f, "Current Entity: {:?}", entity_type)?;
            if let Ok(can_harvest) = self.can_harvest() {
                writeln!(f, "Can Harvest: {}", can_harvest)?;
            }
        } else {
            writeln!(f, "Current Entity: None")?;
        }

        writeln!(f)?;

        writeln!(f, "World Grid:")?;
        self.print_world_grid(f)?;

        Ok(())
    }
}

impl Game {
    pub fn print_world_grid(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = self.get_world_size();
        let drone_x = self.get_pos_x();
        let drone_y = self.get_pos_y();

        write!(f, "  ")?;
        for x in 0..width {
            write!(f, "{:2}", x)?;
        }
        writeln!(f)?;

        for display_y in (0..height).rev() {
            write!(f, "{:2} ", display_y)?;
            for x in 0..width {
                let tile_char = if x == drone_x && display_y == drone_y {
                    DRONE_SYMBOL
                } else {
                    match self.world().get_tile(x, display_y) {
                        Ok(tile) => match (tile.ground_type(), tile.entity()) {
                            (GroundType::Grassland, Some(entity)) => match entity.entity_type() {
                                EntityType::Grass => GRASS_SYMBOL,
                                EntityType::Bush => BUSH_SYMBOL,
                                EntityType::Tree => TREE_SYMBOL,
                                _ => UNKNOWN_SYMBOL,
                            },
                            (GroundType::Grassland, None) => GRASSLAND_EMPTY_SYMBOL,
                            (GroundType::Soil, Some(entity)) => match entity.entity_type() {
                                EntityType::Carrot => CARROT_SYMBOL,
                                EntityType::Pumpkin => PUMPKIN_SYMBOL,
                                _ => UNKNOWN_SYMBOL,
                            },
                            (GroundType::Soil, None) => SOIL_EMPTY_SYMBOL,
                        },
                        Err(_) => UNKNOWN_SYMBOL,
                    }
                };
                write!(f, "{:2}", tile_char)?;
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "Legend:")?;
        writeln!(f, "  {} = Drone position", DRONE_SYMBOL)?;
        writeln!(f, "  {} = Grassland (empty)", GRASSLAND_EMPTY_SYMBOL)?;
        writeln!(f, "  {} = Soil (empty)", SOIL_EMPTY_SYMBOL)?;
        writeln!(f, "  {} = Grass", GRASS_SYMBOL)?;
        writeln!(f, "  {} = Bush", BUSH_SYMBOL)?;
        writeln!(f, "  {} = Tree", TREE_SYMBOL)?;
        writeln!(f, "  {} = Carrot", CARROT_SYMBOL)?;
        writeln!(f, "  {} = Pumpkin", PUMPKIN_SYMBOL)?;

        Ok(())
    }

    pub fn get_world_grid_string(&self) -> String {
        let (width, height) = self.get_world_size();
        let drone_x = self.get_pos_x();
        let drone_y = self.get_pos_y();

        let mut grid = String::new();

        // Display from top to bottom, but with y-axis starting from bottom
        for display_y in (0..height).rev() {
            for x in 0..width {
                let tile_char = if x == drone_x && display_y == drone_y {
                    DRONE_SYMBOL
                } else {
                    match self.world().get_tile(x, display_y) {
                        Ok(tile) => match (tile.ground_type(), tile.entity()) {
                            (GroundType::Grassland, Some(entity)) => match entity.entity_type() {
                                EntityType::Grass => GRASS_SYMBOL,
                                EntityType::Bush => BUSH_SYMBOL,
                                EntityType::Tree => TREE_SYMBOL,
                                _ => UNKNOWN_SYMBOL,
                            },
                            (GroundType::Grassland, None) => GRASSLAND_EMPTY_SYMBOL,
                            (GroundType::Soil, Some(entity)) => match entity.entity_type() {
                                EntityType::Carrot => CARROT_SYMBOL,
                                EntityType::Pumpkin => PUMPKIN_SYMBOL,
                                _ => UNKNOWN_SYMBOL,
                            },
                            (GroundType::Soil, None) => SOIL_EMPTY_SYMBOL,
                        },
                        Err(_) => UNKNOWN_SYMBOL,
                    }
                };
                grid.push_str(tile_char);
            }
            if display_y > 0 {
                grid.push('\n');
            }
        }

        grid
    }

    pub fn get_tile_info(&self, x: usize, y: usize) -> Result<String, crate::error::EngineError> {
        let tile = self.world().get_tile(x, y)?;
        let mut info = format!("Tile ({}, {}): ", x, y);

        match (tile.ground_type(), tile.entity()) {
            (GroundType::Grassland, Some(entity)) => {
                info.push_str(&format!("Grassland with {:?}", entity.entity_type()));
                if entity.is_grown() {
                    info.push_str(" (grown)");
                } else {
                    info.push_str(" (growing)");
                }
            }
            (GroundType::Grassland, None) => {
                info.push_str("Grassland (empty)");
            }
            (GroundType::Soil, Some(entity)) => {
                info.push_str(&format!("Soil with {:?}", entity.entity_type()));
                if entity.is_grown() {
                    info.push_str(" (grown)");
                } else {
                    info.push_str(" (growing)");
                }
            }
            (GroundType::Soil, None) => {
                info.push_str("Soil (empty)");
            }
        }

        Ok(info)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, GameOptions};

    fn create_test_game() -> Game {
        Game::new(GameOptions {
            world_width: 3,
            world_height: 3,
        })
    }

    #[test]
    fn can_display_game() {
        let game = create_test_game();
        let display_string = format!("{}", game);

        assert!(display_string.contains("World Size: 3x3"));
        assert!(display_string.contains("Drone Position: (0, 0)"));
        assert!(display_string.contains("World Grid:"));
        assert!(display_string.contains("Legend:"));
    }

    #[test]
    fn can_get_world_grid_string() {
        let game = create_test_game();
        let grid_string = game.get_world_grid_string();

        let lines: Vec<&str> = grid_string.lines().collect();
        assert_eq!(lines.len(), 3);
        // With y-axis starting from bottom, drone at (0,0) should be at the last line
        assert!(lines[2].starts_with("D"));
    }

    #[test]
    fn can_get_tile_info() {
        let game = create_test_game();
        let tile_info = game.get_tile_info(0, 0).unwrap();

        assert!(tile_info.contains("Tile (0, 0)"));
        assert!(tile_info.contains("Grassland"));
    }
}
