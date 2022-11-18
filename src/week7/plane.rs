use std::f64::EPSILON;

use crate::week5::{scalar::Scalar as S, vector::Vector};
use crate::week7::point::Point3;

impl From<S> for f64 {
    fn from(s: S) -> Self {
        s.0
    }
}

#[derive(Debug)]
pub struct Plane3 {
    pub point: Point3,
    pub normal: Vector,
}

#[derive(Debug)]
pub enum Intersection3 {
    Coincident,
    Parallel,
    Point(Point3),
}

#[derive(Debug)]
pub enum TwoValues {
    YZ(S, S),
    XZ(S, S),
    XY(S, S),
}

impl Plane3 {
    pub fn from_point_and_normal(point: Point3, normal: Vector) -> Plane3 {
        Plane3 {
            point,
            normal: normal.into_unit_in_direction(),
        }
    }

    pub fn from_points(p1: Point3, p2: Point3, p3: Point3) -> Plane3 {
        Plane3 {
            normal: ((p2 - p1) ^ (p3 - p1)).into_unit_in_direction(),
            point: p1,
        }
    }

    // ax + by + cz = d
    pub fn from_equation(mut ax: f64, mut by: f64, mut cz: f64, mut rhs: f64) -> Plane3 {
        let magnitude = (ax * ax + by * by + cz * cz).sqrt();
        ax /= magnitude;
        by /= magnitude;
        cz /= magnitude;
        rhs /= magnitude;
        // let t = rhs / (3.0 * ax * by * cz);

        Plane3 {
            normal: Vector(vec![ax, by, cz]),
            point: Point3(ax * rhs, by * rhs, cz * rhs),
        }
    }

    pub fn contains(&self, point: &Point3) -> bool {
        // satisfies:
        // ax + by + cz = d
        // where normal = <a, b, c>
        // and d = (self.point - &Point3::ZERO).dotted(&self.normal)
        ((point - &Point3::ZERO).dotted(&self.normal)
            - (self.point - &Point3::ZERO).dotted(&self.normal))
        .0 <= EPSILON
    }

    pub fn as_equation(&self) -> (&Vec<f64>, S) {
        // a(x - p0) + b(y - p1) + c(z - p2) = 0
        // ax - a*p0 + by - b*p1 + cz - c*p2 = 0
        // ax + by + cz = a*p0 + b*p1 + c*p2 = n.(p - 0)
        (
            &self.normal.0,
            (self.point - &Point3::ZERO).dotted(&self.normal),
        )
    }

    // return the third value obtained by plugging the first two into the eqn
    // ax + by + cz = d
    pub fn plug_values(&self, values: TwoValues) -> Option<S> {
        let (coefficients, d) = self.as_equation();
        if let [a, b, c] = coefficients[0..3] {
            match values {
                // (ax + by - d)/c = z
                TwoValues::YZ(y, z) => (a != 0.0).then(|| (b * y + c * z - d) / a),
                TwoValues::XZ(x, z) => (b != 0.0).then(|| (a * x + c * z - d) / b),
                TwoValues::XY(x, y) => (c != 0.0).then(|| (a * x + b * y - d) / c),
            }
        } else {
            panic!()
        }
    }

    pub fn axis_intersects(&self) -> (Intersection3, Intersection3, Intersection3) {
        let on_origin = self.contains(&Point3::ZERO);
        (
            match self.plug_values(TwoValues::YZ(S(0.0), S(0.0))) {
                Some(x) => Intersection3::Point(Point3(x.into(), 0.0, 0.0)),
                None => {
                    if on_origin {
                        Intersection3::Coincident
                    } else {
                        Intersection3::Parallel
                    }
                }
            },
            match self.plug_values(TwoValues::YZ(S(0.0), S(0.0))) {
                Some(y) => Intersection3::Point(Point3(0.0, y.into(), 0.0)),
                None => {
                    if on_origin {
                        Intersection3::Coincident
                    } else {
                        Intersection3::Parallel
                    }
                }
            },
            match self.plug_values(TwoValues::YZ(S(0.0), S(0.0))) {
                Some(z) => Intersection3::Point(Point3(0.0, 0.0, z.into())),
                None => {
                    if on_origin {
                        Intersection3::Coincident
                    } else {
                        Intersection3::Parallel
                    }
                }
            },
        )
    }

    pub fn is_orthogonal(&self, other: &Plane3) -> bool {
        self.normal.dotted(&other.normal) == S(0.0)
    }

    pub fn is_parallel(&self, other: &Plane3) -> bool {
        (&self.normal ^ &other.normal).magnitude_squared().0 <= EPSILON
    }
}

impl PartialEq<Plane3> for Plane3 {
    fn eq(&self, other: &Plane3) -> bool {
        self.contains(&other.point) && (self.is_parallel(other))
    }
}

#[cfg(test)]
mod test {
    use crate::{week5::vector::Vector, week7::point::Point3};

    use super::Plane3;

    #[test]
    fn contains() {
        let plane = Plane3 {
            point: Point3(1.0, 0.0, -1.0),
            normal: Vector(vec![-4.0, 5.0, -3.0]),
        };

        assert!(plane.contains(&plane.point));
        assert!(plane.contains(&Point3(1.0, 0.0, -1.0)));
    }

    #[test]
    fn equality() {
        let plane = Plane3 {
            point: Point3(1.0, 0.0, -1.0),
            normal: Vector(vec![-4.0, 5.0, -3.0]),
        };
        let other = Plane3 {
            point: Point3(1.0, 0.0, -1.0),
            normal: Vector(vec![-4.0, 5.0, -3.0]),
        };

        assert!(plane.contains(&plane.point));

        assert!(plane.contains(&other.point));
        assert!(plane.is_parallel(&other));

        assert_eq!(plane, other);
    }

    #[test]
    fn construction() {
        let plane = Plane3::from_points(
            Point3(1.0, 0.0, -1.0),
            Point3(0.0, 1.0, 2.0),
            Point3(3.0, 1.0, -2.0),
        );

        let actual =
            Plane3::from_point_and_normal(Point3(1.0, 0.0, -1.0), Vector(vec![-4.0, 5.0, -3.0]));

        assert!(plane.contains(&actual.point));
        assert!(plane.is_parallel(&actual));

        assert_eq!(plane, actual);

        let plane2 = Plane3::from_points(
            Point3(0.0, 1.0, 2.0),
            Point3(1.0, 0.0, -1.0),
            Point3(3.0, 1.0, -2.0),
        );
        let plane3 = Plane3::from_points(
            Point3(3.0, 1.0, -2.0),
            Point3(1.0, 0.0, -1.0),
            Point3(0.0, 1.0, 2.0),
        );

        assert_eq!(plane, plane2);
        assert_eq!(plane2, plane3);
        assert_eq!(plane, plane3);

        let eqn = plane.as_equation();
        println!(
            "equation: ({}x) + ({}y) + ({}z) = {}",
            eqn.0[0], eqn.0[1], eqn.0[2], eqn.1 .0
        );
        if let [a, b, c] = eqn.0[0..3] {
            assert_eq!(plane, Plane3::from_equation(a, b, c, eqn.1.into()))
        } else {
            panic!()
        }
    }
}
