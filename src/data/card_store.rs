use bevy::prelude::*;

use crate::data::card::Card;


use std::fs::File;
use std::io::BufReader;


#[derive(Resource, Default)]
pub struct CardStore {
    pub cards: Vec<Card>,
}

impl CardStore {
    pub fn load_from_json(&mut self, path: &str) {
        let file = File::open(path)
            .expect("Failed to open cards.json");

        let reader = BufReader::new(file);

        self.cards = serde_json::from_reader(reader)
            .expect("Invalid cards.json");
    }
}


// Dirty means the CardStore has unsaved changes
#[derive(Resource, Default)]
pub struct Dirty(pub bool);

