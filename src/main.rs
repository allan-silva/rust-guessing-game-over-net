#![feature(box_syntax)]

mod game;
mod server;

use server::Server;



fn main() {
    Server::new().run();
}
