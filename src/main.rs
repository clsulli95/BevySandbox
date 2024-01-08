pub mod game;
pub mod menu;
use bevy::prelude::*;
use menu::components::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(menu::components::DisplayQuality::Medium)
        .insert_resource(menu::components::Volume(7))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(menu::splash::SplashPlugin)
        .add_plugins(game::plugin::GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
