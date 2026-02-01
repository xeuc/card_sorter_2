use bevy::prelude::*;
use serde::{Serialize, Deserialize};

const TIER_LIST_TIERS: [&str; 5] = ["S", "A", "B", "C", "D"];
const NUMBER_OF_TIERS: usize = TIER_LIST_TIERS.len();

#[derive(Clone, Serialize, Deserialize, Component)]
pub enum Tier {
    S, A, B, C, D
}


impl Tier {
    pub const ORDER: &'static [Tier] = &[
        Tier::S, Tier::A, Tier::B, Tier::C, Tier::D,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Tier::S => "S",
            Tier::A => "A",
            Tier::B => "B",
            Tier::C => "C",
            Tier::D => "D",
        }
    }

    pub fn color(self) -> Color {
        match self {
            // Tier::S => Color::srgb(0.9, 0.2, 0.2),
            // Tier::A => Color::srgb(0.9, 0.5, 0.2),
            // Tier::B => Color::srgb(0.9, 0.9, 0.2),
            // Tier::C => Color::srgb(0.2, 0.8, 0.3),
            // Tier::D => Color::srgb(0.2, 0.6, 0.9),

            Tier::S => Color::hsl(360. * 0 as f32 / NUMBER_OF_TIERS as f32, 0.95, 0.7),
            Tier::A => Color::hsl(360. * 1 as f32 / NUMBER_OF_TIERS as f32, 0.95, 0.7),
            Tier::B => Color::hsl(360. * 2 as f32 / NUMBER_OF_TIERS as f32, 0.95, 0.7),
            Tier::C => Color::hsl(360. * 3 as f32 / NUMBER_OF_TIERS as f32, 0.95, 0.7),
            Tier::D => Color::hsl(360. * 4 as f32 / NUMBER_OF_TIERS as f32, 0.95, 0.7),
            
        }
    }
}


#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub file_name: String,
    pub path: String,
    pub tier: Option<Tier>,
}
