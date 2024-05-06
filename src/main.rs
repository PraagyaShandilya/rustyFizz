mod particle;
use crate::particle::*;
use raylib::prelude::*;




fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);
        //d.draw_rectangle(vec.x as i32, vec.y as i32, 5,5, Color::BLACK);
        d.draw_text("Hello, world!", 12, 12, 20, Color::RAYWHITE)

    }
}