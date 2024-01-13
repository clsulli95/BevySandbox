pub mod game;
pub mod menu;
pub mod settings;
pub mod splash;
pub mod util;

use bevy::prelude::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(settings::plugin::SettingsPlugin)
        .add_plugins(splash::plugin::SplashPlugin)
        .add_plugins(game::plugin::GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
