use serde::{Deserialize, Serialize};

pub const WIDTH: usize = 60;
pub const HEIGHT: usize = 20;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlayerStatus {
    Loser,
    Leaver,
    Player(i32), //Playing and current score
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub nb_players: i32,
    pub snakes: Vec<Snake>,
    pub food: Point,
    pub scores: Vec<PlayerStatus>,
    pub speed: u64,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
}

pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Quit,
}
