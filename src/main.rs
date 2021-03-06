extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::{MouseCursorEvent, MouseButton};
use piston::window::WindowSettings;
use piston::event_loop::{ EventSettings, Events };
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateEvent };
use glutin_window::GlutinWindow;
use opengl_graphics:: {GlGraphics, OpenGL };

mod object;


const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const G: f64 = 0.000000000066743;

pub struct App {
    gl: GlGraphics,
    objects: Vec<object::Object>,
    cursor_x: f64,
    cursor_y: f64,
}

impl App{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const BACKGROUND: [f32; 4] = [62.0/255.0, 199.0/255.0, 230.0/255.0, 1.0];
        const FOREGROUND: [f32; 4] = [192.0/255.0, 27.0/255.0, 247.0/255.0, 1.0];

        let main_rect = rectangle::square(0.0, 0.0, 20.0);
        
        self.gl.draw(args.viewport(), |c, gl| {

            clear(BACKGROUND, gl);

            for obj in self.objects.iter() {
                circle_arc(
                    FOREGROUND,
                    20.0,
                    77.0,
                    47.0,
                    main_rect,
                    c.transform.trans(obj.x_pos, obj.y_pos), 
                    gl);
            }
        });
    }

    fn update_cursor(&mut self, args: &[f64; 2]){
        self.cursor_x = args[0];
        self.cursor_y = args[1];
    }

    fn update(&mut self) {
        if self.objects.len() != 0 {
            let length = self.objects.len();
            for i in 0..(length - 1) {                
                'inner: for j in 0..(length - 1){ // doesn't work????
                    
                    let distance = find_distance( // distance
                        self.objects[i].x_pos, 
                        self.objects[i].y_pos,
                        self.objects[j].x_pos, 
                        self.objects[j].y_pos);
                    

                    if distance == 0.0 {
                        // break 'inner;
                        continue 'inner;
                    }
                        

                    let angle = find_angle( // angle
                        self.objects[i].x_pos, 
                        self.objects[i].y_pos,
                        self.objects[j].x_pos, 
                        self.objects[j].y_pos);
                    
                    
                    let force = find_force( // force 
                        self.objects[i].mass as f64, 
                        self.objects[j].mass as f64, 
                        distance);


                     // -1 to adjust for the way pixels are counted starting from the top
                    let mut x_acc = force * angle.cos() / self.objects[i].mass as f64; // force = mass * acceleration
                    let mut y_acc = -1.0 * force * angle.sin() / self.objects[i].mass as f64;

                    if x_acc < 0.0000000 { 
                        x_acc = 0.0;
                    }

                    if y_acc < 0.0000000 { 
                        y_acc = 0.0;
                    }

                    // temp_vec.push((x_acc, y_acc));

                    self.objects[i].x_vel += x_acc; // add acceleration to the velocity 
                    self.objects[i].y_vel += y_acc;

                }
            }
        }
        
    
        for elem in self.objects.iter_mut() {
            elem.x_pos += elem.x_vel;
            elem.y_pos += elem.y_vel;
        }

        self.objects.retain(|elem| 
            elem.x_pos < (WIDTH + 50) as f64 && 
            elem.y_pos < (HEIGHT + 50) as f64 &&
            elem.x_pos > -50.0 &&
            elem.y_pos > -50.0);
    }

    fn press(&mut self, args: &Button){
        if let &Button::Keyboard(key) = args {
            match key {
                Key::S => {
                    //println!("Pressed. velocity is: {} and current x is: {}", self.x_vel, self.x);
                    println!("Hello bahahaha");
                    
                    //self.x_vel += 1;
                }

                _ => {}
            }
        }

        if let &Button::Mouse(MouseButton) = args {
            match MouseButton {
                MouseButton::Left => {
                    //println!("This is a left click at x: {} and y: {}", self.cursor_x, self.cursor_y);
                
                    self.objects.push(
                        object::Object::new(
                            "Unnamed".to_string(), 
                            self.cursor_x, 
                            self.cursor_y, 
                            0.0, 
                            0.0, 
                            2000, 
                        )
                    );
                }
                _ => {}
            }
        }
    }
}


fn find_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn find_angle(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    -(y2-y1).atan2(x2-x1)
}

fn find_force(m1: f64, m2: f64, r: f64) -> f64 {
    (m1 * m2) / r.powi(2)
}

fn main() {
    let opengl = OpenGL::V4_5;


    let mut window: GlutinWindow = WindowSettings::new("Gravity Sim", [WIDTH,HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let earth = object::Object::new(
        "Earth".to_string(), 
        300.0, 
        500.0, 
        0.0, 
        0.0, 
        4000);

    
    let mars = object::Object::new(
        "Mars".to_string(), 
        300.0, 
        100.0, 
        0.0, 
        0.0, 
        2000);
    
    let jupiter = object::Object::new(
        "Jupiter".to_string(), 
        800.0, 
        500.0, 
        0.0, 
        0.0, 
        16000);
        
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        objects: vec![earth, mars, jupiter],
        cursor_x: 0.0,
        cursor_y: 0.0,
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

        if let Some(t) = e.mouse_cursor_args(){
            app.update_cursor(&t);
        }
    }
}
