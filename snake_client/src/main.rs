#![allow(dead_code)]
pub mod client;
//pub mod game;
//pub mod piston;
pub mod shared_structures;
//pub mod snake;
use client::Drawer;
use client::Termion;
//use game::Game;
use shared_structures::ServerMsg;
use shared_structures::ServerMsg::*;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::net::TcpStream;

fn main() {
    let srv_address = "127.0.0.1:12345";
    let mut stream: TcpStream = TcpStream::connect(srv_address)
        .expect(&format!("No server found at address: {}\n", srv_address));
    let mut client: Termion = Termion::init();

    //match recv_game
    loop {
        match listen_server(&mut stream) {
            Ok(msg) => match msg {
                Error(e) => client.draw_error(),
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
