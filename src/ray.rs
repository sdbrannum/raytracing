use crate::vec3::Vec3;
use std::rc::Rc;

#[derive(Debug)]
pub struct Ray {
    A: Rc<Vec3>,
    B: Rc<Vec3>,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {
            A: Rc::new(a),
            B: Rc::new(b),
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.A
    }

    pub fn direction(&self) -> &Vec3 {
        &self.B
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        *self.A + (*self.B * t)
    }
}
