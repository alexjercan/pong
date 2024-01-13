use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const PADDLE_SPEED: f32 = 500.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

const BALL_RADIUS: f32 = 10.0;
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_SPEED: f32 = 250.0;

const ARENA_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const ARENA_WIDTH: f32 = 800.0;
const ARENA_HEIGHT: f32 = 600.0;
const ARENA_PADDING: f32 = 30.0;
const ARENA_WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const ARENA_WALL_HEIGHT: f32 = 25.0;

#[derive(Debug, Component)]
struct HumanPaddle {
    up_key: KeyCode,
    down_key: KeyCode,
}

#[derive(Debug, Component)]
struct RandomPaddle;

#[derive(Debug, Component, Default)]
struct PaddleInput {
    up: bool,
    down: bool,
}

#[derive(Debug, Component)]
struct Ball {
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                title: "Pong!".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(ARENA_COLOR))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                paddle_movement,
                random_paddle,
                human_paddle,
                handle_create_ball,
                move_ball,
                handle_wall_collision,
                handle_paddle_collision,
                handle_destroy_ball,
            ),
        )
        .run();
}

fn setup(
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

fn paddle_movement(time: Res<Time>, mut paddle_input_query: Query<(&PaddleInput, &mut Transform)>) {
    for (paddle_input, mut transform) in paddle_input_query.iter_mut() {
        let direction = (paddle_input.up as i32 - paddle_input.down as i32) as f32;

        transform.translation.y =
            (transform.translation.y + direction * PADDLE_SPEED * time.delta_seconds()).clamp(
                -ARENA_HEIGHT / 2.0 + PADDLE_HEIGHT / 2.0 + ARENA_WALL_HEIGHT,
                ARENA_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0 - ARENA_WALL_HEIGHT,
            );
    }
}

fn random_paddle(mut paddle_input_query: Query<(&RandomPaddle, &mut PaddleInput)>) {
    for (_, mut paddle_input) in paddle_input_query.iter_mut() {
        paddle_input.up = rand::random();
        paddle_input.down = rand::random();
    }
}

fn human_paddle(
    mut paddle_input_query: Query<(&HumanPaddle, &mut PaddleInput)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (human_paddle, mut paddle_input) in paddle_input_query.iter_mut() {
        paddle_input.up = keyboard_input.pressed(human_paddle.up_key);
        paddle_input.down = keyboard_input.pressed(human_paddle.down_key);
    }
}

fn handle_create_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ball_query: Query<&Ball>,
) {
    if ball_query.iter().next().is_none() {
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
                velocity: Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5)
                    .normalize()
                    * BALL_SPEED,
            });
    }
}

fn move_ball(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in ball_query.iter_mut() {
        transform.translation += Vec3::new(
            ball.velocity.x * time.delta_seconds(),
            ball.velocity.y * time.delta_seconds(),
            0.0,
        );
    }
}

fn handle_wall_collision(mut ball_query: Query<(&mut Ball, &mut Transform)>) {
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

fn handle_paddle_collision(
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
                transform.translation.x = ball_x.clamp(min_x, max_x);
                ball.velocity.x = -ball.velocity.x;
            }
        }
    }
}

fn handle_destroy_ball(mut commands: Commands, mut ball_query: Query<(Entity, &Ball, &Transform)>) {
    if let Ok((entity, _, transform)) = ball_query.get_single_mut() {
        let ball_x = transform.translation.x;

        if !(-ARENA_WIDTH / 2.0..=ARENA_WIDTH / 2.0).contains(&ball_x) {
            commands.entity(entity).despawn();
        }
    }
}
