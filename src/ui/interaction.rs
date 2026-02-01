use bevy::prelude::*;


#[derive(Resource, Default)]
pub struct SelectedCard {
    pub card_id: Option<String>,
}

// When you hover a thumblnail image,
// It will show the big preview on the right side.
#[derive(Resource, Default)]
pub struct HoveredCard {
    pub card_id: Option<String>,
}

use crate::ui::card_view::{CardView, CardId};
use crate::ui::tier_list::{CardPreviewArea, TierContainer};
use crate::data::card_store::{CardStore, Dirty};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedCard>()
            .init_resource::<HoveredCard>()
            // .add_systems(Update, (
                // select_card,
                // move_card_to_tier,
                // show_card_to_big_preview,
            // ))
            ;
    }
}



fn _select_card(
    mut selected: ResMut<SelectedCard>,
    mut hovered: ResMut<HoveredCard>,
    query: Query<(&Interaction, &CardId), (Changed<Interaction>, With<CardView>)>,
) {
    for (interaction, card_id) in &query {
        match *interaction {
            Interaction::Pressed => {
                selected.card_id = Some(card_id.0.clone());
                info!("Selected card {}", card_id.0);
            }
            Interaction::Hovered => {
                hovered.card_id = Some(card_id.0.clone());
                // info!("Hovered card {}", card_id.0);
            }
            Interaction::None => {
                if hovered.card_id.as_ref() == Some(&card_id.0) {
                    hovered.card_id = None;
                }
            }
        }

    }
}



fn _move_card_to_tier(
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


// hovered card in HoveredCard ressource
// Should be shown in the big preview area
fn _show_card_to_big_preview(
    mut commands: Commands,
    hovered: ResMut<HoveredCard>,
    mut big_card_preview_area_querry: Query<Entity, With<CardPreviewArea>>,
    card_query: Query<(Entity, &CardId), With<CardView>>,
) {
    let Some(hovered_id) = hovered.card_id.clone() else { return; };
    let big_card_preview_area_entity = big_card_preview_area_querry.single_mut();
    match big_card_preview_area_entity {
        Ok(entity) => {
            // clear previous preview
            commands.entity(entity).detach_all_children();

            // spawn new preview
            let Some((card_entity, _)) = card_query
                .iter()
                .find(|(_, id)| id.0 == hovered_id)
            else { return; };

            commands.entity(entity).add_child(card_entity);

        }
        Err(_) => {}
    }
}
