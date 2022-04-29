
pub struct Object {
    pub name: String,
    pub x_pos: f64,
    pub y_pos: f64,
    pub x_vel: f64,
    pub y_vel: f64,
    pub mass: i32,
}


impl Object {
    pub fn new(name: String, x_pos: f64,y_pos: f64,x_vel: f64,y_vel: f64,mass: i32) -> Object {
        Object {
            name: name,
            x_pos: x_pos,
            y_pos: y_pos,
            x_vel: x_vel,
            y_vel: y_vel,
            mass: mass,
        }
    }
}