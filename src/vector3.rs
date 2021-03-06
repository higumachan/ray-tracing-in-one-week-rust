use crate::to_pixel_value;
use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Neg, Range, Sub, SubAssign};

const EPS: f64 = 1e-8;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    pub fn new_x(x: f64) -> Self {
        Self::new(x, 0.0, 0.0)
    }
    pub fn new_y(y: f64) -> Self {
        Self::new(0.0, y, 0.0)
    }
    pub fn new_z(z: f64) -> Self {
        Self::new(0.0, 0.0, z)
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn sqrt(&self) -> Self {
        Self::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn random(rng: &mut ThreadRng) -> Self {
        Vector3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range<R: RngCore>(rng: &mut R, range: Range<f64>) -> Self {
        Vector3::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }

    pub fn random_in_unit_sphere<R: RngCore>(rng: &mut R) -> Self {
        loop {
            let p = Self::random_range(
                rng,
                Range {
                    start: -1.0,
                    end: 1.0,
                },
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector<R: RngCore>(rng: &mut R) -> Self {
        Self::random_in_unit_sphere(rng).unit_vector()
    }

    pub fn approx_zero(&self) -> bool {
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        (&self).div(rhs)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Sum<Vector3> for Vector3 {
    fn sum<I: Iterator<Item = Vector3>>(iter: I) -> Self {
        let mut s = Self::zero();
        for v in iter {
            s += v;
        }
        s
    }
}

impl From<Color> for Vector3 {
    fn from(c: Color) -> Self {
        c.0
    }
}

impl From<Point3> for Vector3 {
    fn from(p: Point3) -> Self {
        p.0
    }
}

impl<'a> From<&'a Point3> for &'a Vector3 {
    fn from(p: &'a Point3) -> Self {
        &p.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point3(Vector3);

impl From<Vector3> for Point3 {
    fn from(v: Vector3) -> Self {
        Self(v)
    }
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vector3 { x, y, z })
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
    pub fn z(&self) -> f64 {
        self.0.z
    }

    pub fn new_x(x: f64) -> Self {
        Self::new(x, 0.0, 0.0)
    }
    pub fn new_y(y: f64) -> Self {
        Self::new(0.0, y, 0.0)
    }
    pub fn new_z(z: f64) -> Self {
        Self::new(0.0, 0.0, z)
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Add<Vector3> for &Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Add<&Vector3> for &Point3 {
    type Output = Point3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Self::Output::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl Sub<&Vector3> for &Point3 {
    type Output = Point3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Self::Output::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Sub<&Point3> for &Point3 {
    type Output = Vector3;

    fn sub(self, rhs: &Point3) -> Self::Output {
        Self::Output::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color(Vector3);

impl From<Vector3> for Color {
    fn from(v: Vector3) -> Self {
        Self(v)
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vector3 { x: r, y: g, z: b })
    }
    pub fn r(&self) -> f64 {
        self.0.x
    }
    pub fn g(&self) -> f64 {
        self.0.y
    }
    pub fn b(&self) -> f64 {
        self.0.z
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            to_pixel_value(self.r()),
            to_pixel_value(self.g()),
            to_pixel_value(self.b())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vec() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);

        assert_eq!(v1 + v2, Vector3::new(1.0, 1.0, 0.0))
    }

    #[test]
    fn cross_dot() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);

        assert_eq!(v1.dot(&v2), 0.0);
        let v3 = v1.cross(&v2);
        assert_eq!(v1.dot(&v3), 0.0);
        assert_eq!(v2.dot(&v3), 0.0);
    }
}
