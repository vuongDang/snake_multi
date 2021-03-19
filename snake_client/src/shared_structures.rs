use serde::{Deserialize, Serialize};

// Dimensions du terrain de jeu
pub const WIDTH: usize = 60;
pub const HEIGHT: usize = 21;
pub const SERVER_ADDR: &str = "127.0.0.1:12345";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlayerStatus {
    Loser,
    Leaver,
    Player(i32), //Playing and current score
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    // Number of snakes that will be in the game
    pub nb_snakes: u32,
    // If None means that the Snake has lost
    pub snakes: Vec<Option<Snake>>,
    // Ids of snakes controlled by bots
    pub bots: Vec<u32>,
    pub bots_difficulty: BotMovement,
    pub points_to_win: u32,

    pub food: Point,
    pub scores: Vec<PlayerStatus>,
    pub speed: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BotMovement {
    Random,
    ToTheFood,
    Survival,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Structure du serpent
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Snake {
    pub id: u32,
    pub head: Point,
    // L'index de la queue est la valeur 0
    pub body: Vec<Point>,
    pub direction: Direction,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMsg {
    InitAck(Vec<u32>),
    Playing(Game, Vec<u32>),
    End(Option<u32>),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMsg {
    // Dis au serveur le nombre de joueurs sur le client
    Init(u32),
    SnakeDirection(Vec<Option<Direction>>),
    Leave(u32),
}
