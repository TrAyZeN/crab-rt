use crate::vec::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3, // We could try to use a Cow here :thinking:
    direction: Vec3,
    time: f32,
}

impl Ray {
    #[inline]
    pub const fn new(origin: Vec3, direction: Vec3, time: f32) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    #[inline]
    pub const fn get_origin(&self) -> &Vec3 {
        &self.origin
    }

    #[inline]
    pub const fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    #[inline]
    pub const fn get_time(&self) -> f32 {
        self.time
    }

    #[inline]
    pub fn point(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

// use core::ops;

// impl ops::Fn<f32> for Ray {
//     type Output = Vec3;

//     fn call(self, t: f32) -> Self::Output {
//         self.origin + self.direction * t
//     }
// }
