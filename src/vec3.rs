use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn make_unit_vector(&mut self) {
        let k: f32 = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn cross(&self, v2: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * v2.e[2] - self.e[2] * v2.e[1],
            -(self.e[0] * v2.e[2] - self.e[2] * v2.e[0]),
            self.e[0] * v2.e[1] - self.e[1] * v2.e[0],
        )
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, v2: Vec3) -> Self {
        Vec3::new(
            self.e[0] + v2.e[0],
            self.e[1] + v2.e[1],
            self.e[2] + v2.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, v2: Vec3) -> Self {
        Vec3::new(
            self.e[0] - v2.e[0],
            self.e[1] - v2.e[1],
            self.e[2] - v2.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, v2: Vec3) -> Self {
        Vec3::new(
            self.e[0] * v2.e[0],
            self.e[1] * v2.e[1],
            self.e[2] * v2.e[2],
        )
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, v2: Vec3) -> Self {
        Vec3::new(
            self.e[0] / v2.e[0],
            self.e[1] / v2.e[1],
            self.e[2] / v2.e[2],
        )
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, t: f32) -> Self {
        Vec3::new(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}
