use std::ops::{Index,IndexMut, MulAssign, DivAssign, Mul, Div, Neg, AddAssign, SubAssign, Add, Sub};
use fast_inv_sqrt::InvSqrt32;
use std::f32;
#[derive(Debug, PartialEq, Copy, Clone)]
struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    fn new(x: f32, y: f32, z: f32) -> Vector3D{
        Vector3D {x: x, y: y, z: z}
    }

    fn magnitude(&self) -> f32 {
        (&self.x * &self.x + &self.y * &self.y + &self.z * &self.z).sqrt()
    }

    fn normalize_precise(&self) -> Vector3D {
        *self / self.magnitude()
    }

    fn normalize(&self) -> Vector3D {
        let mag_squared = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).inv_sqrt32();
        *self * mag_squared
    }
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


// Self is the type e.g Vector3d
impl AddAssign for Vector3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
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

    #[test]
    fn new_vector() {
        let vec3 = Vector3D::new(2.0,3.0,4.0);
        assert_eq!(vec3, Vector3D{x:2.0, y:3.0, z:4.0})
    }

    #[test]
    fn magnitude_test() {
        let vec3 = Vector3D::new(2.0,3.0,4.0);
        assert_eq!(vec3.magnitude(), 5.38516480713)
    }

    #[test]
    fn normalize_precise_test() {
        let vec3 = Vector3D::new(2.0,3.0,4.0);
        assert_eq!(vec3.normalize_precise(),Vector3D::new(0.37139067635,0.55708605,0.7427813527) )
    }

    #[test]
    fn normalize_test() {
        let vec3 = Vector3D::new(2.0,3.0,4.0);
        assert_eq!(vec3.normalize(),Vector3D::new(0.37097,0.556455,0.74194));
        assert_eq!(vec3, Vector3D::new(2.0,3.0,4.0));
    }

    #[test]
    fn add_assign_test() {
        let mut vec3 = Vector3D::new(2.0,3.0,4.0);
        vec3 += Vector3D::new(2.0,3.0,4.0);
        assert_eq!(vec3,Vector3D {x:4.0, y: 6.0, z:8.0});
      
    }

    #[test]
    fn sub_assign_test() {
        let mut vec3 = Vector3D {x:4.0, y: 6.0, z:8.0};
        vec3 -= Vector3D::new(2.0,3.0,4.0);
        assert_eq!(vec3,Vector3D::new(2.0,3.0,4.0));  
    }

    fn add_test() {
        let vec3 = Vector3D::new(2.0,3.0,4.0);
        assert_eq!(vec3 + Vector3D::new(2.0,3.0,4.0),Vector3D {x:4.0, y: 6.0, z:8.0});
      
    }

    #[test]
    fn sub_test() {
        let vec3 = Vector3D {x:4.0, y: 6.0, z:8.0};
        assert_eq!(vec3 - Vector3D::new(2.0,3.0,4.0),Vector3D::new(2.0,3.0,4.0));  
    }



