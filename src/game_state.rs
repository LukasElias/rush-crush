use bevy::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Level,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}
