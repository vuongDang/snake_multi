use game::TurnOutcome;
use shared_structures::*;
use std::env;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
pub mod game;
pub mod shared_structures;
pub mod snake;

// Default number of snakes and bots
const NB_SNAKES: u32 = 4;
const NB_BOT: u32 = 3;
// Une macro pour afficher des messages de log de la forme
// [LOG] ...
#[macro_export]
macro_rules! log {
    ($($y:expr),+) => {
        print!("[LOG] ");
        println!($($y,)*);
        println!();
    }
}

macro_rules! error {
    ($($y:expr),+) => {
        print!("[ERROR] ");
        println!($($y,)*);
        println!();
    }
}

fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).unwrap();
    log!("Server address: {}", SERVER_ADDR);
    let (nb_snakes, nb_bots) = match get_args() {
        Ok(v) => v,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
    let nb_humans = nb_snakes - nb_bots;

    // Store every client connected with the players numbers associated
    let mut clients: Vec<(Vec<u32>, Option<TcpStream>)> = vec![];
    let mut players_pending: u32 = 0;
    log!("Server waiting for connection");
    while players_pending < nb_humans {
        let (mut sock, addr) = listener.accept().expect("Connection failed");
        // On demande un message "Init" aux clients
        if let ClientMsg::Init(nb_players) = listen_to_client((&mut vec![], Some(&mut sock))) {
            // Trop de joueurs
            if players_pending as u32 + nb_players > nb_humans as u32 {
                log!("Client {} has too many players for this game", addr);
                if let Ok(_) = send_msg_to_client(
                    &ServerMsg::Error(format!("Too many players for this game")),
                    &mut sock,
                ) {
                    sock.shutdown(Shutdown::Both).unwrap();
                }
            } else {
                // Attribue les numéros de serpents aux joueurs
                let players_numbers: Vec<u32> =
                    (players_pending + 1..players_pending + 1 + nb_players).collect();
                players_pending = players_pending + nb_players;
                // Envoie les numéros des serpents aux joueurs
                if let Ok(_) =
                    send_msg_to_client(&ServerMsg::InitAck(players_numbers.clone()), &mut sock)
                {
                    log!("New connection from {}", addr);
                    clients.push((players_numbers, Some(sock)));
                }
            }
        } else {
            log!("Client {} did not send its number of players", addr);
            sock.shutdown(Shutdown::Both).unwrap();
        }
    }

    // We restart game after 3 seconds
    loop {
        match Game::init(nb_snakes, nb_bots) {
            Ok(game) => {
                log!(
                    "Game created: snakes: {} - human players: {} - bots: {}",
                    nb_snakes,
                    nb_humans,
                    nb_bots
                );

                play(game, &mut clients);
            }
            Err(msg) => {
                error!("{}", msg);
                return;
            }
        }
        sleep(Duration::from_secs(3));
    }
}

// Crée une partie en fonction des arguments fournis
fn get_args() -> Result<(u32, u32), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if let Ok(nb_snakes) = args[1].parse() {
            if args.len() > 2 {
                if let Ok(nb_bots) = args[2].parse() {
                    return Ok((nb_snakes, nb_bots));
                } else {
                    return Err(format!("Arguments should be integers: {}", args[2]));
                }
            } else {
                return Ok((nb_snakes, nb_snakes - 1));
            }
        } else {
            return Err(format!("Arguments should be integers: {}", args[1]));
        }
    }
    Ok((NB_SNAKES, NB_BOT))
}

// Lance une partie de Snake
fn play(mut game: Game, players: &mut Vec<(Vec<u32>, Option<TcpStream>)>) {
    // TODO we should not clone
    send_msg_to_clients(ServerMsg::Playing(game.clone(), vec![]), players);
    loop {
        sleep(Duration::from_millis(game.speed));

        let players_inputs = listen_to_clients(players);

        // on fait avancer le jeu d'un tour
        let turn_outcome = game.turn(players_inputs);

        // on check si la partie est finie
        match turn_outcome {
            // On envoie la partie avec les perdants éventuels
            TurnOutcome::Playing(losers) => {
                send_msg_to_clients(ServerMsg::Playing(game.clone(), losers), players)
            }
            TurnOutcome::End(winner) => {
                send_msg_to_clients(ServerMsg::End(winner), players);
                log!("Game ended");
                return;
            }
        }
    }
}

// Envoie un message aux différents clients
fn send_msg_to_clients(msg: ServerMsg, clients: &mut Vec<(Vec<u32>, Option<TcpStream>)>) {
    // For debugging purpose
    //log!("Sending message to clients:\n {:?}", msg);
    for (_, opt_stream) in clients.iter_mut() {
        if let Some(stream) = opt_stream {
            match send_msg_to_client(&msg, stream) {
                Ok(_) => stream.flush().unwrap(),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => {
                        *opt_stream = None;
                    }
                    _ => panic!("No sé que pasa"),
                },
            };
        }
    }
}

fn send_msg_to_client(msg: &ServerMsg, client: &mut TcpStream) -> Result<usize, std::io::Error> {
    let json = serde_json::to_string(msg).unwrap();
    client.write(json.as_bytes())
}

fn listen_to_clients(clients: &mut Vec<(Vec<u32>, Option<TcpStream>)>) -> Vec<ClientMsg> {
    let mut v = vec![];
    for (players_nb, stream) in clients.iter_mut() {
        let json = listen_to_client((players_nb, stream.as_mut()));
        v.push(json);
    }
    v
}

fn listen_to_client(client: (&mut Vec<u32>, Option<&mut TcpStream>)) -> ClientMsg {
    let mut buffer = [0; 1024];
    let stream = client.1;
    if let Some(stream) = stream {
        stream.read(&mut buffer).unwrap();
        let json = String::from_utf8_lossy(&buffer);
        let json = &json.trim_end_matches(char::from(0));
        if let Ok(json) = serde_json::from_str(&json) {
            return json;
        } else {
            if let Ok(addr) = stream.peer_addr() {
                error!("Client {} has sent erronous data:\n{}", addr, json);
            } else {
                error!("Client has sent erronous data:\n{}", json);
            }
        }
    }
    ClientMsg::Leave(client.0.len() as u32)
}
