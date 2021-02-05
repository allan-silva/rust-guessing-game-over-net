#![feature(box_syntax)]

mod game;
mod server;
mod protocol;
mod messages;

use server::Server;



fn main() {
    Server::new().run();
}
