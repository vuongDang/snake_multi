use crate::log;
use crate::shared_structures::ClientMsg::*;
use crate::shared_structures::*;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::Write;

const SPEED: u64 = 100;
const POINTS: i32 = 10;
const LOG_FILE: &'static str = "log";
const MAX_SNAKE_NB: u32 = 4;

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

    pub fn sub(p1: &Point, p2: &Point) -> (i16, i16) {
        (p1.x as i16 - p2.x as i16, p1.y as i16 - p2.y as i16)
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
        // Maximum 4 serpents
        if nb_snakes > MAX_SNAKE_NB as i32 {
            return Err(format!("Maximum {} snakes", MAX_SNAKE_NB));
        }

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
            snakes.push(Some(Snake::init(nb_snakes, player_nb)));
            scores.push(PlayerStatus::Player(0));
        }

        Ok(Game {
            nb_snakes: nb_snakes,
            nb_snakes_alive: nb_snakes,
            snakes: snakes,
            food: Point::random(),
            bots: bots,
            bots_difficulty: BotMovement::ToTheFood,
            scores: scores,
            speed: SPEED,
        })
    }

    // Change la direction des serpents selon les commandes reçues
    // Si retourne [1,2] les joueurs 1 et 2 ont quitté
    fn handle_inputs(&mut self, inputs: Vec<ClientMsg>) -> Vec<i32> {
        let mut leavers = vec![];
        //Pour chaque commande d'un joueur
        for (player, input) in inputs.into_iter().enumerate() {
            if let Some(snake) = &mut self.snakes[player] {
                match input {
                    SnakeDirection(Some(d)) => snake.change_direction(d),
                    SnakeDirection(None) => (),
                    Leave => {
                        leavers.push(1 + player as i32);
                        break;
                    }
                }
            }
        }
        leavers
    }

    // Algorithme qui fait bouger les bots
    fn move_snake_bots(&mut self) {
        for bot in self.bots.iter() {
            if let Some(snake) = &mut self.snakes[*bot as usize - 1] {
                match self.bots_difficulty {
                    // Si les bots bougent alétoirement
                    BotMovement::Random => snake.move_randomly(),

                    // Les bots se dirigent vers la pomme
                    BotMovement::ToTheFood => snake.move_to_food(&self.food),

                    // Bougent pour survivre
                    BotMovement::Survival => {}
                }
            }
        }
    }

    // Si retoune None un joueur a quitté la partir
    // Si on retoune _Some([1])_, le joueur 1 a perdu
    pub fn turn(&mut self, inputs: Vec<ClientMsg>) -> TurnOutcome {
        let mut has_eaten = false;

        // Récupère les touches
        let leavers = self.handle_inputs(inputs);

        // Fais bouger les bots
        self.move_snake_bots();

        // Fais mouvoir les serpents
        for (i, snake) in self.snakes.iter_mut().enumerate() {
            if let Some(snake) = snake {
                let is_gonna_eat = (Point::next_point(&snake.head, &snake.direction)) == self.food;
                snake.step(is_gonna_eat);

                if is_gonna_eat {
                    if let PlayerStatus::Player(points) = self.scores[i] {
                        self.scores[i] = PlayerStatus::Player(POINTS + points);
                    }
                    has_eaten = true;
                }

                // Vérification
                if let PlayerStatus::Player(points) = self.scores[i] {
                    assert!((snake.body.len() as i32 - 1) * 10 == points);
                }
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
        self.nb_snakes_alive -= leavers_losers.len() as i32;
        for l in leavers_losers.iter() {
            self.snakes[(l - 1) as usize] = None;
            //self.bots.retain(|bot| *bot != *l);

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
        match self.nb_snakes_alive {
            0 => TurnOutcome::End(None),
            1 => {
                let winner: Option<&Option<Snake>> =
                    self.snakes.iter().find(|snake| snake.is_some());
                let winner: &Option<Snake> = winner.unwrap();
                let winner: &Snake = winner.as_ref().unwrap();
                TurnOutcome::End(Some(winner.id))
            }
            _ => TurnOutcome::Playing(losers),
        }
    }

    // Check for collisions and return array of losing players
    pub fn check_collisions(&mut self) -> Vec<i32> {
        let mut losers = vec![];
        let mut snakes_alive: Vec<&Snake> = vec![];
        for snake in self.snakes.iter() {
            if let Some(snake) = snake {
                snakes_alive.push(snake);
            }
        }

        for snake in snakes_alive.iter() {
            // Teste les collisions entre serpents
            for other_snake in snakes_alive.iter() {
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
            if snake.head.x <= 1
                || snake.head.y <= 1
                || snake.head.x >= (WIDTH as u16)
                || snake.head.y >= (HEIGHT as u16)
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
