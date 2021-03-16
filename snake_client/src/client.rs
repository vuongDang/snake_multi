//use crate::game::*;
//use crate::snake::{Direction, Snake, Point};
use crate::shared_structures::ClientMsg::*;
use crate::shared_structures::Direction::*;
use crate::shared_structures::*;
use crate::shared_structures::{Game, PlayerStatus, Point, Snake};
use std::fs::{File, OpenOptions};
use std::io::{stdout, Read, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, clear, color, cursor, AsyncReader};

pub const FOOD_CHAR: char = 'Ծ';
const LOG_FILE: &'static str = "log";

pub trait Drawer {
    fn init(nb_players: u32) -> Self;
    fn draw_game(&mut self, game: &Game);
    fn draw_error(&mut self);
    fn draw_end(&mut self, winner: Option<i32>);
}

//fn draw_field(&mut self, width: u16, height: u16);
//fn draw_snake(&mut self, snake: &Snake);
//fn draw_food(&mut self, food: &Point);
//fn draw_scores(&mut self, scores: &Vec<i32>);
//fn draw_results(&mut self, losers: Vec<i32>, scores: &Vec<i32>);
//fn get_inputs(&mut self) -> Vec<Option<Input>>;

pub struct Termion {
    nb_players: u32,
    stdin: AsyncReader,
    stdout: RawTerminal<Stdout>,
}

impl Drawer for Termion {
    fn init(nb_players: u32) -> Self {
        let stdin = async_stdin();
        let stdout = stdout().into_raw_mode().unwrap();
        File::create(LOG_FILE).unwrap();
        Termion {
            nb_players,
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
        self.draw_scores(&game);
        self.draw_food(&game.food);
    }

    fn draw_end(&mut self, winner: Option<i32>) {
        match winner {
            None => self.draw_draw(),
            Some(winner) => self.draw_winner(winner),
        }
        self.cursor_at_bottom();
    }

    fn draw_error(&mut self) {}
}

impl Termion {
    pub fn get_inputs(&mut self) -> Vec<ClientMsg> {
        //On lit 10 caractères
        let mut buffer = [0; 10];
        self.stdin.read(&mut buffer).unwrap();
        let mut v = vec![SnakeDirection(None); self.nb_players as usize];
        for c in buffer.iter() {
            match c {
                // Premier serpent
                b'q' => v[0] = SnakeDirection(Some(Left)),
                b's' => v[0] = SnakeDirection(Some(Down)),
                b'z' => v[0] = SnakeDirection(Some(Up)),
                b'd' => v[0] = SnakeDirection(Some(Right)),

                // Deuxième serpent
                b'j' => v[1] = SnakeDirection(Some(Left)),
                b'k' => v[1] = SnakeDirection(Some(Down)),
                b'i' => v[1] = SnakeDirection(Some(Up)),
                b'l' => v[1] = SnakeDirection(Some(Right)),

                // Quitter la partie avec la touch Esc
                27 => {
                    return vec![Leave; self.nb_players as usize];
                }
                _ => (),
            }
        }
        log_in_file(format!("{:?}", v));
        v
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
        match snake.id {
            1 => write!(
                self.stdout,
                "{}{}{}",
                cursor::Goto(snake.head.x, snake.head.y),
                color::Fg(color::Red),
                Termion::head_char(snake.direction.clone())
            )
            .unwrap(),
            2 => write!(
                self.stdout,
                "{}{}{}",
                cursor::Goto(snake.head.x, snake.head.y),
                color::Fg(color::Blue),
                Termion::head_char(snake.direction.clone())
            )
            .unwrap(),
            3 => write!(
                self.stdout,
                "{}{}{}",
                cursor::Goto(snake.head.x, snake.head.y),
                color::Fg(color::Green),
                Termion::head_char(snake.direction.clone())
            )
            .unwrap(),
            4 => write!(
                self.stdout,
                "{}{}{}",
                cursor::Goto(snake.head.x, snake.head.y),
                color::Fg(color::Yellow),
                Termion::head_char(snake.direction.clone())
            )
            .unwrap(),
            _ => unimplemented!(),
        };

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

    fn draw_winner(&mut self, winner: i32) {
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

    fn draw_scores(&mut self, game: &Game) {
        let starting_height = 4;

        fn score_msg(status: &PlayerStatus) -> String {
            match status {
                PlayerStatus::Leaver => "Left the game".to_owned(),
                PlayerStatus::Loser => "Lost".to_owned(),
                PlayerStatus::Player(score) => score.to_string(),
            }
        }

        //Player1
        if game.nb_snakes > 0 {
            write!(
                self.stdout,
                "{}{}Score {}: {}{}",
                cursor::Goto(WIDTH as u16 + 7, starting_height + 1 as u16),
                color::Fg(color::Red),
                1,
                score_msg(&game.scores[0]),
                color::Fg(color::Reset)
            )
            .unwrap();
        }

        //Player2
        if game.nb_snakes > 1 {
            write!(
                self.stdout,
                "{}{}Score {}: {}{}",
                cursor::Goto(WIDTH as u16 + 7, starting_height + 2 as u16),
                color::Fg(color::Blue),
                2,
                score_msg(&game.scores[1]),
                color::Fg(color::Reset)
            )
            .unwrap();
        }

        //Player3
        if game.nb_snakes > 2 {
            write!(
                self.stdout,
                "{}{}Score {}: {}{}",
                cursor::Goto(WIDTH as u16 + 7, starting_height + 3 as u16),
                color::Fg(color::Green),
                3,
                score_msg(&game.scores[2]),
                color::Fg(color::Reset)
            )
            .unwrap();
        }

        //Player4
        if game.nb_snakes > 3 {
            write!(
                self.stdout,
                "{}{}Score {}: {}{}",
                cursor::Goto(WIDTH as u16 + 7, starting_height + 4 as u16),
                color::Fg(color::Yellow),
                4,
                score_msg(&game.scores[3]),
                color::Fg(color::Reset)
            )
            .unwrap();
        }

        write!(
            self.stdout,
            "{}Esc: quit",
            cursor::Goto(
                WIDTH as u16 + 7,
                starting_height + 1 + game.nb_snakes as u16
            )
        )
        .unwrap();
        self.stdout.flush().unwrap();
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

fn log_in_file(s: String) {
    if let Ok(mut file) = OpenOptions::new().append(true).open(LOG_FILE) {
        file.write(s.as_bytes()).unwrap();
    }
}
