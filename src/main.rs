extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

struct App {
    gl: GlGraphics,
    snake: Snake,
}

impl App {
    fn render(&mut self, arg: &RenderArgs) {
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(green, gl);
        });

        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button){
        let last_dir = self.snake.dir.clone();
        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_dir != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_dir != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_dir != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_dir != Direction::Left => Direction::Right,
            _ => last_dir
        };
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * 20) as f64,
                    (y * 20) as f64,
                    20_f64)
            })
            .collect();



        gl.draw(args.viewport(), |c, gl|{
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(red, square, transform, gl));
        });
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = App {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()),
            dir: Direction::Right,
        }
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
