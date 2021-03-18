//use crate::game::*;
//use crate::snake::{Direction, Snake, Point};
use crate::shared_structures::ClientMsg::*;
use crate::shared_structures::Direction::*;
use crate::shared_structures::*;
use crate::shared_structures::{Game, PlayerStatus, Point, Snake};
use crate::{log_in_file, LOG_FILE};
use std::fs::File;
use std::io::{stdout, Read, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, clear, color, cursor, AsyncReader};

pub const FOOD_CHAR: char = 'Ծ';
pub const MAX_PLAYERS_ON_1_TERMINAL: u32 = 2;
const MARGIN_AFTER_FIELD: u16 = 4;
const MARGIN_TOP: u16 = 1;
const PLAYER_1_CONTROLS: [u8; 4] = [b'q', b's', b'd', b'z'];
const PLAYER_2_CONTROLS: [u8; 4] = [b'j', b'i', b'k', b'l'];

const PLAYERS_CONTROLS: [[u8; 4]; 2] = [PLAYER_1_CONTROLS, PLAYER_2_CONTROLS];
const PLAYERS_COLORS: [&dyn color::Color; 4] =
    [&color::Red, &color::Blue, &color::Green, &color::Yellow];

pub trait Drawer {
    fn init(nb_players: u32, serpents: Vec<u32>) -> Self;
    fn draw_game(&mut self, game: &Game);
    fn draw_error(&mut self);
    fn draw_end(&mut self, winner: Option<u32>);
}

pub struct Termion {
    nb_players: u32,
    snakes_nb: Vec<u32>,
    stdin: AsyncReader,
    stdout: RawTerminal<Stdout>,
}

impl Drawer for Termion {
    fn init(nb_players: u32, serpents: Vec<u32>) -> Self {
        let stdin = async_stdin();
        let stdout = stdout().into_raw_mode().unwrap();
        File::create(LOG_FILE).unwrap();
        Termion {
            nb_players,
            snakes_nb: serpents,
            stdin,
            stdout,
        }
    }

    fn draw_game(&mut self, game: &Game) {
        self.draw_field(WIDTH as u16, HEIGHT as u16);
        for snake in game.snakes.iter() {
            if let Some(snake) = snake {
                self.draw_snake(snake);
            }
        }
        self.draw_food(&game.food);
        let current_y = self.draw_scores(&game);
        let _current_y = self.draw_instructions(current_y, game.points_to_win);
        self.cursor_at_bottom();
    }

    // Met le curseur en bas
    fn draw_end(&mut self, winner: Option<u32>) {
        match winner {
            None => self.draw_draw(),
            Some(winner) => self.draw_winner(winner),
        }
        self.cursor_at_bottom();
    }

    fn draw_error(&mut self) {}
}

impl Termion {
    pub fn get_inputs(&mut self) -> ClientMsg {
        //On lit 10 caractères
        let mut buffer = [0; 10];
        self.stdin.read(&mut buffer).unwrap();
        let mut v = vec![None; self.nb_players as usize];
        for c in buffer.iter() {
            match c {
                // Premier serpent
                b'q' => v[0] = Some(Left),
                b's' => v[0] = Some(Down),
                b'z' => v[0] = Some(Up),
                b'd' => v[0] = Some(Right),

                // Deuxième serpent
                b'j' => v[1] = Some(Left),
                b'k' => v[1] = Some(Down),
                b'i' => v[1] = Some(Up),
                b'l' => v[1] = Some(Right),

                // Quitter la partie avec la touch Esc
                27 => {
                    return Leave(self.nb_players);
                }
                _ => (),
            }
        }
        log_in_file(format!("{:?}\n", v));
        SnakeDirection(v)
    }

    fn draw_instructions(&mut self, mut current_y: u16, points_to_win: u32) -> u16 {
        current_y += 2;
        write!(
            self.stdout,
            "{}#How to win:",
            cursor::Goto(WIDTH as u16 + MARGIN_AFTER_FIELD, current_y)
        )
        .unwrap();
        current_y += 1;
        write!(
            self.stdout,
            "{}- Last survivor",
            cursor::Goto(WIDTH as u16 + MARGIN_AFTER_FIELD, current_y)
        )
        .unwrap();
        current_y += 1;
        write!(
            self.stdout,
            "{}- First to reach {}",
            cursor::Goto(WIDTH as u16 + MARGIN_AFTER_FIELD, current_y),
            points_to_win
        )
        .unwrap();

        current_y += 2;
        write!(
            self.stdout,
            "{}# Controls",
            cursor::Goto(WIDTH as u16 + MARGIN_AFTER_FIELD, current_y)
        )
        .unwrap();
        current_y += 1;
        write!(
            self.stdout,
            "{}Quit: \"Esc\"",
            cursor::Goto(WIDTH as u16 + MARGIN_AFTER_FIELD, current_y)
        )
        .unwrap();

        for player in self.snakes_nb.iter() {
            let player_index = (*player - 1) as usize;
            current_y += 1;
            write!(
                self.stdout,
                "{}{}Player {}: {:?}{}",
                cursor::Goto(WIDTH as u16 + MARGIN_AFTER_FIELD, current_y),
                color::Fg(PLAYERS_COLORS[player_index]),
                player,
                PLAYERS_CONTROLS[player_index]
                    .iter()
                    .fold(String::from(""), |acc, c| format!("{}{}", acc, *c as char)),
                color::Fg(color::Reset)
            )
            .unwrap();
        }

        self.stdout.flush().unwrap();
        current_y
    }

    fn draw_field(&mut self, width: u16, height: u16) {
        // On écrit dans notre console statique dans l'ordre
        // - on efface tout le contenu
        // - place le curseur au début de la première ligne
        // - la couleur du ForeGround choisie est bleu
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            color::Fg(color::Blue)
        )
        .unwrap();
        // On appelle flush() pour forcer les modifications dans
        // stdout
        self.stdout.flush().unwrap();

        // Affichage de l'espace de jeu
        for i in 0..height {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(1, i),
                Termion::BORDER_CHAR
            )
            .unwrap();
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(width, i),
                Termion::BORDER_CHAR
            )
            .unwrap();
        }

        let line = Termion::BORDER_CHAR.to_string().repeat(width as usize);
        write!(self.stdout, "{}{}", cursor::Goto(1, 1), line).unwrap();
        write!(self.stdout, "{}{}", cursor::Goto(1, height), line).unwrap();

        // Remet à jour la couleur utilisé
        write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn draw_snake(&mut self, snake: &Snake) {
        let snake_index = snake.id - 1;
        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(snake.head.x, snake.head.y),
            color::Fg(PLAYERS_COLORS[snake_index as usize]),
            Termion::head_char(snake.direction.clone())
        )
        .unwrap();

        for i in 0..snake.body.len() {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(snake.body[i].x, snake.body[i].y),
                Termion::BODY_CHAR
            )
            .unwrap();
        }
        write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn draw_food(&mut self, food: &Point) {
        write!(
            self.stdout,
            "{}{}{}{}",
            cursor::Goto(food.x, food.y),
            color::Fg(color::Red),
            FOOD_CHAR,
            color::Fg(color::Reset)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn cursor_at_bottom(&mut self) {
        write!(self.stdout, "{}", cursor::Goto(0, HEIGHT as u16 + 1)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn draw_draw(&mut self) {
        let w: u16 = WIDTH as u16;
        let h: u16 = HEIGHT as u16;
        write!(
            self.stdout,
            "{}-------------------",
            cursor::Goto((w / 2) - 10, h / 2 - 1)
        )
        .unwrap();
        write!(
            self.stdout,
            "{}|       DRAW!      |",
            cursor::Goto((w / 2) - 10, h / 2)
        )
        .unwrap();
        write!(
            self.stdout,
            "{}-------------------",
            cursor::Goto((w / 2) - 10, h / 2 + 1)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn draw_winner(&mut self, winner: u32) {
        let w: u16 = WIDTH as u16;
        let h: u16 = HEIGHT as u16;
        write!(
            self.stdout,
            "{}------------------------",
            cursor::Goto((w / 2) - 10, h / 2 - 1)
        )
        .unwrap();
        write!(
            self.stdout,
            "{}|    Player {} WINS!    |",
            cursor::Goto((w / 2) - 10, h / 2),
            winner
        )
        .unwrap();
        write!(
            self.stdout,
            "{}------------------------",
            cursor::Goto((w / 2) - 10, h / 2 + 1)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn draw_scores(&mut self, game: &Game) -> u16 {
        let mut current_y = MARGIN_TOP;

        fn score_msg(status: &PlayerStatus) -> String {
            match status {
                PlayerStatus::Leaver => "Left the game".to_owned(),
                PlayerStatus::Loser => "Lost".to_owned(),
                PlayerStatus::Player(score) => score.to_string(),
            }
        }

        for snake in self.snakes_nb.iter() {
            current_y += 1;
            write!(
                self.stdout,
                "{}You are {}Player {}{}",
                cursor::Goto(WIDTH as u16 + MARGIN_AFTER_FIELD, current_y),
                color::Fg(PLAYERS_COLORS[*snake as usize - 1]),
                snake,
                color::Fg(color::Reset)
            )
            .unwrap();
        }
        current_y += 1;

        for (i, score) in game.scores.iter().enumerate() {
            current_y += 1;
            write!(
                self.stdout,
                "{}{}Score {}: {}{}",
                cursor::Goto(WIDTH as u16 + MARGIN_AFTER_FIELD, current_y),
                color::Fg(PLAYERS_COLORS[i]),
                i + 1,
                score_msg(score),
                color::Fg(color::Reset)
            )
            .unwrap();
        }

        self.stdout.flush().unwrap();
        current_y
    }
}

impl Termion {
    const BORDER_CHAR: char = '#';
    const BODY_CHAR: char = '▪';
    const HEAD_UP: char = '▲';
    const HEAD_DOWN: char = '▼';
    const HEAD_LEFT: char = '◀';
    const HEAD_RIGHT: char = '▶';

    fn head_char(direction: Direction) -> char {
        match direction {
            Direction::Up => Termion::HEAD_UP,
            Direction::Down => Termion::HEAD_DOWN,
            Direction::Left => Termion::HEAD_LEFT,
            Direction::Right => Termion::HEAD_RIGHT,
        }
    }
}
