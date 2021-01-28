use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use uuid::Uuid;

struct Player {
    id: String,
    name: String,
    registered_at: DateTime<Local>,
}

struct ServerStats {
    total_players: u32,
    total_game_sessions: u32,
}

struct GameSession {}

struct GameMessage {}

enum ServerCommand {
    AcceptedConnection(TcpStream, Box<Sender<ServerCommand>>),
}

pub struct Server {
    players: HashMap<String, Player>,
    sessions: HashMap<String, GameSession>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            sessions: HashMap::new(),
            players: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let (main_tx, main_rx) = channel::<ServerCommand>();
        self.start_server(main_tx);
        self.listen_game_commands(main_rx);
    }

    fn start_server(&self, main_tx: Sender<ServerCommand>) {
        thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:4242").unwrap();
            for stream in listener.incoming() {
                let stream = stream.unwrap().try_clone().unwrap();
                main_tx
                    .send(ServerCommand::AcceptedConnection(
                        stream,
                        box main_tx.clone(),
                    ))
                    .unwrap();
            }
        });
    }

    fn listen_game_commands(&mut self, receiver: Receiver<ServerCommand>) {
        println!("Server started, waiting for connections");

        for server_command in receiver {
            match server_command {
                ServerCommand::AcceptedConnection(stream, main_tx) => {
                    match self.register_user(stream, *main_tx) {
                        Ok(player_id) => println!("Player registred {}", player_id),
                        Err(err_message) => println!("User registration error: {}", err_message),
                    }
                }
            }
        }
    }

    fn register_user(
        &mut self,
        stream: TcpStream,
        main_tx: Sender<ServerCommand>,
    ) -> Result<String, String> {
        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();

        match reader.read_until(4, &mut buffer) {
            Ok(_) => {
                let player_id = Uuid::new_v4().to_string();
                let player = Player {
                    id: player_id.clone(),
                    name: String::from(""),
                    registered_at: Local::now(),
                };
                println!("{}", String::from_utf8(buffer).unwrap());
                self.players.insert(player_id.clone(), player);
                Ok(player_id)
            }
            Err(e) => {
                stream.shutdown(Shutdown::Both).unwrap();
                Err(e.raw_os_error().unwrap().to_string())
            }
        }
    }
}
