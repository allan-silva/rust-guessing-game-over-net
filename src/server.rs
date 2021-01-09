use std::collections::HashMap;
use std::thread;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};

use crate::game::{Game, Player};

pub struct GameData {
    players: HashMap<String, Player>,
    games: HashMap<String, Game<'static>>,
}

// impl Deref for GameData<'a> {
//     type Target = HashMap<String, Player>;

//     fn deref(&self) -> &Self::Target {
//         &self.players
//     }
// }

// impl DerefMut for GameData {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.players
//     }
// }

impl GameData {
    pub fn new() -> GameData {
        GameData {
            players: HashMap::new(),
            games: HashMap::new(),
        }
    }
}

pub struct Server {
    game_data: GameData,
}

impl Server {
    pub fn new() -> Server {
        Server {
            game_data: GameData::new()
        }
    }

    pub fn run(self) {
        println!("Starting server");

        let mut game_data = self.game_data;

        thread::spawn(move || {
            let player_one = Player::new("Chico".to_string());
            let player_two = Player::new("Paloma".to_string());

            game_data.players.insert(player_one.id.clone(), player_one);
            game_data.players.insert(player_two.id.clone(), player_two);

            println!("Running");
        }).join();
    }
}
