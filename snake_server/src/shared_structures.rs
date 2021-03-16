use serde::{Deserialize, Serialize};

// Dimensions du terrain de jeu
pub const WIDTH: usize = 60;
pub const HEIGHT: usize = 20;
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
    pub nb_snakes: i32,
    // Ids of snakes controlled by bots
    pub bots: Vec<i32>,
    pub bots_difficulty: BotMovement,
    pub snakes: Vec<Snake>,
    pub food: Point,
    pub scores: Vec<PlayerStatus>,
    pub speed: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BotMovement {
    Random,
    ToTheFood,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snake {
    pub id: i32,
    pub head: Point,
    // L'index de la queue est la valeur 0
    pub body: Vec<Point>,
    pub direction: Direction,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMsg {
    Playing(Game, Vec<i32>),
    End(Option<i32>),
    Error(String),
}

pub enum ClientMsg {
    SnakeDirection(Direction),
    Leave,
}

#[derive(Debug)]
pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Quit,
}
