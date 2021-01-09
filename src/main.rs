#![feature(box_syntax)]

mod game;
mod server;

use server::Server;


fn main() {
    let server = Server::new();
    server.run();
}
