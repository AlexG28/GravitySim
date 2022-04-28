extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

//use std::process;
use piston::window::WindowSettings;
use piston::event_loop::{ EventSettings, Events };
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateEvent };
use glutin_window::GlutinWindow;
use opengl_graphics:: {GlGraphics, OpenGL };


pub struct App {
    gl: GlGraphics,
    x: i32,
    y: i32, 
    x_vel: i32,
    y_vel: i32,
}

impl App{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const BACKGROUND: [f32; 4] = [62.0/255.0, 199.0/255.0, 230.0/255.0, 1.0];
        const FOREGROUND: [f32; 4] = [192.0/255.0, 27.0/255.0, 247.0/255.0, 1.0];

        let main_rect = rectangle::square(0.0, 0.0, 50.0);
    
        self.gl.draw(args.viewport(), |c, gl| {

            clear(BACKGROUND, gl);
            rectangle(FOREGROUND, main_rect, c.transform.trans(self.x as f64, self.y as f64), gl);

        });
    }

    fn update(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
    }

    fn press(&mut self, args: &Button){
        if let &Button::Keyboard(key) = args {
            match key {
                Key::S => {
                    println!("Pressed. velocity is: {} and current x is: {}", self.x_vel, self.x);
                    self.x_vel += 1;
                }

                _ => {}
            }
        }
    }
}


fn main() {
    let opengl = OpenGL::V4_5;


    let mut window: GlutinWindow = WindowSettings::new("Gravity Sim", [1280,720])
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut app = App{
        gl: GlGraphics::new(opengl),
        x: 150,
        y: 300,
        x_vel: 0,
        y_vel: 0,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args(){
            app.render(&r);
        }

        if let Some(_u) = e.update_args(){
            app.update();
        }

        if let Some(z) = e.press_args(){
            app.press(&z);
        }
    }

}
