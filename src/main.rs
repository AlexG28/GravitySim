extern crate piston_window;

use piston_window::*;

fn main() {

    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [1280, 720])
        .exit_on_esc(true).build().unwrap();


    while let Some(event) = window.next() {
        
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            
            
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                    [0.0,0.0,100.0,100.0],
                    context.transform,
                    graphics);
        
            circle_arc([0.0, 1.0, 0.0, 1.0], 94.3, 0.0, 450.0, [500.0,500.0,100.0,100.0], context.transform, graphics);


        });
    }


    //println!("Hello, world!");
}
