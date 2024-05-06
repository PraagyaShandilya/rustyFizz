mod particle;
use crate::particle::cartvec::cvec::Cartvec as Cartvec;
use crate::particle::particle::Particle::particle as Particle;
use particle::WIDTH;
use particle::HEIGHT;
use raylib::prelude::*;


const GAME_SPEED:f32 = 0.01;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("rustyFizz")
        .build();

    rl.set_target_fps(60);
    let mut molecule:Particle = Particle::new();
    molecule.set_pos(50.0,100.0,15.0);
    molecule.set_velocity(5.0,10.0,0.0);
    let mut duration:f32 = 0.0;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);
        molecule.integrate(duration);
        d.draw_rectangle(molecule.pos.x as i32, 
                         molecule.pos.y as i32,
                         molecule.pos.z as i32,
                         molecule.pos.z as i32, 
                         Color::RAYWHITE);
        let text:String = format!("x:{} y:{} dmp:{}", 
                            molecule.pos.x,
                            molecule.pos.y,
                            molecule.damping);
        //println!("{}", text.as_str());
        d.draw_text(text.as_str(), 12, 12, 20, Color::RAYWHITE);
        duration+=GAME_SPEED;
    }
}