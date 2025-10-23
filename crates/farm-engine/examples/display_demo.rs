use farm_engine::game::{Direction, Game, GameOptions};
use farm_engine::world::entities::EntityType;

fn main() {
    let mut game = Game::new(GameOptions {
        world_width: 5,
        world_height: 5,
    });

    println!("Initial game state:");
    println!("{}", game);

    println!("================================================");

    game.plant(EntityType::Grass).unwrap();
    game.move_drone(Direction::East).unwrap();
    game.plant(EntityType::Bush).unwrap();

    game.move_drone(Direction::East).unwrap();
    game.till().unwrap();
    game.plant(EntityType::Carrot).unwrap();

    game.move_drone(Direction::South).unwrap();
    game.plant(EntityType::Tree).unwrap();

    game.move_drone(Direction::East).unwrap();
    game.till().unwrap();
    game.plant(EntityType::Pumpkin).unwrap();

    println!("\nAfter planting various entities:");
    println!("{}", game);

    println!("\nDetailed tile information:");
    for x in 0..5 {
        for y in 0..5 {
            if let Ok(info) = game.get_tile_info(x, y) {
                println!("{}", info);
            }
        }
    }

    println!("\nGrid only:");
    println!("{}", game.get_world_grid_string());
}
