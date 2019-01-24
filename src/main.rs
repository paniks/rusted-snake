extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::LinkedList;

const SNAKE_WIDTH: u32 = 10;

#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Snake {
    x: u32,
    y: u32,
    direction: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;
        const BODYCOLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(self.x as f64, self.y as f64, SNAKE_WIDTH as f64);
    
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle( BODYCOLOR, square, transform, gl);
        });
    }

    fn update(&mut self){
        match self.direction {
            Direction::RIGHT => self.x += 1,
            Direction::LEFT => self.x -= 1,
            Direction::DOWN => self.y += 1,
            Direction::UP => self.y -= 1, 
        }
    }

}

pub struct App {
    gl: GlGraphics,
    snake: Snake,   
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 0.0];        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(BACKGROUND, gl);
        });

        self.snake.render(&mut self.gl, args)
    }

    fn update(&mut self){
        self.snake.update();
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
    let mut window: Window = WindowSettings::new("Ssssssnake!", [200, 200])
                            .opengl(opengl)
                            .exit_on_esc(true)
                            .build()
                            .unwrap();

    // Create a new game and run it.
    let mut app = App {gl: GlGraphics::new(opengl), snake: Snake {x :9, y : 9, direction: Direction::RIGHT}};

    // Create a new event loop. ups method is upgrades per second, works like sleep.
    let mut events = Events::new(EventSettings::new()).ups(1);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.render_args() {
            app.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                app.pressed(&k.button)
            }
        }

    }
}
