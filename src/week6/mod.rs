pub mod angle;
pub mod dot;

#[cfg(test)]
mod tests {
    use crate::week5::{vector::Vector, scalar::Scalar};


    #[test]
    fn q1a() {
        let u = Vector(vec![-2.0, 1.0, -2.0]);
        let v = Vector(vec![1.0, 2.0, 1.0]);

        assert_eq!((u * 3.0) - (v * 2.0), Vector(vec![-8.0, -1.0, -8.0]));
    }

    #[test]
    fn q1b() {
        let u = Vector(vec![-2.0, 1.0, -2.0]);
        let v = Vector(vec![1.0, 2.0, 1.0]);

        assert_eq!((u + (v * 3.0)).magnitude(), Scalar(51.0_f32.sqrt()));
    }

    #[test]
    fn q2() {
        let v = Vector(vec![2.0, -1.0, 3.0]);
        let u = v.unit_in_direction();

        assert_eq!(v / 14.0_f32.sqrt(), u);
    }

    #[test]
    fn q5() {
        let u = Vector(vec![1.0, 1.0, 4.0]);
        let v = Vector(vec![3.0, 1.0, -1.0]);

        assert_eq!(u * v, Scalar(0.0));
    }

    #[test]
    fn q6() {
        let u = Vector(vec![7.0, 0.0, 15.0]);
        let v = Vector(vec![0.0, 4.0, -2.0]);

        assert_eq!(v << u, Vector(vec![0.0, -6.0, 3.0]));
    }
}
