use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::components::*;
use crate::constants::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(ARENA_WIDTH, ARENA_WALL_HEIGHT)).into())
            .into(),
        material: materials.add(ColorMaterial::from(ARENA_WALL_COLOR)),
        transform: Transform::from_translation(Vec3::new(
            0.0,
            -ARENA_HEIGHT / 2.0 + ARENA_WALL_HEIGHT / 2.0,
            0.0,
        )),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(ARENA_WIDTH, ARENA_WALL_HEIGHT)).into())
            .into(),
        material: materials.add(ColorMaterial::from(ARENA_WALL_COLOR)),
        transform: Transform::from_translation(Vec3::new(
            0.0,
            ARENA_HEIGHT / 2.0 - ARENA_WALL_HEIGHT / 2.0,
            0.0,
        )),
        ..default()
    });

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)).into())
                .into(),
            material: materials.add(ColorMaterial::from(PADDLE_COLOR)),
            transform: Transform::from_translation(Vec3::new(
                -ARENA_WIDTH / 2.0 + PADDLE_WIDTH / 2.0 + ARENA_PADDING,
                0.0,
                0.0,
            )),
            ..default()
        })
        .insert(PaddleInput::default())
        .insert(HumanPaddle {
            up_key: KeyCode::W,
            down_key: KeyCode::S,
        });

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)).into())
                .into(),
            material: materials.add(ColorMaterial::from(PADDLE_COLOR)),
            transform: Transform::from_translation(Vec3::new(
                ARENA_WIDTH / 2.0 - PADDLE_WIDTH / 2.0 - ARENA_PADDING,
                0.0,
                0.0,
            )),
            ..default()
        })
        .insert(PaddleInput::default())
        .insert(HumanPaddle {
            up_key: KeyCode::Up,
            down_key: KeyCode::Down,
        });
}

pub fn paddle_movement(
    time: Res<Time>,
    mut paddle_input_query: Query<(&PaddleInput, &mut Transform)>,
) {
    for (paddle_input, mut transform) in paddle_input_query.iter_mut() {
        let direction = (paddle_input.up as i32 - paddle_input.down as i32) as f32;

        transform.translation.y =
            (transform.translation.y + direction * PADDLE_SPEED * time.delta_seconds()).clamp(
                -ARENA_HEIGHT / 2.0 + PADDLE_HEIGHT / 2.0 + ARENA_WALL_HEIGHT,
                ARENA_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0 - ARENA_WALL_HEIGHT,
            );
    }
}

pub fn random_paddle(mut paddle_input_query: Query<(&RandomPaddle, &mut PaddleInput)>) {
    for (_, mut paddle_input) in paddle_input_query.iter_mut() {
        paddle_input.up = rand::random();
        paddle_input.down = rand::random();
    }
}

pub fn human_paddle(
    mut paddle_input_query: Query<(&HumanPaddle, &mut PaddleInput)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (human_paddle, mut paddle_input) in paddle_input_query.iter_mut() {
        paddle_input.up = keyboard_input.pressed(human_paddle.up_key);
        paddle_input.down = keyboard_input.pressed(human_paddle.down_key);
    }
}

pub fn handle_create_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ball_query: Query<&Ball>,
) {
    if ball_query.iter().next().is_none() {
        let speed_y = (rand::random::<f32>() * 2.0 - 1.0) * 0.2;
        let speed_x = (1.0 - speed_y.powi(2)).sqrt() * (if rand::random() { 1.0 } else { -1.0 });

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(
                        shape::Circle {
                            radius: BALL_RADIUS,
                            ..default()
                        }
                        .into(),
                    )
                    .into(),
                material: materials.add(ColorMaterial::from(BALL_COLOR)),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            })
            .insert(Ball {
                velocity: Vec2::new(speed_x, speed_y).normalize() * BALL_SPEED,
            });
    }
}

pub fn move_ball(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in ball_query.iter_mut() {
        transform.translation += Vec3::new(
            ball.velocity.x * time.delta_seconds(),
            ball.velocity.y * time.delta_seconds(),
            0.0,
        );
    }
}

pub fn handle_wall_collision(mut ball_query: Query<(&mut Ball, &mut Transform)>) {
    let max_y = ARENA_HEIGHT / 2.0 - ARENA_WALL_HEIGHT - BALL_RADIUS;
    let min_y = -ARENA_HEIGHT / 2.0 + ARENA_WALL_HEIGHT + BALL_RADIUS;

    if let Ok((mut ball, mut transform)) = ball_query.get_single_mut() {
        let ball_y = transform.translation.y;

        if ball_y < min_y || max_y < ball_y {
            transform.translation.y = ball_y.clamp(min_y, max_y);
            ball.velocity.y = -ball.velocity.y;
        }
    }
}

pub fn handle_paddle_collision(
    mut ball_query: Query<(&mut Ball, &mut Transform), Without<PaddleInput>>,
    paddle_query: Query<&Transform, With<PaddleInput>>,
) {
    let max_x = ARENA_WIDTH / 2.0 - PADDLE_WIDTH - ARENA_PADDING - BALL_RADIUS;
    let min_x = -ARENA_WIDTH / 2.0 + PADDLE_WIDTH + ARENA_PADDING + BALL_RADIUS;

    if let Ok((mut ball, mut transform)) = ball_query.get_single_mut() {
        let ball_x = transform.translation.x;
        let ball_y = transform.translation.y;

        let paddle = paddle_query.iter().find(|paddle| {
            let paddle_x = paddle.translation.x;

            ball.velocity.x > 0.0 && ball_x < paddle_x || ball.velocity.x < 0.0 && ball_x > paddle_x
        });

        if let Some(paddle_transform) = paddle {
            let paddle_y = paddle_transform.translation.y;

            let max_y = paddle_y + PADDLE_HEIGHT / 2.0;
            let min_y = paddle_y - PADDLE_HEIGHT / 2.0;

            if (ball_x < min_x || max_x < ball_x) && min_y < ball_y && ball_y < max_y {
                let fraction_y = (ball_y - min_y) / (max_y - min_y);
                let factor = fraction_y * 2.0 - 1.0;

                transform.translation.x = ball_x.clamp(min_x, max_x);

                ball.velocity.y = factor * BALL_SPEED;
                ball.velocity.x = -ball.velocity.x;
            }
        }
    }
}

pub fn handle_destroy_ball(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &Ball, &Transform)>,
) {
    if let Ok((entity, _, transform)) = ball_query.get_single_mut() {
        let ball_x = transform.translation.x;

        if !(-ARENA_WIDTH / 2.0..=ARENA_WIDTH / 2.0).contains(&ball_x) {
            commands.entity(entity).despawn();
        }
    }
}
