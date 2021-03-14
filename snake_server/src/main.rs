use crate::shared_structures::*;
use game::TurnOutcome;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
pub mod game;
pub mod shared_structures;
pub mod snake;

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

// Information supplémentaire sur une partie qui n'est pas partagé
// aux clients
struct ServerGame {
    game: Game,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
    let mut game = shared_structures::Game::init(4);

    let mut players: Vec<&TcpStream> = vec![];
    let (sock, addr) = listener.accept().expect("Connection failed");
    log!("New connection from {}", addr);
    players.push(&sock);
    play(&mut game, &mut players);
}

// Lance une partie de Snake
fn play(game: &mut Game, players: &mut Vec<&TcpStream>) {
    // TODO we should not clone
    send_msg_to_clients(ServerMsg::Playing(game.clone(), vec![]), players);
    loop {
        sleep(Duration::from_millis(game.speed));

        // on fait avancer le jeu d'un tour
        let turn_outcome = game.turn(vec![vec![]]);

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
fn send_msg_to_clients(msg: ServerMsg, clients: &mut Vec<&TcpStream>) {
    log!("Sending message to clients:\n {:?}", msg);
    for stream in clients.iter_mut() {
        let json = serde_json::to_string(&msg).unwrap();
        stream.write(json.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
