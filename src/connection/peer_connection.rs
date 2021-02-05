use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};

use crate::messages::Command;

pub struct PeerConnection {
    stream: TcpStream,
    connection_tx: Sender<Command>,
}
