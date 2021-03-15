#![allow(dead_code)]
pub mod client;
pub mod shared_structures;
use client::Drawer;
use client::Termion;
use shared_structures::ServerMsg::*;
use shared_structures::{ServerMsg, SERVER_ADDR};
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream: TcpStream = TcpStream::connect(SERVER_ADDR)
        .expect(&format!("No server found at address: {}\n", SERVER_ADDR));
    let mut client: Termion = Termion::init();

    //match recv_game
    loop {
        match listen_server(&mut stream) {
            Ok(msg) => match msg {
                Error(_) => client.draw_error(),
                Playing(game, _) => client.draw_game(&game),
                //End(winner) => client.draw_end(winner),
                End(winner) => {
                    client.draw_end(winner);
                    break;
                }
            },
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}

fn listen_server(stream: &mut TcpStream) -> Result<ServerMsg, serde_json::error::Error> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let json = String::from_utf8_lossy(&buffer);
    let json = &json.trim_end_matches(char::from(0));
    //println!("Receiving game from server:\n {}", json);
    serde_json::from_str(&json)
}
