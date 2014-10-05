#![feature(globs)]
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate shader_version;

use sort::sort::qsort;

use std::vec::Vec;

use sdl2_game_window::WindowSDL2;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL_2_1;

use piston::{
    Render,
    RenderArgs,
    Update,
    UpdateArgs,
};


use piston::graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
};

use std::sync::spsc_queue::{Producer, Consumer, queue};
mod sort;

pub struct App {
    gl: Gl,       // OpenGL drawing backend.
    consumer: Consumer<Locations>,
    element_width: f64,
    height: u32,
    width: u32,
    elements:Vec<f64>,
    last_location:Locations,
}

pub static RED:[f32, ..4] = [255.0, 0.0, 0.0 ,1.0];
pub static GREEN:[f32, ..4] = [0.0, 255.0, 0.0, 1.0];
pub static BLUE:[f32, ..4] = [0.0, 0.0, 255.0, 1.0];

impl App {
    fn render(&mut self, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen.
        context.color(GREEN).draw(&mut self.gl);

        let padding = 10.0f64;
        for (index, height) in self.elements.iter().enumerate() {
            let color:[f32, ..4];
            if index == self.last_location.i  || index == self.last_location.j {
                color = BLUE;
            } else {
                color = RED
            }
            let i = index as f64;
            context.rect((self.element_width + padding) * i , 0f64, self.element_width, *height)
                .color(color)
                .draw(&mut self.gl);
        }
    }

    fn update(&mut self, _: &UpdateArgs) {
        let data = self.consumer.pop();
        match data {
            Some(location) => {
                self.last_location = location;
                self.elements.as_mut_slice().swap(location.i, location.j);
            }
            None => return,
        }
        println!("{}", self.elements.as_slice());
    }
}

struct Locations {
    i: uint,
    j: uint,
}

impl Send for Locations {}

fn main() {
    let mut v = [32i32,63251,34,1,2,4,0, 432, 124,512,5121,11,241,51,2541,51,516];
    //let mut v = [1i32, 4, 5, 6];
    let (consumer, mut producer) = queue::<Locations>(0);
    spawn(proc() {
        qsort(v, |i, j| {
            if i == j {return;}
//           std::io::timer::sleep(std::time::duration::Duration::seconds(1));
           println!("i:{}, j:{}", i, j);
           producer.push(Locations{i:i, j:j});
        });
        println!("{}", v.as_slice());
        println!("Sort finish");
    });

    let window_settings = piston::WindowSettings::default();
    // Create an SDL window.
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_2_1,
        piston::WindowSettings::default()
    );

    // Some settings for how the game should be run.
    let event_settings = piston::EventSettings {
        updates_per_second: 1,
        max_frames_per_second: 20
    };

    //println!("{}", v.as_slice());
    let heights = generate_heights(v);
    // Create a new game and run it.
    let mut app = App{gl: Gl::new(OpenGL_2_1),
                      consumer: consumer,
                      element_width: 5f64,
                      elements: heights,
                      height: window_settings.size[0],
                      width: window_settings.size[1],
                      last_location:Locations{i:0, j:0},
                    };

    for e in piston::EventIterator::new(&mut window, &event_settings) {
        match e {
            Render(_args) =>
                app.render(&_args),
            Update(_args) =>
                app.update(&_args),
            _ => {},
        }
    }
}

fn generate_heights<T: Ord>(elements:&[T]) -> std::vec::Vec<f64>{
    let step = 10f64;
    let mut v = std::vec::Vec::<f64>::with_capacity(elements.len());
    for element in elements.iter() {
        let mut count  = 1f64;
        for e in elements.iter() {
            if *element > *e {count+=1f64;}
        }
        v.push(count * step);
    }
    v
}

#[cfg(test)]
#[test]
fn test_generate_height() {
    let origin = [1i32, 30, 5, 3, 100];
    let expected = [10f64, 40f64, 30f64, 20f64, 50f64];
    let result = generate_heights(origin);
    let compare = expected.partial_cmp(&result.as_slice()).unwrap();
    match compare {
        Equal => return,
        Greater => {
            println!("Greater");
        },
        Less => {
            println!("Less");
        }
    }
    println!("Origin {}", origin.as_slice());
    println!("Expected: {}", expected.as_slice());
    println!("Result: {}", result.as_slice());
    fail!("");
}
