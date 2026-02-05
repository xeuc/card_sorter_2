
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

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
            .add_systems(Update, send_scroll_events)
            .add_observer(on_scroll_handler)
            ;

    }
}


// TODO to move
fn setup_camera(mut commands: Commands) {
    // commands.spawn(Camera2d);
    commands.spawn((Camera2d, IsDefaultUiCamera));
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




const LINE_HEIGHT: f32 = 21.;

/// Injects scroll events into the UI hierarchy.
fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(Scroll { entity, delta });
            }
        }
    }
}

/// UI scrolling event.
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
struct Scroll {
    entity: Entity,
    /// Scroll delta in logical coordinates.
    delta: Vec2,
}

fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let delta = &mut scroll.delta;
    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.x > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };

        if !max {
            scroll_position.x += delta.x;
            // Consume the X portion of the scroll delta.
            delta.x = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
            scroll_position.y += delta.y;
            // Consume the Y portion of the scroll delta.
            delta.y = 0.;
        }
    }

    // Stop propagating when the delta is fully consumed.
    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}
