use bevy::prelude::*;
use crate::data::card::{Tier};
use crate::data::card_store::{CardStore, Dirty};
use crate::ui::card_view::{CardId, CardView};
use crate::ui::interaction::SelectedCard;

pub struct TierListUiPlugin;

impl Plugin for TierListUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_tier_list_ui.in_set(TierUiSet));

    }
}

const TIERS_HOLDER_FONT_COLOR: Color = Color::linear_rgb(0.05, 0.05, 0.05);
const WAITING_FONT_COLOR: Color = Color::linear_rgb(0.01, 0.01, 0.01);


#[derive(Component)]
pub struct BigCardFullShowArea;


fn spawn_tier_list_ui(mut commands: Commands) {
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        ..default()
    })
    .with_children(|root| {

        // left half of screen for tier list
        root.spawn(Node {
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|left| {
            spawn_tiers(left);
            spawn_unranked_area(left);
        });

        // right half for big Card preview
        root.spawn((
            BigCardFullShowArea,
            Node {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                overflow: Overflow::hidden(),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.15, 0.15, 0.15)),
        ));
    });
}



fn spawn_tiers(parent: &mut ChildSpawnerCommands) {
    for tier in Tier::ORDER {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(80.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Stretch,
                ..default()
            },
            tier.clone(),
        ))
        .with_children(|row| {
            spawn_tier_label(row, tier.clone());
            spawn_tier_container(row, tier.clone());
        })
        // .observe(rotate_on_drag)
        ;
    }
}


fn _rotate_on_drag(
    drag: On<Pointer<Release>>,

    mut selected: ResMut<SelectedCard>,
    mut store: ResMut<CardStore>,
    mut dirty: ResMut<Dirty>,
    mut commands: Commands,

    card_query: Query<(Entity, &CardId), With<CardView>>,
    tier_query: Query<&Tier>,
) {
    let tier = tier_query.get(drag.entity).unwrap().clone();

    let Some(selected_id) = selected.card_id.clone() else { return; };
    info!("Dropping card {:?} into tier {:?}", selected_id, tier.clone().label());

    let Some((card_entity, _)) = card_query
        .iter()
        .find(|(_, id)| id.0 == selected_id)
    else { return };

    commands.entity(drag.entity).add_child(card_entity);

    if let Some(card) = store.cards.iter_mut().find(|c| c.id == selected_id) {
        card.tier = Some(tier);
    }

    dirty.0 = true;
    selected.card_id = None;
}



fn spawn_tier_label(parent: &mut ChildSpawnerCommands, tier: Tier) {
    parent.spawn((
        TierLabel,
        Node {
            width: Val::Px(60.0),
            ..default()
        },
        BackgroundColor(tier.clone().color()),
        Text::new(tier.label()),
    ));
}


fn spawn_tier_container(parent: &mut ChildSpawnerCommands, tier: Tier) {
    parent
        // wrapper for scroll
        .spawn((
            Node {
                flex_grow: 1.0,
                height: Val::Px(120.0), // fixed height for the scroll to work
                overflow: Overflow::scroll_y(), // n.b.
                ..default()
            },
            BackgroundColor(TIERS_HOLDER_FONT_COLOR),
        ))
        .with_children(|scroll_view| {
            // real show of cards
            scroll_view.spawn((
                TierContainer { tier },
                Interaction::default(),
                Node {
                    width: Val::Percent(100.0),
                    flex_wrap: FlexWrap::Wrap,
                    padding: UiRect::all(Val::Px(6.0)),
                    ..default()
                },
            ));
        });
}


fn spawn_unranked_area(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        UnrankedArea,
        Node {
            width: Val::Percent(100.0),
            min_height: Val::Px(200.0),
            flex_wrap: FlexWrap::Wrap,
            padding: UiRect::all(Val::Px(8.0)),
            overflow: Overflow::scroll_y(), // n.b.
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    ));
}



#[derive(Component)]
pub struct TierLabel;

#[derive(Component)]
pub struct TierContainer {
    pub tier: Tier,
}

#[derive(Component)]
pub struct UnrankedArea;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TierUiSet;


