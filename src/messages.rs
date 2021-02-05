use std::sync::mpsc::Sender;
use std::net::TcpStream;

pub enum Command {
    Message(String),
    IncomingConnection(TcpStream),
}
