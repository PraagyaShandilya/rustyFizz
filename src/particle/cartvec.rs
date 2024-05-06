
extern crate fast_inv_sqrt;

use fast_inv_sqrt::InvSqrt64;


//#[default]
pub mod cvec{
    use fast_inv_sqrt::InvSqrt32;

    #[derive(Debug,Clone,Copy)]
    pub struct cartvec{
        x: f32,
        y: f32,
        z: f32,
    }
    impl PartialEq for cartvec {
        fn eq(&self, other: &Self) -> bool {
            return (self.x == other.x) 
                    &&
                   (self.y == other.y) 
                    &&
                   (self.z == other.z)    
        }
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

        pub fn is_zero_vec(vec: &cartvec) -> bool {
            vec.eq(&Self::initcart())
        }

        pub fn invert(self) -> cartvec{
            return cartvec{
            x:(-self.x),y:(-self.y),z:(-self.z),
            }
        }

        pub fn inverse_magnitude(self) -> f32 {
            ((self.x*self.x)+(self.y*self.y)+(self.z*self.z))
                .inv_sqrt32()
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
        //single-vector ops
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
        //vector multiplication
        
        pub fn dot_product(vec1:cartvec,vec2:cartvec) -> f32 {
            return vec1.x+vec2.x+vec1.y+vec2.y+vec1.z+vec2.z;
        }

        pub fn cross_product(vec1:cartvec,vec2:cartvec) -> cartvec {
            return cartvec{ x:(vec1.y*vec2.z-vec1.z*vec2.y), 
                            y:(vec1.z*vec2.x-vec2.x*vec2.z), 
                            z:(vec2.x*vec2.y-vec1.y*vec2.x)
                 }
        }   
        //calculate angle between two vectors usign the dot_product()
        pub fn theta(mut veca:cartvec,mut vecb:cartvec) -> f32{
            veca.normalise();
            vecb.normalise();
            let res:f32 =Self::dot_product(
                veca,
                vecb,
            );
            return res.acos()
        }

    }
}