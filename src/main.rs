use bevy::prelude::*;

mod constants;
mod snake;
mod food;

use constants::*;
use snake::SnakePlugin;
use food::FoodPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "贪吃蛇".into(),
                    resolution: (
                        (GRID_SIZE * GRID_WIDTH as f32) as u32,
                        (GRID_SIZE * GRID_HEIGHT as f32) as u32,
                    )
                        .into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(SnakePlugin)
        .add_plugins(FoodPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // 相机
    commands.spawn(Camera2d::default());
}
