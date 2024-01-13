use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

#[allow(clippy::module_name_repetitions)]
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayQuality::Medium)
            .insert_resource(Volume(7))
            .insert_resource(Volume(7));
    }
}
