use core::f32;

#[derive(Debug,Clone,Copy)]
//#[default]
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
        cartvec
    }

    pub fn initcart() ->cartvec {
        let cartvec = cartvec::newcart(0.0,0.0,0.0);
        cartvec
    }

    pub fn invert(self) -> cartvec{
        cartvec{
        x:(-self.x),y:(-self.y),z:(-self.z),
        }
    }

}
