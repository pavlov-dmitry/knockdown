use super::{Angle, Distance};
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn angle_to(&self, to: &Point) -> Angle {
        let a = to.x - self.x;
        let b = to.y - self.y;
        Angle::from_radians(a.atan2(b))
    }

    pub fn distance_to(&self, to: &Point) -> Distance {
        let div_x = to.x - self.x;
        let div_y = to.y - self.y;
        let distance = (div_x * div_x + div_y * div_y).sqrt();
        distance
    }

    pub fn layout_point(&self, angle: &Angle, distance: Distance) -> Point {
        let x = self.x + angle.sin() * distance;
        let y = self.y + angle.cos() * distance;
        Point::new(x, y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let div_x = (self.x - other.x).abs();
        let div_y = (self.y - other.y).abs();
        div_x <= std::f32::EPSILON && div_y <= std::f32::EPSILON
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

    #[test]
    fn layout_point() {
        let pnt = Point::new(0.0, 0.0);
        let soft_compare = |p1: Point, p2: Point| {
            let x_ok = (p1.x - p2.x).abs() < 0.001;
            let y_ok = (p1.y - p2.y).abs() < 0.001;
            assert!(x_ok && y_ok)
        };
        soft_compare(
            pnt.layout_point(&Angle::new(90.0), 10.0),
            Point::new(10.0, 0.0),
        );
        soft_compare(
            pnt.layout_point(&Angle::new(0.0), 20.0),
            Point::new(0.0, 20.0),
        );
        soft_compare(
            pnt.layout_point(&Angle::new(180.0), 30.0),
            Point::new(0.0, -30.0),
        );
        soft_compare(
            pnt.layout_point(&Angle::new(270.0), 40.0),
            Point::new(-40.0, 0.0),
        );
    }
}
