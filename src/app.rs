use bevy::prelude::*;

use crate::data::card_store::{CardStore, Dirty};
use crate::ui::{
    tier_list::TierListUiPlugin,
    interaction::InteractionPlugin,
};
use crate::ui::card_view::CardViewPlugin;

pub struct TierListAppPlugin;

impl Plugin for TierListAppPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CardStore>()
            .init_resource::<Dirty>()

            // === Load Cards from cards.json ===
            .add_systems(Startup, load_cards)

            // === Cam ===
            .add_systems(Startup, setup_camera)

            // === UI ===
            .add_plugins(TierListUiPlugin)
            .add_plugins(InteractionPlugin)

            // === Cards ? ===
            .add_plugins(CardViewPlugin)

            // === Auto Save ===
            .add_systems(Update, auto_save_system)
            
            ;

    }
}


// TODO to move
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}


// TODO to move
fn load_cards(mut store: ResMut<CardStore>) {
    store.load_from_json("assets/cards.json");
}


use std::fs::File;
use std::io::BufWriter;

// TODO to move
fn auto_save_system(
    mut dirty: ResMut<Dirty>,
    store: Res<CardStore>,
) {
    if !dirty.0 { return; }

    let path = "assets/cards.json";
    if let Ok(file) = File::create(path) {
        let writer = BufWriter::new(file);
        if let Err(e) = serde_json::to_writer_pretty(writer, &store.cards) {
            error!("Auto-save failed: {}", e);
        } else {
            info!("Auto-save completed!");
            dirty.0 = false; // reset dirty
        }
    } else {
        error!("Cannot create cards.json for auto-save");
    }
}





// Updates the scroll position of scrollable nodes in response to mouse input
// pub fn update_scroll_position(
//     mut mouse_wheel_events: EventReader<MouseWheel>,
//     hover_map: Res<HoverMap>,
//     mut scrolled_node_query: Query<&mut ScrollPosition>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// ) {
//     for mouse_wheel_event in mouse_wheel_events.read() {
//         let (mut dx, mut dy) = match mouse_wheel_event.unit {
//             MouseScrollUnit::Line => (
//                 mouse_wheel_event.x * 20., // * LINE_HEIGHT,
//                 mouse_wheel_event.y * 20., // * LINE_HEIGHT,
//             ),
//             MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
//         };

//         if keyboard_input.pressed(KeyCode::ControlLeft)
//             || keyboard_input.pressed(KeyCode::ControlRight)
//         {
//             std::mem::swap(&mut dx, &mut dy);
//         }

//         for (_pointer, pointer_map) in hover_map.iter() {
//             for (entity, _hit) in pointer_map.iter() {
//                 if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
//                     scroll_position.offset_x -= dx;
//                     scroll_position.offset_y -= dy;
//                 }
//             }
//         }
//     }
// }
