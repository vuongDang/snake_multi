use game::TurnOutcome;
use shared_structures::*;
use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
pub mod game;
pub mod shared_structures;
pub mod snake;

const NB_SNAKES: i32 = 4;
const NB_BOT: i32 = 3;
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
    let game = match setup_game() {
        Ok(g) => g,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
    let nb_humans = game.nb_snakes - (game.bots.len() as i32);
    log!(
        "Game created: snakes: {} - human players: {} - bots: {}",
        game.nb_snakes,
        nb_humans,
        game.bots.len()
    );

    let mut players: Vec<TcpStream> = vec![];
    log!("Server waiting for connection");
    while players.len() < nb_humans as usize {
        let (sock, addr) = listener.accept().expect("Connection failed");
        log!("New connection from {}", addr);
        players.push(sock);
    }
    play(game, players);
}

// Crée une partie en fonction des arguments fournis
fn setup_game() -> Result<Game, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if let Ok(nb_snakes) = args[1].parse() {
            if args.len() > 2 {
                if let Ok(nb_bots) = args[2].parse() {
                    return Game::init(nb_snakes, nb_bots);
                } else {
                    return Err(format!("Arguments should be integers: {}", args[2]));
                }
            } else {
                return Game::init(nb_snakes, nb_snakes - 1);
            }
        } else {
            return Err(format!("Arguments should be integers: {}", args[1]));
        }
    }
    Game::init(NB_SNAKES, NB_BOT)
}

// Lance une partie de Snake
fn play(mut game: Game, mut players: Vec<TcpStream>) {
    // TODO we should not clone
    send_msg_to_clients(ServerMsg::Playing(game.clone(), vec![]), &mut players);
    loop {
        sleep(Duration::from_millis(game.speed));

        // on fait avancer le jeu d'un tour
        let turn_outcome = game.turn(vec![vec![]]);

        // on check si la partie est finie
        match turn_outcome {
            // On envoie la partie avec les perdants éventuels
            TurnOutcome::Playing(losers) => {
                send_msg_to_clients(ServerMsg::Playing(game.clone(), losers), &mut players)
            }
            TurnOutcome::End(winner) => {
                send_msg_to_clients(ServerMsg::End(winner), &mut players);
                log!("Game ended");
                return;
            }
        }
    }
}

// Envoie un message aux différents clients
fn send_msg_to_clients(msg: ServerMsg, clients: &mut Vec<TcpStream>) {
    // For debugging purpose
    //log!("Sending message to clients:\n {:?}", msg);
    for stream in clients.iter_mut() {
        let json = serde_json::to_string(&msg).unwrap();
        stream.write(json.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
