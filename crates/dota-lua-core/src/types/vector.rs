use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Div for Vector {
    type Output = Self;
    fn div(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl PartialEq for Vector {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
    fn ne(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl Sub for Vector {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl Mul for Vector {
    type Output = Self;
    fn mul(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl ToString for Vector {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}
impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl Vector {
    pub fn len(&self) -> f64 { unimplemented!() }
    pub fn cross(&self) -> Self { unimplemented!() }
    pub fn dot(&self) -> f64 { unimplemented!() }
    pub fn length2d(&self) -> f64 { unimplemented!() }
    pub fn normalized(&self) -> Self { unimplemented!() }
    pub fn lerp(&self) -> Self { unimplemented!() }
}
