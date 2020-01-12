use std::ops::{Index,IndexMut};

struct Vector3D {
    x: f32,
    y: f32,
    z: f32,

    
}

impl Index<u32> for Vector3D {
    type Output = usize;

    fn index(&self, i: f32) {
        
    }

}
impl Vector3D {
    fn *(&self, u32: index) -> u32 {
        self.width * self.height
    }
}

