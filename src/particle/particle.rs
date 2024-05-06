pub mod Particle{
    use crate::particle::{cartvec::cvec::Cartvec as cartvec, HEIGHT, WIDTH};
    pub struct particle{
        pub(crate) pos:cartvec,
        pub(crate) velocity:cartvec,
        pub(crate) acceleration:cartvec,
        pub(crate) damping:f32,
        pub(crate) inv_mass:f32
    }

    impl particle{
           pub fn new()-> Self{
                particle{
                    pos: cartvec::initcart(),
                    velocity: cartvec::initcart(),
                    acceleration: cartvec::initcart(),
                    damping: 0.99,
                    inv_mass: 0.9,
                }
           }

           pub fn set_inv_mass(&mut self,m:f32){
                self.inv_mass = m
           }

           pub fn set_mass(&mut self,m:f32){
                self.inv_mass=1.0/m
           }

           pub fn set_damping(&mut self,d:f32){
                self.damping = d
           }

           pub fn set_pos(&mut self,x:f32,y:f32,z:f32){
                self.pos = cartvec{
                    x:(x),y:(y),z:(z),
                }
            }

           pub fn set_acceleration(&mut self,x:f32,y:f32,z:f32){
                self.acceleration= cartvec{
                    x:(x),y:(y),z:(z),
                }
           }

           pub fn set_velocity(&mut self,x:f32,y:f32,z:f32){
                self.velocity = cartvec{
                    x:(x),y:(y),z:(z),
                }
           }

           pub fn integrate(&mut self,duration:f32){
            //calculating new position 
                let x;let y;let z;
                (x,y,z) = cartvec::vector_add(self.pos.clone(),
                                              self.velocity.clone())
                        .deconstruct();
                
                self.set_pos(x, y, z);
            //TODO calculate the new force 
            //TODO calculate the new acceleration
            //damping velocity
                self.damping *= self.damping.powf(duration);
                if x >= ((WIDTH as f32) - x){
                    self.velocity.x*=-1.0;
                }
                if y >= ((HEIGHT as f32) - y){
                    self.velocity.y*=-1.0;
                }
                self.velocity.scalar_mult(self.damping);
                return
            
           }

    }
}