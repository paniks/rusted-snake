extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

struct Snake {
    x: u32,
    y: u32,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;
        const BODYCOLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(self.x as f64, self.y as f64, 10.0);
    
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle( BODYCOLOR, square, transform, gl);
        });
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
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
    let mut app = App {gl: GlGraphics::new(opengl), snake: Snake {x :50, y : 20}};

    // Create a new event loop.
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}
