use bevy::prelude::*;
use crate::data::card::Tier;

pub struct TierListUiPlugin;

impl Plugin for TierListUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_tier_list_ui.in_set(TierUiSet));

    }
}


fn spawn_tier_list_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
    ))
    .with_children(|root| {
        spawn_tiers(root);
        spawn_unranked_area(root);
    });
}


fn spawn_tiers(parent: &mut ChildSpawnerCommands) {
    for tier in Tier::ORDER {
        parent.spawn((
            // TierRow { tier: tier.clone() },
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(80.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Stretch,
                ..default()
            },
        ))
        .with_children(|row| {
            spawn_tier_label(row, tier.clone());
            spawn_tier_container(row, tier.clone());
        });
    }
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
    parent.spawn((
        TierContainer { tier },
        Interaction::default(),
        Node {
            flex_grow: 1.0,
            flex_wrap: FlexWrap::Wrap,
            padding: UiRect::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    ));
}


fn spawn_unranked_area(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        UnrankedArea,
        Node {
            width: Val::Percent(100.0),
            min_height: Val::Px(200.0),
            flex_wrap: FlexWrap::Wrap,
            padding: UiRect::all(Val::Px(8.0)),
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


