use std::io::Empty;
use std::time::{SystemTime, UNIX_EPOCH};

use graphics::math::rotate_radians;
use piston_window::*;
use piston_window::types::Color;
use rand::{Rng, RngCore, thread_rng};
use rand::rngs::ThreadRng;

const WINDOW_TITLE: &str = "Rust Snake";
const WINDOW_WIDTH_PIXELS: f64 = 640.0;
const WINDOW_HEIGHT_PIXELS: f64 = 480.0;

const TILE_SIZE: f64 = 20.0;
const ROW_COUNT: usize = WINDOW_HEIGHT_PIXELS as usize / TILE_SIZE as usize;
const COL_COUNT: usize = WINDOW_WIDTH_PIXELS as usize / TILE_SIZE as usize;

const COLOR_WHITE: Color = [200.0, 200.0, 200.0, 1.0];
const COLOR_RED: Color = [200.0, 0.0, 0.0, 1.0];
const COLOR_GREEN: Color = [0.0, 200.0, 0.0, 1.0];

const COLOR_EMPTY: Color = COLOR_WHITE;
const COLOR_FOOD: Color = COLOR_GREEN;
const COLOR_SNAKE: Color = COLOR_RED;

const FRAME_PER_SECONDS: u128 = 30;
const MILLIS_PER_FRAME: u128 = (1000.0 / FRAME_PER_SECONDS as f64) as u128;

#[derive(Debug)]
enum MovementDirection {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
struct World {
    pub is_running: bool,
    pub row_count: usize,
    pub col_count: usize,
    pub rng: ThreadRng,
    pub movement_direction: MovementDirection,
    pub snake_body: Vec<[usize; 2]>,
}

impl World {
    pub fn new(rows: usize, cols: usize) -> World {
        let mut world = World {
            is_running: true,
            row_count: rows,
            col_count: cols,
            rng: thread_rng(),
            movement_direction: MovementDirection::Up,
            snake_body: Vec::new(),
        };

        world.init();
        world
    }

    pub fn init(&mut self) {
        self.movement_direction = MovementDirection::Left;
        self.snake_body.push([0, 0]);
        self.snake_body.push([0, 1]);
        self.snake_body.push([0, 2]);
    }

    pub fn step(&mut self) {
        if !self.is_running {
            return;
        }

        let len = self.snake_body.len();

        for i in 0..len - 1 {
            self.snake_body[i][0] = self.snake_body[i + 1][0];
            self.snake_body[i][1] = self.snake_body[i + 1][1];
        }

        if let Some(last) = self.snake_body.last_mut() {
            let wrap = |n: i32, max: u32| {
                let max = max as i32;
                if n >= 0 {
                    n % max
                } else {
                    (max + (n % max)) % max
                }
            };

            match self.movement_direction {
                MovementDirection::Up => { last[0] = wrap(last[0] as i32 - 1, self.row_count as u32) as usize }
                MovementDirection::Left => { last[1] = wrap(last[1] as i32 - 1, self.col_count as u32) as usize }
                MovementDirection::Down => { last[0] = wrap(last[0] as i32 + 1, self.row_count as u32) as usize }
                MovementDirection::Right => { last[1] = wrap(last[1] as i32 + 1, self.col_count as u32) as usize }
            }
        }
    }
}

fn main() {
    let mut window: PistonWindow<> =
        WindowSettings::new(
            WINDOW_TITLE,
            [WINDOW_WIDTH_PIXELS, WINDOW_HEIGHT_PIXELS], )
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut world: World = World::new(20, 20);
    let mut food_pos: [usize; 2] = [0, 0];
    food_pos[0] = world.rng.gen_range(0..world.row_count);
    food_pos[1] = world.rng.gen_range(0..world.col_count);

    // MAIN LOOP
    let mut previous_update = UNIX_EPOCH;
    while let Some(event) = window.next() {
        if let Some(key) = event.press_args() {
            if key == Button::Keyboard(Key::Up) {
                world.movement_direction = MovementDirection::Up
            }

            if key == Button::Keyboard(Key::Left) {
                world.movement_direction = MovementDirection::Left
            }

            if key == Button::Keyboard(Key::Down) {
                world.movement_direction = MovementDirection::Down
            }

            if key == Button::Keyboard(Key::Right) {
                world.movement_direction = MovementDirection::Right
            }

            if key == Button::Keyboard(Key::Space) {
                world.is_running = !world.is_running
            }
        }

        // This part of code ensures that the program always runs at the predetermined amount of FPS rate, e.g. 60
        if previous_update.elapsed().map(|d| d.as_millis()).unwrap_or(0) > MILLIS_PER_FRAME {
            world.step();
            previous_update = SystemTime::now();
        }

        let tile_rect = Rectangle::new(COLOR_EMPTY);
        let tile_border_rect = Rectangle::new_border(COLOR_GREEN, 1.0);
        
        // Check if the snake has eaten the food
        // by comparing the last element of the snake's body with the food's position
        // If so, add a new tile to the snake
        // and generate a new food position

        if world.snake_body.last().unwrap() == &food_pos {
            // Add a new tile to the snake
            // on the tail of the snake
            // and generate a new food position
            // that is not on the snake's body

            // New tail at the end of the snake plus one
            let new_tail = world.snake_body.last().unwrap().clone();
            world.snake_body.push(new_tail);
            food_pos[0] = world.rng.gen_range(0..world.row_count);
            food_pos[1] = world.rng.gen_range(0..world.col_count);
            // Check if the new food position is on the snake's body
            // If so, generate a new food position
            // until it is not on the snake's body
            while world.snake_body.contains(&food_pos) {
                food_pos[0] = world.rng.gen_range(0..world.row_count);
                food_pos[1] = world.rng.gen_range(0..world.col_count);
            }
        }
        

        window.draw_2d(&event, |context, graphics, _device| {
            // CLEAR SCREEN
            clear(COLOR_EMPTY, graphics);

            for i_row in 0..world.row_count {
                for i_col in 0..world.col_count {
                    let start_coords = [
                        i_col as f64 * TILE_SIZE,
                        i_row as f64 * TILE_SIZE,
                    ];

                    let finish_coords = [
                        (i_col + 1) as f64 * TILE_SIZE,
                        (i_row + 1) as f64 * TILE_SIZE,
                    ];

                    let mut color = COLOR_EMPTY;
                    if world.snake_body.contains(&[i_row, i_col]) {
                        color = COLOR_SNAKE;
                    }
                    if food_pos == [i_row, i_col] {
                        color = COLOR_FOOD;
                    }

                    tile_rect.color(color)
                        .draw_from_to(
                            start_coords,
                            finish_coords,
                              &context.draw_state,
                              context.transform,
                              graphics,
                        );

                    tile_border_rect
                        .draw_from_to(
                            start_coords,
                            finish_coords,
                            &context.draw_state,
                            context.transform,
                            graphics,
                        );
                }
            }
        });
    }
}
