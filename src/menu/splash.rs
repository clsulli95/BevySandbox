use crate::menu::components::GameState;
use crate::menu::util;
use bevy::prelude::*;

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[allow(clippy::module_name_repetitions)]
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(
                OnExit(GameState::Splash),
                util::despawn_screen::<OnSplashScreen>,
            );
    }
}

#[allow(clippy::needless_pass_by_value)]
fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("bevy_logo_fill.png");

    let splash_screen_style = Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    };

    let splash_screen_group = NodeBundle {
        style: splash_screen_style,
        ..default()
    };

    let logo_style = Style {
        width: Val::Px(1280.0),
        height: Val::Px(620.0),
        ..default()
    };

    let logo = ImageBundle {
        style: logo_style,
        image: UiImage::new(icon),
        ..default()
    };

    commands
        .spawn((splash_screen_group, OnSplashScreen))
        .with_children(|parent| {
            parent.spawn(logo);
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

#[allow(clippy::needless_pass_by_value)]
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
