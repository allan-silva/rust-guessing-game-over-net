use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::io::prelude::*;

use crate::game::{Game, Player};

#[derive(Debug)]
pub struct GameManager {
    players: HashMap<String, Arc<RwLock<Player>>>,
    games: HashMap<String, RwLock<Game<'static>>>,
    call_nr: Mutex<u8>,
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            players: HashMap::new(),
            games: HashMap::new(),
            call_nr: Mutex::new(0),
        }
    }

    fn handle_call(&self, stream: &mut TcpStream) -> String {
        let contents = String::from("Hi!");
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        *self.call_nr.lock().unwrap() += 1;

        format!("Call handled({})", self.call_nr.lock().unwrap())
    }

    fn create_player(&mut self, name: &str) {
        let player = Player::new(String::from(name));
        self.players.insert(String::from(name), Arc::new(RwLock::new(player)));
    }
}

pub struct Server {
    game_data: Arc<RwLock<GameManager>>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            game_data: Arc::new(RwLock::new(GameManager::new())),
        }
    }

    pub fn run(self) {
        println!("Server is running");

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let game_manager = self.game_data.clone();

            thread::spawn(move || {
                match game_manager.write() {
                    Ok(ref manager) => {
                        let status = manager.handle_call(&mut stream.unwrap());
                        println!("{}", status);
                    },
                    Err(_) => println!("not GET The lock"),
                };
            });
        }
    }
}
