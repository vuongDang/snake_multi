use crate::shared_structures::{Direction, Point, Snake, HEIGHT, WIDTH};
use rand::Rng;

impl Snake {
    pub fn change_direction(&mut self, d: Direction) {
        match (&self.direction, d) {
            (Direction::Up, Direction::Down) => (),
            (Direction::Down, Direction::Up) => (),
            (Direction::Left, Direction::Right) => (),
            (Direction::Right, Direction::Left) => (),
            (_, d) => self.direction = d,
        }
    }

    pub fn move_randomly(&mut self) {
        self.direction = match rand::thread_rng().gen_range(1, 5) {
            1 => Direction::Up,
            2 => Direction::Right,
            3 => Direction::Down,
            4 => Direction::Left,
            _ => panic!("This should not happen"),
        };
    }

    pub fn move_to_food(&mut self, food: &Point) {
        let (x_dist, y_dist) = Point::sub(food, &self.head);
        if x_dist.abs() >= y_dist.abs() {
            if x_dist < 0 {
                if self.direction != Direction::Right {
                    self.direction = Direction::Left;
                } else {
                    self.move_to_food_y(y_dist);
                }
            } else {
                if self.direction != Direction::Left {
                    self.direction = Direction::Right;
                } else {
                    self.move_to_food_y(y_dist);
                }
            }
        } else {
            if y_dist < 0 {
                if self.direction != Direction::Down {
                    self.direction = Direction::Up;
                } else {
                    self.move_to_food_x(x_dist);
                }
            } else {
                if self.direction != Direction::Up {
                    self.direction = Direction::Down;
                } else {
                    self.move_to_food_x(x_dist);
                }
            }
        }
    }

    pub fn move_to_food_y(&mut self, y_dist: i16) {
        if y_dist < 0 {
            self.direction = Direction::Up
        } else {
            self.direction = Direction::Up
        }
    }

    pub fn move_to_food_x(&mut self, x_dist: i16) {
        if x_dist < 0 {
            self.direction = Direction::Left;
        } else {
            self.direction = Direction::Right
        }
    }

    pub fn step(&mut self, grow: bool) {
        // On modifie le corps du serpent
        self.body.push(self.head.clone());
        if !grow {
            self.body.remove(0);
        }

        // On modifie la tête
        self.head = Point::next_point(&self.head, &self.direction);
    }

    pub fn is_in_body(&self, p: &Point) -> bool {
        for bp in self.body.iter() {
            if bp == p {
                return true;
            }
        }
        false
    }

    pub fn is_player_nb(&self, id: u32) -> bool {
        self.id == id
    }

    pub fn init(nb_players: u32, player_nb: u32) -> Self {
        let x;
        let direction;
        let body;

        let floor: u16 = (player_nb as u16 + 1) / 2;
        let total_nb_of_floors: u16 = (((nb_players + 1) / 2) as u16) + 1;
        let size_of_floor = HEIGHT as u16 / total_nb_of_floors;
        let y = floor * size_of_floor;

        if player_nb % 2 == 0 {
            x = WIDTH as u16 * 3 / 4;
            direction = Direction::Left;
            body = (x + 1, y);
        } else {
            x = WIDTH as u16 / 4;
            direction = Direction::Right;
            body = (x - 1, y);
        };

        Snake {
            id: player_nb,
            head: Point::new(x, y),
            body: vec![Point::new(body.0, body.1)],
            direction: direction,
        }
    }
}
