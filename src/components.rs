use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct HumanPaddle {
    pub up_key: KeyCode,
    pub down_key: KeyCode,
}

#[derive(Debug, Component)]
pub struct RandomPaddle;

#[derive(Debug, Component, Default)]
pub struct PaddleInput {
    pub up: bool,
    pub down: bool,
}

#[derive(Debug, Component)]
pub struct Ball {
    pub velocity: Vec2,
}
