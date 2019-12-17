use super::{Angle, Distance};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn angle_to(&self, to: &Point) -> Angle {
        Angle::new(0.0)
    }

    pub fn distance_to(&self, to: &Point) -> Distance {
        0.0
    }

    pub fn layout_point(&self, angle: &Angle, distance: Distance) -> Point {
        Point::new(0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn angle_to() {
        let pnt = Point::new(0.0, 0.0);
        assert_eq!(pnt.angle_to(&Point::new(0.0, 1.0)), Angle::new(0.0));
        assert_eq!(pnt.angle_to(&Point::new(1.0, 1.0)), Angle::new(45.0));
        assert_eq!(pnt.angle_to(&Point::new(1.0, 0.0)), Angle::new(90.0));
        assert_eq!(pnt.angle_to(&Point::new(1.0, -1.0)), Angle::new(135.0));
        assert_eq!(pnt.angle_to(&Point::new(0.0, -1.0)), Angle::new(180.0));
        assert_eq!(pnt.angle_to(&Point::new(-1.0, -1.0)), Angle::new(225.0));
        assert_eq!(pnt.angle_to(&Point::new(-1.0, 0.0)), Angle::new(270.0));
        assert_eq!(pnt.angle_to(&Point::new(-1.0, 1.0)), Angle::new(315.0));
    }

    #[test]
    fn distance_to() {
        let pnt = Point::new(0.0, 0.0);
        assert_eq!(pnt.distance_to(&Point::new(1.0, 0.0)), 1.0);
        assert_eq!(pnt.distance_to(&Point::new(-2.0, 0.0)), 2.0);
        assert_eq!(pnt.distance_to(&Point::new(0.0, 3.0)), 3.0);
    }
}
