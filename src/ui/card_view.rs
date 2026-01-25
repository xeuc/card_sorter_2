
use bevy::prelude::*;

use crate::ui::tier_list::TierUiSet;



use crate::data::{
    card::Card,
    card_store::CardStore,
};
use crate::ui::tier_list::{TierContainer, UnrankedArea};

pub struct CardViewPlugin;

impl Plugin for CardViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            spawn_cards.after(TierUiSet),
        );

    }
}





fn spawn_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    store: Res<CardStore>,
    tier_containers: Query<(Entity, &TierContainer)>,
    unranked: Query<Entity, With<UnrankedArea>>,
) {
    let unranked_entity = match unranked.single() {
        Ok(e) => e,
        Err(_) => commands.spawn(Node { ..default() }).id(),
    };

    for card in &store.cards {
        let parent = match &card.tier {
            Some(tier) => tier_containers
                .iter()
                .find(|(_, tc)| tc.tier.clone().label() == tier.clone().label()) // TODO remove .label() while addint ParialEq to Tier
                .map(|(e, _)| e)
                .unwrap_or(unranked_entity),
            None => unranked_entity,
        };

        spawn_card_view(
            &mut commands,
            &asset_server,
            parent,
            card,
        );
    }
}


fn spawn_card_view(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    parent: Entity,
    card: &Card,
) {
    let image_handle = asset_server.load(format!(
        "thumbs/{}",
        card.path
    ));

    let card_entity = commands
        .spawn((
            CardView,
            CardId(card.id.clone()),
            Button,
            ImageNode {
                image: image_handle.clone(),
                ..default()
            },

            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                margin: UiRect::all(Val::Px(4.0)),
                ..default()
            },
        ))
        .id();

    commands.entity(parent).add_child(card_entity);
}


#[derive(Component)]
pub struct CardView;

#[derive(Component)]
pub struct CardId(pub String);