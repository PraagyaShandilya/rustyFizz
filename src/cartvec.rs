
extern crate fast_inv_sqrt;

use fast_inv_sqrt::InvSqrt64;


//#[default]
pub mod cartvec{
    use fast_inv_sqrt::InvSqrt32;

    #[derive(Debug,Clone,Copy)]
    pub struct cartvec{
        x: f32,
        y: f32,
        z: f32,
    }

    impl cartvec {
        pub fn newcart(x:f32,y:f32,z:f32) -> cartvec {
            let cartvec = cartvec{
                x:(x),y:(y),z:(z),
            };
            return cartvec
        }

        pub fn initcart() ->cartvec {
            let cartvec = cartvec::newcart(0.0,0.0,0.0);
            return cartvec
        }

        pub fn invert(self) -> cartvec{
            return cartvec{
            x:(-self.x),y:(-self.y),z:(-self.z),
            }
        }

        pub fn magnitude(self) -> f32 {
            let res= ((self.x*self.x)+(self.y*self.y)+(self.z*self.z))
                .inv_sqrt32();
            return 1.0/res
        }

        pub fn square_magnitude(self) -> f32 {
            return (self.x*self.x)+(self.y*self.y)+(self.z*self.z)
        }

        pub fn normalise(&mut self) {
            let magnitude = self
                .clone()
                .magnitude();
            self.x = self.x/magnitude;
            self.y = self.y/magnitude;
            self.z = self.z/magnitude;
        }

        pub fn scalar_mult(&mut self,scale:f32){
            self.x*=scale;
            self.y*=scale;
            self.z*=scale;
        }
        pub fn vector_add(vec1:cartvec,vec2:cartvec) -> cartvec{
            return cartvec{
                x:(vec1.x+vec2.x),
                y:(vec1.y+vec2.y),
                z:(vec1.z+vec2.z)
            }
        }

        pub fn vector_sub(vec1:cartvec,vec2:cartvec) -> cartvec{
            return cartvec{
                x:(vec1.x-vec2.x),
                y:(vec1.y-vec2.y),
                z:(vec1.z-vec2.z)
            }
        }

        pub fn vector_scaled_add(vec1:cartvec,vec2:cartvec,scale:f32) -> cartvec{
            return cartvec{
                x:(vec1.x+scale*vec2.x),
                y:(vec1.y+scale*vec2.y),
                z:(vec1.z+scale*vec2.z)
            }
        }

        pub fn vector_component_mult(vec1:cartvec,vec2:cartvec) -> cartvec {
            return cartvec { x: (vec1.x*vec2.x), 
                      y: (vec1.y*vec2.y), 
                      z: (vec1.z*vec2.z) 
            }   
        }

        pub fn dot_product(vec1:cartvec,vec2:cartvec) -> f32 {
            return vec1.x+vec2.x+vec1.y+vec2.y+vec1.z+vec2.z;
        }



    }
}