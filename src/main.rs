use bevy::prelude::*;
use std::collections::VecDeque;

//=============== 配置 ==================
const GRID_SIZE: f32 = 20.0; // 每个格子大小
const GRID_WIDTH: i32 = 30; // 横向格子数
const GRID_HEIGHT: i32 = 20; // 纵向格子数
const SNAKE_COLOR: Color = Color::srgb(0.0, 0.8, 0.0);
const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

//=============== 组件 ==================

// 蛇头
#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

// 蛇身体段
#[derive(Component)]
struct SnakeSegment;

// 存储蛇的所有段位置（用于身体跟随）
#[derive(Resource)]
struct SnakeSegments(VecDeque<Entity>);

// 移动计时器（控制蛇移动速度）
#[derive(Resource)]
struct MoveTimer(Timer);

// 方向枚举
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

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
        .insert_resource(SnakeSegments(VecDeque::new()))
        .insert_resource(MoveTimer(Timer::from_seconds(0.15, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (input_handling, snake_movement))
        .run();
}

//=============== 系统 ==================

fn setup(mut commands: Commands) {
    // 相机
    commands.spawn(Camera2d::default());

    // 创建蛇头（从中心开始）
    let head_pos = Vec3::new(0.0, 0.0, 0.0);
    let head = commands
        .spawn((
            Sprite {
                color: SNAKE_COLOR,
                custom_size: Some(Vec2::splat(GRID_SIZE - 2.0)), // 与格子间保留2px间距
                ..default()
            },
            Transform::from_translation(head_pos),
            SnakeHead {
                direction: Direction::Right, // 初始向右
            },
        ))
        .id();

    // 添加2段身体（初始长度为3）
    let segment1 = spawn_segment(&mut commands, Vec3::new(-GRID_SIZE, 0.0, 0.0));
    let segment2 = spawn_segment(&mut commands, Vec3::new(-GRID_SIZE * 2.0, 0.0, 0.0));

    // 初始化蛇身
    commands.insert_resource(SnakeSegments({
        let mut deque = VecDeque::new();
        deque.push_back(head);
        deque.push_back(segment1);
        deque.push_back(segment2);
        deque
    }));
}

fn spawn_segment(commands: &mut Commands, position: Vec3) -> Entity {
    commands
        .spawn((
            Sprite {
                color: SNAKE_COLOR,
                custom_size: Some(Vec2::splat(GRID_SIZE - 2.0)),
                ..default()
            },
            Transform::from_translation(position),
            SnakeSegment,
        ))
        .id()
}

// 键盘输入处理
fn input_handling(keyboard: Res<ButtonInput<KeyCode>>, mut head_query: Query<&mut SnakeHead>) {
    let mut head = head_query.single_mut().unwrap();

    let new_direction = if keyboard.just_pressed(KeyCode::KeyW) {
        Some(Direction::Up)
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        Some(Direction::Down)
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        Some(Direction::Left)
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        Some(Direction::Right)
    } else {
        None
    };

    // 防止180度掉头（不能反向移动）
    if let Some(new_dir) = new_direction {
        if new_dir != head.direction.opposite() {
            head.direction = new_dir;
        }
    }
}

// 蛇移动系统
fn snake_movement(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    segments: ResMut<SnakeSegments>,
    mut transforms: Query<&mut Transform>,
    head_query: Query<&SnakeHead>,
) {
    // 计时器控制移动频率
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let head = head_query.single().unwrap();

    // 计算头部位置
    let head_entity = segments.0.front().copied().unwrap();
    let head_transform = transforms.get(head_entity).unwrap();
    let head_pos = head_transform.translation;

    let new_pos = match head.direction {
        Direction::Up => Vec3::new(head_pos.x, head_pos.y + GRID_SIZE, 0.0),
        Direction::Down => Vec3::new(head_pos.x, head_pos.y - GRID_SIZE, 0.0),
        Direction::Left => Vec3::new(head_pos.x - GRID_SIZE, head_pos.y, 0.0),
        Direction::Right => Vec3::new(head_pos.x + GRID_SIZE, head_pos.y, 0.0),
    };

    // 身体跟随：从胃部开始，每个段移动到前一个段的位置
    // 为了避免借用冲突，先收集当前所有段的位置
    let segment_positions: Vec<Vec3> = segments.0.iter()
        .map(|e| transforms.get(*e).unwrap().translation)
        .collect();

    // 更新身体段的位置
    for (i, &entity) in segments.0.iter().enumerate().skip(1) {
        if let Ok(mut transform) = transforms.get_mut(entity) {
            transform.translation = segment_positions[i - 1];
        }
    }
    
    // 头部移动到新位置
    if let Ok(mut head_transform) = transforms.get_mut(head_entity) {
        head_transform.translation = new_pos;
    }
}
