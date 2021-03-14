use crate::shared_structures::{Direction, Point, Snake, HEIGHT, WIDTH};

impl Snake {
    pub fn change_direction(&mut self, d: Direction) {
        match (self.direction.clone(), d) {
            (Direction::Up, Direction::Down) => (),
            (Direction::Down, Direction::Up) => (),
            (Direction::Left, Direction::Right) => (),
            (Direction::Right, Direction::Left) => (),
            (_, d) => self.direction = d,
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

    pub fn is_player_nb(&self, id: i32) -> bool {
        self.id == id
    }

    pub fn init(nb_players: i32, player_nb: i32) -> Self {
        let x;
        let y;
        let direction;
        let body;

        if nb_players <= 2 {
            match player_nb {
                1 => {
                    x = WIDTH as u16 / 4;
                    y = HEIGHT as u16 / 2;
                    direction = Direction::Right;
                    body = (x - 1, y);
                }
                2 => {
                    x = WIDTH as u16 * 3 / 4;
                    y = HEIGHT as u16 / 2;
                    direction = Direction::Left;
                    body = (x + 1, y);
                }
                _ => unimplemented!(), // should not happen
            }
        } else {
            match player_nb {
                1 => {
                    x = WIDTH as u16 / 4;
                    y = HEIGHT as u16 / 4;
                    direction = Direction::Right;
                    body = (x - 1, y);
                }
                2 => {
                    x = WIDTH as u16 * 3 / 4;
                    y = HEIGHT as u16 / 4;
                    direction = Direction::Left;
                    body = (x + 1, y);
                }
                3 => {
                    x = WIDTH as u16 / 4;
                    y = HEIGHT as u16 * 3 / 4;
                    direction = Direction::Right;
                    body = (x - 1, y);
                }
                4 => {
                    x = WIDTH as u16 * 3 / 4;
                    y = HEIGHT as u16 * 3 / 4;
                    direction = Direction::Left;
                    body = (x + 1, y);
                }
                _ => unimplemented!(), // should not happen
            }
        }
        Snake {
            id: player_nb,
            head: Point::new(x, y),
            body: vec![Point::new(body.0, body.1)],
            direction: direction,
        }
    }
}
