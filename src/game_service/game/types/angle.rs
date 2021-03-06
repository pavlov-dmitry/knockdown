use std::convert::From;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Angle(pub f32);

impl Angle {
    pub fn new(value: f32) -> Angle {
        Angle(value).normalize()
    }

    pub fn from_radians(value: f32) -> Angle {
        Angle(value.to_degrees()).normalize()
    }

    fn normalize(self) -> Self {
        let Angle(val) = self;
        let val = val % 360.0;
        let val = if val < 0.0 { val + 360.0 } else { val };
        Angle(val)
    }

    pub fn sin(&self) -> f32 {
        let Angle(val) = self;
        val.to_radians().sin()
    }

    pub fn cos(&self) -> f32 {
        let Angle(val) = self;
        val.to_radians().cos()
    }

    pub fn opposite(&self) -> Angle {
        self.clone() + 180.0
    }
}

impl Add<f32> for Angle {
    type Output = Angle;

    fn add(self, other: f32) -> Self::Output {
        let Angle(val) = self;
        let val = val + other;
        Angle::new(val)
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;
    fn sub(self, other: Angle) -> Self::Output {
        let Angle(val) = self;
        let Angle(other_val) = other;
        Angle::new(val - other_val)
    }
}

impl Sub<f32> for Angle {
    type Output = Angle;

    fn sub(self, other: f32) -> Self::Output {
        let Angle(val) = self;
        let val = val - other;
        Angle::new(val)
    }
}

impl From<f32> for Angle {
    fn from(val: f32) -> Angle {
        Angle::new(val)
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        let Angle(val) = self;
        let Angle(other_val) = other;
        (val - other_val).abs() < std::f32::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normilize() {
        assert_eq!(Angle(50.0).normalize(), Angle(50.0));
        assert_eq!(Angle(-60.0).normalize(), Angle(300.0));
        assert_eq!(Angle(380.0).normalize(), Angle(20.0));
        assert_eq!(Angle(730.0).normalize(), Angle(10.0));
        assert_eq!(Angle(-730.0).normalize(), Angle(350.0));
    }
}
