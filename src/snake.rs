use bevy::prelude::*;
use std::collections::VecDeque;

use crate::constants::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeSegments(VecDeque::new()))
            .insert_resource(MoveTimer(Timer::from_seconds(0.15, TimerMode::Repeating)))
            .add_systems(Startup, spawn_snake)
            .add_systems(Update, (input_handling, snake_movement));
    }
}

//=============== 组件 ==================

// 蛇头
#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

// 蛇身体段
#[derive(Component)]
pub struct SnakeSegment;

// 存储蛇的所有段位置（用于身体跟随）
#[derive(Resource)]
pub struct SnakeSegments(pub VecDeque<Entity>);

// 移动计时器（控制蛇移动速度）
#[derive(Resource)]
pub struct MoveTimer(pub Timer);

// 方向枚举
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

//=============== 系统 ==================

fn spawn_snake(mut commands: Commands) {
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
    if let Ok(mut head) = head_query.single_mut() {
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

    if let Ok(head) = head_query.single() {
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

        // 身体跟随：从尾部开始，每个段移动到前一个段的位置
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
}
