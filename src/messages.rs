use std::sync::mpsc::Sender;
use std::net::TcpStream;

pub enum ServerCommand {
    Message(String),
    AcceptedConnection(TcpStream, Box<Sender<ServerCommand>>),
}
