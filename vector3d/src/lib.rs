use std::ops::{Index,IndexMut, MulAssign, DivAssign, Mul, Div, Neg};

#[derive(Debug, PartialEq)]
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

impl MulAssign<f32> for Vector3D {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar; 
    }
}

impl DivAssign<f32> for Vector3D {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar; 
    }
}

impl Mul<f32> for Vector3D {
    type Output = Self;
    fn mul(self, rhs_scalar: f32) -> Self {
        Vector3D { x: self.x * rhs_scalar, y: self.y * rhs_scalar, z: self.z * rhs_scalar}
    }
}

impl Div<f32> for Vector3D {
    type Output = Self;
    fn div(self, rhs_scalar: f32) -> Self {
        Vector3D { x: self.x / rhs_scalar, y: self.y / rhs_scalar, z: self.z / rhs_scalar}
    }
}

impl Neg for Vector3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector3D { x: -self.x, y: -self.y, z: -self.z}
    }
}



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

    #[test]
    fn scalar_multiplication_assign() {
        let mut vec3 = Vector3D {x:2.0,y:2.0,z:3.0};
        vec3*=2.0;
        assert_eq!(vec3.x, 4.0 );
        assert_eq!(vec3.y, 4.0 );
        assert_eq!(vec3.z, 6.0 ); 
    }

    #[test]
    fn scalar_division_assign() {
        let mut vec3 = Vector3D {x:2.0,y:2.0,z:3.0};
        vec3/=2.0;
        assert_eq!(vec3.x, 1.0 );
        assert_eq!(vec3.y, 1.0 );
        assert_eq!(vec3.z, 1.5 ); 
    }

    #[test]
    fn inline_scalar_multiplication() {
        let vec3 = Vector3D {x:2.0,y:2.0,z:3.0};
        assert_eq!(vec3 * 2.0, Vector3D{x:4.0, y:4.0, z:6.0})
    }

    #[test]
    fn inline_scalar_division() {
        let vec3 = Vector3D {x:2.0,y:2.0,z:3.0};
        assert_eq!(vec3 / 2.0, Vector3D{x:1.0, y:1.0, z:1.5})
    }

    #[test]
    fn negation() {
        let vec3 = Vector3D {x:2.0,y:2.0,z:3.0};
        assert_eq!(-vec3, Vector3D{x:-2.0, y:-2.0, z:-3.0})
    }

