use crate::messages::ServerCommand;
use crate::protocol::{Connection, ProtocolHeader, FRAME_END};
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use uuid::Uuid;

struct Player {
    id: String,
    name: String,
    registered_at: DateTime<Local>,
}

struct PlayerSession {
    player: Player,
    thread: JoinHandle<()>,
}

struct ServerStats {
    total_players: u32,
    total_game_sessions: u32,
}

struct GameSession {}

struct GameMessage {}

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
        self.listen_server_commands(main_rx);
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

    fn listen_server_commands(&mut self, receiver: Receiver<ServerCommand>) {
        println!("Server started, waiting for connections");

        for server_command in receiver {
            match server_command {
                ServerCommand::AcceptedConnection(stream, main_tx) => {
                    match self.start_connection(stream, *main_tx) {
                        Ok(_) => println!("Successfully negotiated"),
                        Err(err_message) => println!("Negotiation error: {}", err_message),
                    }
                }
                ServerCommand::Message(message) => println!("{}", message),
            }
        }
    }

    fn start_connection(
        &mut self,
        stream: TcpStream,
        main_tx: Sender<ServerCommand>,
    ) -> Result<(), String> {
        let mut connection = Connection::new(stream, main_tx);
        connection.start()
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::protocol::FRAME_END;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpStream;

    #[test]
    fn test_send_protocol_header() {
        let mut stream = TcpStream::connect("127.0.0.1:4242").unwrap();
        let protocol_header = "GG010".as_bytes();
        stream.write(&protocol_header).unwrap();

        let mut response_buffer = Vec::new();

        let mut reader = BufReader::new(&stream);
        reader.read_until(FRAME_END, &mut response_buffer).unwrap();
        response_buffer.pop();

        println!("{:?}", response_buffer);
        println!(
            "{:?}",
            String::from_utf8(response_buffer[4..].to_vec()).unwrap()
        );
        assert_eq!(1, 0);
    }
}
