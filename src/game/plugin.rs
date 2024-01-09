use bevy::prelude::*;

use crate::settings::plugin::{DisplayQuality, Volume};
use crate::util;

#[allow(clippy::module_name_repetitions)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(crate::GameState::Game)))
            .add_systems(
                OnExit(crate::GameState::Game),
                util::spawn::despawn_screen::<OnGameScreen>,
            );
    }
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

#[allow(clippy::needless_pass_by_value)]
fn game_setup(mut commands: Commands, display_quality: Res<DisplayQuality>, volume: Res<Volume>) {
    /////////////
    let game_bundle_style = Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let game_bundle = NodeBundle {
        style: game_bundle_style,
        ..default()
    };

    let options_bundle_style = Style {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    };

    let options_bundle = NodeBundle {
        style: options_bundle_style,
        background_color: Color::BLACK.into(),
        ..default()
    };

    commands
        .spawn((game_bundle, OnGameScreen))
        .with_children(|parent| {
            parent.spawn(options_bundle).with_children(|parent| {
                parent.spawn(get_back_to_menu_prompt(60.0, Color::WHITE));
                parent.spawn(get_current_settings(
                    60.0,
                    Color::WHITE,
                    *display_quality,
                    *volume,
                ));
            });
        });

    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

#[allow(clippy::needless_pass_by_value)]
fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<crate::GameState>>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(crate::GameState::Menu);
    }
}

fn get_back_to_menu_prompt(font_size: f32, color: Color) -> TextBundle {
    let back_to_menu_text_section =
        create_text_section("Will be back to the menu shortly...", font_size, color);

    let back_to_menu_text_style = Style {
        margin: UiRect::all(Val::Px(50.0)),
        ..default()
    };

    TextBundle::from_sections([back_to_menu_text_section]).with_style(back_to_menu_text_style)
}

fn get_current_settings(
    font_size: f32,
    color: Color,
    display_quality: DisplayQuality,
    volume: Volume,
) -> TextBundle {
    let quality_text_section = create_text_section(
        format!("quality: {display_quality:?}").as_str(),
        font_size,
        color,
    );

    let separator_text_section = create_text_section(" - ", font_size, color);

    let volume_text_section =
        create_text_section(format!("volume: {volume:?}").as_str(), font_size, color);

    let options_bundle_style = Style {
        margin: UiRect::all(Val::Px(50.0)),
        ..default()
    };

    TextBundle::from_sections([
        quality_text_section,
        separator_text_section,
        volume_text_section,
    ])
    .with_style(options_bundle_style)
}

fn create_text_section(text: &str, font_size: f32, color: Color) -> TextSection {
    TextSection::new(
        text.to_string(),
        TextStyle {
            font_size,
            color,
            ..default()
        },
    )
}
