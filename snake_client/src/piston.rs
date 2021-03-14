use crate::client::Client;
use crate::game::{Point, HEIGHT, WIDTH};
use crate::snake::{Direction, Snake};
use piston_window::types::Color;
use piston_window::*;

pub const BLOCK_SIZE: f64 = 1.00;
pub const BACKGROUND: Color = [0.0, 0.0, 0.0, 1.0];
pub const SCORE: Color = [1.0, 1.0, 1.0, 1.0];
pub const SNAKE: Color = [0.1, 0.9, 0.1, 1.0];
pub const FRUIT: Color = [1.0, 0.0, 0.0, 1.0];
pub const OVERLAY: Color = [1.0, 0.0, 0.0, 0.5];
pub const WINDOW_TITLE: &'static str = "Snake";

struct Piston {
    window: PistonWindow,
    context: Context,
    g: G2d,
}

impl Client for Piston {
    fn init() -> Self {
        let size = [
            blocks_in_pixels(WIDTH as u32),
            blocks_in_pixels(HEIGHT as u32),
        ];
        let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, size)
            .resizable(false)
            .build()
            .unwrap();
        Piston { window }
    }

    fn draw_field(&mut self, width: u16, height: u16) {
        unimplemented!();
    }

    fn draw_snake(&mut self, snake: &Snake) {
        unimplemented!();
    }
    fn draw_food(&mut self, food: &Point) {
        //draw_block(ctx: &Context, g: &mut G2d, c: Color, pos: &Point)
        unimplemented!();
    }
    fn draw_scores(&mut self, scores: &Vec<i32>) {
        unimplemented!();
    }
    fn draw_results(&mut self, losers: Vec<i32>, scores: &Vec<i32>) {
        unimplemented!();
    }
}

fn init() {
    let size = [blocks_in_pixels(WIDTH), blocks_in_pixels(HEIGHT)];

    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, size)
        .resizable(false)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            main.key_down(key);
        }

        window.draw_2d(&event, |ctx, g| {
            clear(BACKGROUND, g);
            text::Text::new_color(SCORE, 20)
                .draw(
                    main.get_score().to_string().as_ref(),
                    &mut glyphs,
                    &ctx.draw_state,
                    ctx.transform.trans(0.0, 20.0),
                    g,
                )
                .unwrap();
            main.draw(ctx, g);
        });

        event.update(|arg| {
            main.update(arg.dt);
        });
    }
}
pub fn draw_block(ctx: &Context, g: &mut G2d, c: Color, pos: &Point) {
    rectangle(
        // Couleur du rectangle
        c,
        [
            // Position x
            pos.x as f64 * BLOCK_SIZE,
            // Position y
            pos.y as f64 * BLOCK_SIZE,
            // longueur
            BLOCK_SIZE,
            // largeur
            BLOCK_SIZE,
        ],
        ctx.transform,
        g,
    );
}

pub fn draw_snake_head(ctx: &Context, g: &mut G2d, c: Color, pos: &Point, dir: &Direction) {
    draw_block(ctx, g, c, pos);

    // Dessine un oeil de largeur et longueur 5
    fn draw_eye(ctx: &Context, g: &mut G2d, x: f64, y: f64) {
        rectangle(BACKGROUND, [x, y, 5.0, 5.0], ctx.transform, g);
    }

    let (x, y) = (
        blocks_in_pixels(pos.x as u32) as f64,
        blocks_in_pixels(pos.y as u32) as f64,
    );

    let block = blocks_in_pixels(1) as f64;

    match dir {
        Direction::Up => {
            draw_eye(ctx, g, x + 5.0, y + 5.0);
            draw_eye(ctx, g, x + block - 10.0, y + 5.0);
        }
        Direction::Right => {
            draw_eye(ctx, g, x + block - 10.0, y + 5.0);
            draw_eye(ctx, g, x + block - 10.0, y + block - 10.0);
        }
        Direction::Down => {
            draw_eye(ctx, g, x + 5.0, y + block - 10.0);
            draw_eye(ctx, g, x + block - 10.0, y + block - 10.0);
        }
        Direction::Left => {
            draw_eye(ctx, g, x + 5.0, y + 5.0);
            draw_eye(ctx, g, x + 5.0, y + block - 10.0);
        }
    }
}

pub fn draw_fruit(ctx: &Context, g: &mut G2d, c: Color, pos: &Point) {}

pub fn draw_overlay(ctx: &Context, g: &mut G2d, c: Color, size: (u32, u32)) {
    rectangle(
        c,
        [
            0.0,
            0.0,
            blocks_in_pixels(size.0) as f64,
            blocks_in_pixels(size.1) as f64,
        ],
        ctx.transform,
        g,
    );
}

pub fn blocks_in_pixels(n: u32) -> u32 {
    n * BLOCK_SIZE as u32
}
