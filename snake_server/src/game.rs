use crate::log;
use crate::shared_structures::*;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::Write;

const SPEED: u64 = 100;
const POINTS: i32 = 10;
const LOG_FILE: &'static str = "log";

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }

    pub fn next_point(p: &Point, d: &Direction) -> Point {
        match d {
            Direction::Up => Point::new(p.x, p.y - 1),
            Direction::Down => Point::new(p.x, p.y + 1),
            Direction::Left => Point::new(p.x - 1, p.y),
            Direction::Right => Point::new(p.x + 1, p.y),
        }
    }

    // Génère alétoirement un point dans l'espace du jeu
    // où sera placée la prochaine pomme
    pub fn random() -> Self {
        let rx = rand::thread_rng().gen_range(2, WIDTH);
        let ry = rand::thread_rng().gen_range(2, HEIGHT);
        Point::new(rx as u16, ry as u16)
    }
}

pub enum TurnOutcome {
    // La partie continue, le vecteur contient les perdants potentiels du tour
    Playing(Vec<i32>),
    // La partie est terminée, l'option contient le gagnant ou None pour une égalité
    End(Option<i32>),
}

impl Game {
    // Initialise une structure Game
    pub fn init(nb_snakes: i32, nb_bots: i32) -> Result<Self, String> {
        // Il doit y avoir au moins 2 serpents
        if nb_snakes < 2 {
            return Err(String::from("At least 2 snakes is needed"));
        }

        // Le nombre de joueurs doit être au moins de 1
        if nb_snakes <= nb_bots {
            return Err(String::from("At least 1 human player is needed"));
        }
        // Efface le contenu du fichier de log
        File::create(LOG_FILE).unwrap();
        let mut snakes = vec![];
        let mut scores = vec![];
        let mut bots = vec![];

        for player_nb in 1..nb_snakes + 1 {
            if player_nb > (nb_snakes - nb_bots) {
                bots.push(player_nb);
            }
            snakes.push(Snake::init(nb_snakes, player_nb));
            scores.push(PlayerStatus::Player(0));
        }

        Ok(Game {
            nb_snakes: nb_snakes,
            bots: bots,
            snakes: snakes,
            food: Point::random(),
            scores: scores,
            speed: SPEED,
        })
    }

    // Change la direction des serpents selon les commandes reçues
    // Si retourne [1,2)_ les joueurs 1 et 2 ont quitté
    fn handle_inputs(&mut self, inputs: Vec<Vec<Input>>) -> Vec<i32> {
        let mut leavers = vec![];
        for player in 0..inputs.len() {
            //Pour chaque joueur
            for input in inputs[player].iter() {
                //Pour chaque commande d'un joueur
                match input {
                    Input::Up => self.snakes[player].change_direction(Direction::Up),
                    Input::Right => self.snakes[player].change_direction(Direction::Right),
                    Input::Left => self.snakes[player].change_direction(Direction::Left),
                    Input::Down => self.snakes[player].change_direction(Direction::Down),
                    Input::Quit => {
                        leavers.push(player as i32);
                        break;
                    }
                }
            }
        }
        leavers
    }

    // Si retoune None un joueur a quitté la partir
    // Si on retoune _Some([1])_, le joueur 1 a perdu
    pub fn turn(&mut self, inputs: Vec<Vec<Input>>) -> TurnOutcome {
        let mut has_eaten = false;

        // Récupère les touches
        let leavers = self.handle_inputs(inputs);

        // Fais mouvoir les serpents
        for i in 0..self.snakes.len() {
            let snake: &mut Snake = &mut self.snakes[i];
            let is_gonna_eat = (Point::next_point(&snake.head, &snake.direction)) == self.food;
            snake.step(is_gonna_eat);
            if is_gonna_eat {
                if let PlayerStatus::Player(points) = self.scores[i] {
                    self.scores[i] = PlayerStatus::Player(POINTS + points);
                }
                has_eaten = true
            }
        }

        // Si un serpent a mangé, régénérer de la nourriture
        if has_eaten {
            self.food = Point::random()
        }

        // _losers_ contient les serpents perdants
        let losers = self.check_collisions();

        let mut leavers_losers: Vec<i32> = vec![];
        leavers_losers.extend(&losers);
        leavers_losers.extend(&leavers);
        leavers_losers.sort();
        leavers_losers.dedup();

        // on enlève les serpents qui ont perdu ou quitté
        for l in leavers_losers.iter() {
            self.snakes.retain(|snake| !snake.is_player_nb(*l));
            if leavers.contains(l) {
                log!("Serpent {} has left!", l);
                self.scores[(l - 1) as usize] = PlayerStatus::Leaver;
            }

            if losers.contains(l) {
                log!("Serpent {} has lost!", l);
                self.scores[(l - 1) as usize] = PlayerStatus::Loser;
            }
        }

        // Retourne les perdants
        match self.snakes.len() {
            0 => TurnOutcome::End(None),
            1 => TurnOutcome::End(Some(self.snakes[0].id)),
            _ => TurnOutcome::Playing(losers),
        }
    }

    // Check for collisions and return array of losing players
    pub fn check_collisions(&mut self) -> Vec<i32> {
        let mut losers = vec![];
        log_in_file(format!(
            "S1: {:?}, S2: {:?}\n",
            self.snakes[0].head, self.snakes[1].head
        ));
        for snake in self.snakes.iter() {
            // Teste les collisions entre serpents
            for other_snake in self.snakes.iter() {
                // Collisions tête - corps
                if other_snake.is_in_body(&snake.head) {
                    log_in_file("Is in body!\n".to_owned());
                    losers.push(snake.id);
                }

                // Collisions tête - tête
                if other_snake.head == snake.head && snake.id != other_snake.id {
                    losers.push(snake.id);
                    losers.push(other_snake.id);
                }
            }
            // Teste les collisions avec les bordures
            if snake.head.x == 0
                || snake.head.y == 0
                || snake.head.x == (WIDTH as u16)
                || snake.head.y == (HEIGHT as u16)
            {
                losers.push(snake.id);
            }
        }
        losers.sort();
        losers.dedup();
        losers
    }
}

fn log_in_file(s: String) {
    if let Ok(mut file) = OpenOptions::new().append(true).open(LOG_FILE) {
        file.write(s.as_bytes()).unwrap();
    }
}