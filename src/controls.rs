use crate::constants::*;
use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridConfig>()
            // 注册 Observer 监听 ControlEvent
            .add_observer(on_control_event)
            .add_systems(Update, (handle_input, draw_grid));
    }
}

#[derive(Resource, Default)]
struct GridConfig {
    show_grid: bool,
}

#[derive(Event)]
enum ControlEvent {
    ToggleGrid,
    // 未来可以在这里添加更多事件，如:
    // PauseGame,
    // RestartGame,
}

fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    // 检查 Command 键（Mac）或 Super 键
    let cmd_pressed = keyboard_input.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);

    // 检查 Cmd + G
    if cmd_pressed && keyboard_input.just_pressed(KeyCode::KeyG) {
        // 使用 trigger 立即触发事件
        commands.trigger(ControlEvent::ToggleGrid);
    }
}

// Observer 回调函数
fn on_control_event(trigger: On<ControlEvent>, mut grid_config: ResMut<GridConfig>) {
    match trigger.event() {
        ControlEvent::ToggleGrid => {
            grid_config.show_grid = !grid_config.show_grid;
            info!("Grid toggled: {}", grid_config.show_grid);
        }
    }
}

fn draw_grid(grid_config: Res<GridConfig>, mut gizmos: Gizmos) {
    if !grid_config.show_grid {
        return;
    }

    let width = GRID_WIDTH as f32 * GRID_SIZE;
    let height = GRID_HEIGHT as f32 * GRID_SIZE;

    let left = -width / 2.0;
    let right = width / 2.0;
    let bottom = -height / 2.0;
    let top = height / 2.0;

    // 绘制竖线
    for i in 0..=GRID_WIDTH {
        let x = left + i as f32 * GRID_SIZE;
        gizmos.line_2d(
            Vec2::new(x, bottom),
            Vec2::new(x, top),
            Color::WHITE.with_alpha(0.1),
        );
    }

    // 绘制横线
    for i in 0..=GRID_HEIGHT {
        let y = bottom + i as f32 * GRID_SIZE;
        gizmos.line_2d(
            Vec2::new(left, y),
            Vec2::new(right, y),
            Color::WHITE.with_alpha(0.1),
        );
    }
}
