use bevy::prelude::*;

use crate::board::*;

pub fn game_over_menu(mut commands: Commands, game_over_events: EventReader<GameOverEvent>) {
    if !game_over_events.is_empty() {
        println!("Show game over menu");
    }
}