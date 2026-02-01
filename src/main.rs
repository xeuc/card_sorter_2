use bevy::prelude::*;

mod app;
mod data;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(app::TierListAppPlugin)
        .run();
}
