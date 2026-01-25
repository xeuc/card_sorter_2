use bevy::prelude::*;


#[derive(Resource, Default)]
pub struct SelectedCard {
    pub card_id: Option<String>,
}

use crate::ui::card_view::{CardView, CardId};
use crate::ui::tier_list::TierContainer;
use crate::data::card_store::{CardStore, Dirty};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedCard>()
            .add_systems(Update, (
                select_card,
                move_card_to_tier,
            ));
    }
}



fn select_card(
    mut selected: ResMut<SelectedCard>,
    query: Query<(&Interaction, &CardId), (Changed<Interaction>, With<CardView>)>,
) {
    for (interaction, card_id) in &query {
        if *interaction == Interaction::Pressed {
            selected.card_id = Some(card_id.0.clone());
            info!("Selected card {}", card_id.0);
        }
    }
}



fn move_card_to_tier(
    mut commands: Commands,
    mut store: ResMut<CardStore>,
    mut selected: ResMut<SelectedCard>,
    mut dirty: ResMut<Dirty>,

    tier_query: Query<(Entity, &Interaction, &TierContainer), Changed<Interaction>>,
    card_query: Query<(Entity, &CardId), With<CardView>>,
) {
    let Some(selected_id) = selected.card_id.clone() else { return; };

    for (tier_entity, interaction, tier_container) in &tier_query {
        if *interaction != Interaction::Pressed { continue; }

        let Some((card_entity, _)) = card_query
            .iter()
            .find(|(_, id)| id.0 == selected_id)
        else { return; };

        commands.entity(tier_entity).add_child(card_entity);

        if let Some(card) = store.cards.iter_mut().find(|c| c.id == selected_id) {
            card.tier = Some(tier_container.tier.clone());
        }

        dirty.0 = true;
        selected.card_id = None;

    }
}
