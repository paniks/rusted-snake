extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::LinkedList;
use std::iter::FromIterator;

// GLOBAL

const SQUARE_WIDTH: u32 = 10;
const COLUMNS: u32 = 60;
const ROWS: u32 = 40;


#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Snake {
    gl: GlGraphics,
    body: LinkedList<(u32, u32)>,
    direction: Direction,
}

impl Snake {
    fn render(&mut self, args: &RenderArgs){
        use graphics;
        const BODYCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let snake_body: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                (x*SQUARE_WIDTH) as f64, 
                (y*SQUARE_WIDTH) as f64, 
                SQUARE_WIDTH as f64)  
                })
            .collect();

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            snake_body.into_iter().for_each(|square| graphics::rectangle( BODYCOLOR, square, transform, gl))
        });
    }

    fn update(&mut self, piece_eaten: bool) -> bool {

        let mut head = (*self.body.front().expect("No body")).clone();

        if (self.direction == Direction::UP && head.1 == 0)
            || (self.direction == Direction::LEFT && head.0 == 0)
            || (self.direction == Direction::DOWN && head.1 == ROWS - 1)
            || (self.direction == Direction::RIGHT && head.0 == COLUMNS - 1)
        {
            return false;
        }

        match self.direction {
            Direction::RIGHT => head.0 += 1,
            Direction::LEFT => head.0 -= 1,
            Direction::DOWN => head.1 += 1,
            Direction::UP => head.1 -= 1, 
        };

        if !piece_eaten{
            self.body.pop_back();
        }

        if self.collision(head.0, head.1) {
            return false;
        }

        self.body.push_front(head);
        true
    }

    fn collision(&self, x: u32, y: u32) -> bool {
        self.body.iter().any(|p| x == p.0 && y == p.1)
    }
}

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;
        const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; 

        let x = self.x * SQUARE_WIDTH;
        let y = self.y * SQUARE_WIDTH;

        let square = graphics::rectangle::square(x as f64, y as f64, SQUARE_WIDTH as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(COLOR, square, transform, gl);
        });
    }

    fn update(&mut self, s: &Snake) -> bool {
        let head = s.body.front().unwrap();
        if head.0 == self.x && head.1 == self.y {
            true
        } else {
            false
        }
    } 
}

pub struct App {
    gl: GlGraphics,
    snake: Snake,   
    food: Food,
    piece_eaten: bool,
    score: u32
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics;

        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 0.1];         
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BACKGROUND, gl);
        });

        self.snake.render(args);
        self.food.render(&mut self.gl, args);
    }

    fn update(&mut self) -> bool {
        if !self.snake.update(self.piece_eaten) {
            return false;
        }

        if self.piece_eaten {
            self.score +=1;
            self.piece_eaten = false;
        }

        self.piece_eaten = self.food.update(&self.snake);
        if self.piece_eaten {
            use rand::Rng;
            use rand::thread_rng;

            let mut r = thread_rng();
            loop {
                let new_x = r.gen_range(0, COLUMNS);
                let new_y = r.gen_range(0, ROWS);
                if !self.snake.collision(new_x, new_y) {
                    self.food = Food { x: new_x, y: new_y };
                    break;
                }
            }
        }

        return true;
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction
        };

    }

}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("Ssssssnake!", [SQUARE_WIDTH * COLUMNS, SQUARE_WIDTH * ROWS])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.graphics::rectangle( BODYCOLOR, square, transform, gl)
    let mut app = App {
        gl: GlGraphics::new(opengl),
        food: Food { x: 1, y: 1 },
        piece_eaten: false,
        score: 0,
        snake: Snake {
            gl: GlGraphics::new(opengl),
            body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()), 
            direction: Direction::RIGHT
            },
        };

    // Create a new event loop. ups method is upgrades per second, works like sleep.
    let mut events = Events::new(EventSettings::new()).max_fps(20);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(_u) = e.render_args() {
            if !app.update() {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                app.pressed(&k.button)
            }
        }

    }
    println!("GAME OVER! Your score {}", app.score);
}
