use std::ops::{Index,IndexMut};

struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Index<u32> for Vector3D {
    type Output = f32;

    fn index(&self, index: u32) -> &f32 {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of bounds!")
        };
    }

}

impl IndexMut<u32> for Vector3D {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("out of bounds!")
        };
    }

}
// impl Vector3D {
//     fn *(&self, f32: index) -> f32 {
//         self.width * self.height
//     }
// }

#[cfg(test)]
    #[test]
    fn create_vector_3d() {
        let vec3 = Vector3D {x:1.0,y:2.0,z:3.0 };
        assert_eq!(vec3.x,1.0 );
        assert_eq!(vec3.y,2.0 );
        assert_eq!(vec3.z,3.0 );
    }

    #[test]
    fn access_vector_fields_with_array() {
        let vec3 = Vector3D {x:1.0,y:2.0,z:3.0 };
        assert_eq!(vec3[0],1.0 );
        assert_eq!(vec3[1],2.0 );
        assert_eq!(vec3[2],3.0 );
    }

    #[test]
    fn mutate_vector_field_() {
        let mut vec3 = Vector3D {x:1.0,y:2.0,z:3.0 };
        vec3[0] = -2.0;
        vec3[1] = -3.0;
        vec3[2] = -55.0;
        assert_eq!(vec3[0],-2.0 );
        assert_eq!(vec3[1],-3.0 );
        assert_eq!(vec3[2],-55.0 );
    }

