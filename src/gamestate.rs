use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub static mut current_score: i32 = 0;
pub static mut best_score: i32 = 0;

