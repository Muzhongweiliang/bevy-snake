use bevy::prelude::*;
use rand::RngExt; // 引入 RngExt trait 以使用 random_range

use crate::{constants::*, snake::SnakeSegments};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, food_spawner);
    }
}

#[derive(Component)]
pub struct Food;

fn food_spawner(
    mut commands: Commands,
    food_query: Query<&Food>,
    snake_segments: Res<SnakeSegments>,
    transfoms: Query<&Transform>,
) {
    // 如果已经有食物，就不生成
    if !food_query.is_empty() {
        return;
    }

    let mut rng = rand::rng();

    let snake_positions: Vec<Vec3> = snake_segments
        .0
        .iter()
        .filter_map(|e| transfoms.get(*e).ok())
        .map(|t| t.translation)
        .collect();

    loop {
        let x = rng.random_range(-GRID_WIDTH / 2..GRID_WIDTH / 2) as f32 * GRID_SIZE;
        let y = rng.random_range(-GRID_HEIGHT / 2..GRID_HEIGHT / 2) as f32 * GRID_SIZE;
        let pos = Vec3::new(x, y, 0.0);

        if !snake_positions.iter().any(|&p| p == pos) {
            commands.spawn((
                Food,
                Sprite {
                    color: FOOD_COLOR,
                    custom_size: Some(Vec2::splat(GRID_SIZE - 2.0)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
            break;
        }
    }
}
