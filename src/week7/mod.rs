pub mod cross;
pub mod determinant;
pub mod plane;
pub mod point;

#[cfg(test)]
mod test {
    use crate::{week5::vector::Vector, week7::point::Point3};

    use super::plane::Plane3;

    #[test]
    fn q1() {
        let u = Vector(vec![3.0, -4.0, 1.0]);
        let v = Vector(vec![-1.0, 2.0, 5.0]);

        assert_eq!(&u ^ &u, Vector::from_points(Point3::ZERO, Point3::ZERO));
        assert_eq!(&u ^ &v, Vector(vec![-22.0, -16.0, 2.0]));
        assert_eq!(&u ^ &v, -(&v ^ &u));
    }

    #[test]
    fn q3() {
        let plane = Plane3::from_point_and_normal(
            Point3(-3.0, 1.0, 2.0),
            Plane3::from_equation(1., 1., 1., 0.).normal
                ^ Plane3::from_equation(0., 2., -1., 0.).normal,
        );
        let eqn = plane.as_equation();
        println!(
            "equation: ({}x) + ({}y) + ({}z) = {}",
            eqn.0[0], eqn.0[1], eqn.0[2], eqn.1 .0
        );
        assert_eq!(plane, Plane3::from_equation(-3., 1., 2., 14.))
    }
}
