use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub c: [f64; 3],
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.c[0] += rhs.c[0];
        self.c[1] += rhs.c[1];
        self.c[2] += rhs.c[2];
        self
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.c[0] -= rhs.c[0];
        self.c[1] -= rhs.c[1];
        self.c[2] -= rhs.c[2];
        self
    }
}
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(mut self) -> Self::Output {
        self.c[0] = -self.c[0];
        self.c[1] = -self.c[1];
        self.c[2] = -self.c[2];
        self
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self.c[0] *= rhs;
        self.c[1] *= rhs;
        self.c[2] *= rhs;
        self
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(mut self, rhs: f64) -> Self::Output {
        self.c[0] /= rhs;
        self.c[1] /= rhs;
        self.c[2] /= rhs;
        self
    }
}
impl Vec3 {
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.c[0] * rhs.c[0] + self.c[1] * rhs.c[1] + self.c[2] * rhs.c[2]
    }
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            c: [
                self.c[1] * rhs.c[2] - self.c[2] * rhs.c[1],
                self.c[2] * rhs.c[0] - self.c[0] * rhs.c[2],
                self.c[0] * rhs.c[1] - self.c[1] * rhs.c[0],
            ],
        }
    }
    pub fn len(&self)->f64{
        (self.c[0]*self.c[0]+self.c[1]*self.c[1]+self.c[2]*self.c[2]).sqrt()
    }
    pub fn normalize(self)->Self{
        self/self.len()
    }
    pub fn new(x:f64, y:f64, z:f64)->Self{
        Vec3{
            c:[
                x, y, z
            ]
        }
    }
    pub fn lin_comb(&self, v1:Vec3, v2:Vec3, v3:Vec3)->Vec3{
        let x1 = v1.c[0];
        let y1 = v1.c[1];
        let z1 = v1.c[2];

        let x2 = v2.c[0];
        let y2 = v2.c[1];
        let z2 = v2.c[2];

        let x3 = v3.c[0];
        let y3 = v3.c[1];
        let z3 = v3.c[2];

        let x4 = self.c[0];
        let y4 = self.c[1];
        let z4 = self.c[2];
        let h = -x1 * y2 * z3 + x1 * y3 * z2 + x2 * y1 * z3 - x2 * y3 * z1 - x3 * y1 * z2 + x3 * y2 * z1;
        let g1 = -(x2 * y3 * z4 - x2 * y4 * z3 - x3 * y2 * z4 + x3 * y4 * z2 + x4 * y2 * z3 - x4 * y3 * z2) / h;
        let g2 = -(-x1 * y3 * z4 + x1 * y4 * z3 + x3 * y1 * z4 - x3 * y4 * z1 - x4 * y1 * z3 + x4 * y3 * z1) / h;
        let g3 = -(x1 * y2 * z4 - x1 * y4 * z2 - x2 * y1 * z4 + x2 * y4 * z1 + x4 * y1 * z2 - x4 * y2 * z1) / h;
        Vec3::new(g1,g2,g3)
    }
}
