pub mod client;
pub mod shared_structures;
use client::{max_players_on_terminal, Drawer, Termion};
use shared_structures::ServerMsg::*;
use shared_structures::{ClientMsg, ServerMsg, SERVER_ADDR};
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::mem::drop;
use std::net::TcpStream;

pub(crate) const LOG_FILE: &'static str = "log";

enum ErrorFromServer {
    MessageError(serde_json::error::Error),
    StreamError(std::io::Error),
}

fn main() {
    match get_nb_of_players() {
        Ok(nb_players) => {
            let mut stream: TcpStream = TcpStream::connect(SERVER_ADDR)
                .expect(&format!("No server found at address: {}\n", SERVER_ADDR));

            // Dis au serveur le nombre de joueurs sur ce client
            send_msg_to_server(ClientMsg::Init(nb_players), &mut stream);

            let serpents: Vec<u32>;
            match listen_server(&mut stream) {
                Ok(ServerMsg::InitAck(serpents_nb)) => serpents = serpents_nb,
                Ok(ServerMsg::Error(msg)) => error_msg_from_server(None, msg),
                _ => {
                    println!("Wrong message from server");
                    return;
                }
            }

            let mut client: Termion = Termion::init(nb_players, serpents);
            let mut playing;
            loop {
                // Reçoit les messages du serveur
                match listen_server(&mut stream) {
                    Ok(msg) => match msg {
                        ServerMsg::Error(e) => error_msg_from_server(Some(client), e),
                        Playing(game, _) => {
                            playing = true;
                            client.draw_game(&game)
                        }
                        End(winner) => {
                            client.draw_end(winner);
                            playing = false;
                        }
                        _ => panic!("Should not happen"),
                    },
                    Err(ErrorFromServer::MessageError(e)) => match e.classify() {
                        serde_json::error::Category::Eof => error_msg_from_server(
                            Some(client),
                            String::from("Server has disconnected"),
                        ),
                        _ => error_msg_from_server(
                            Some(client),
                            format!("MessageError: {}", e.to_string()),
                        ),
                    },
                    Err(ErrorFromServer::StreamError(e)) => match e.kind() {
                        std::io::ErrorKind::ConnectionReset => error_msg_from_server(
                            Some(client),
                            String::from("Server has disconnected"),
                        ),
                        std::io::ErrorKind::UnexpectedEof => error_msg_from_server(
                            Some(client),
                            String::from("Server has disconnected"),
                        ),

                        _ => error_msg_from_server(Some(client), String::from("toto")),
                    },
                }
                // Récupère les touches des joueurs
                if playing {
                    let inputs = client.get_inputs();
                    if let ClientMsg::Leave(_) = inputs {
                        return;
                    }
                    send_msg_to_server(inputs, &mut stream);
                }
            }
        }
        Err(msg) => println!("ERROR: {}", msg),
    }
}

fn get_nb_of_players() -> Result<u32, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if let Ok(nb_players) = args[1].parse::<u32>() {
            if nb_players <= max_players_on_terminal() {
                return Ok(nb_players);
            } else {
                println!("{} {}", nb_players, max_players_on_terminal());
                return Err(format!("Maximum {} players", max_players_on_terminal()));
            }
        } else {
            return Err(String::from("Arguments must be integers"));
        }
    }
    Ok(1)
}

fn listen_server(stream: &mut TcpStream) -> Result<ServerMsg, ErrorFromServer> {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let json = String::from_utf8_lossy(&buffer);
            let json = &json.trim_end_matches(char::from(0));
            log_in_file(String::from(*json));
            match serde_json::from_str(&json) {
                Ok(msg) => Ok(msg),
                Err(e) => Err(ErrorFromServer::MessageError(e)),
            }
        }
        // TODO handle properly
        Err(e) => Err(ErrorFromServer::StreamError(e)),
    }
}

fn send_msg_to_server(msg: ClientMsg, stream: &mut TcpStream) {
    let json = serde_json::to_string(&msg).unwrap();
    stream.write(json.as_bytes()).unwrap();
    stream.flush().unwrap()
}

fn error_msg_from_server(client: Option<Termion>, error_msg: String) -> ! {
    if let Some(client) = client {
        drop(client);
    }
    println!("[SERVER ERROR] {}", error_msg);
    std::process::exit(1)
}

fn log_in_file(mut s: String) {
    s.push('\n');
    if let Ok(mut file) = OpenOptions::new().append(true).open(LOG_FILE) {
        file.write(s.as_bytes()).unwrap();
    }
}
