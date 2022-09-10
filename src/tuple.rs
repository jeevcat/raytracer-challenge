use std::ops::{Add, Div, Mul, Neg, Sub};

use approx::relative_eq;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Scalar(f64);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.x, other.x)
            && relative_eq!(self.y, other.y)
            && relative_eq!(self.z, other.z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.x, other.x)
            && relative_eq!(self.y, other.y)
            && relative_eq!(self.z, other.z)
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.0, other.0)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// TODO does it matter that w != -1?
impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Scalar> for Vector {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Vector {
            x: self.x * rhs.0,
            y: self.y * rhs.0,
            z: self.z * rhs.0,
        }
    }
}

impl Div<Scalar> for Vector {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        Vector {
            x: self.x / rhs.0,
            y: self.y / rhs.0,
            z: self.z / rhs.0,
        }
    }
}

impl Vector {
    pub fn magnitude(&self) -> Scalar {
        Scalar((self.x * self.x + self.y * self.y + self.z * self.z).sqrt())
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude().0;
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn adding_point_vector() {
        let p = Point {
            x: 3.,
            y: -2.,
            z: 5.,
        };
        let v = Vector {
            x: -2.,
            y: 3.,
            z: 1.,
        };

        assert_eq!(
            p + v,
            Point {
                x: 1.,
                y: 1.,
                z: 6.,
            }
        )
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point {
            x: 3.,
            y: 2.,
            z: 1.,
        };
        let p2 = Point {
            x: 5.,
            y: 6.,
            z: 7.,
        };

        assert_eq!(
            p1 - p2,
            Vector {
                x: -2.,
                y: -4.,
                z: -6.,
            }
        )
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Point {
            x: 3.,
            y: 2.,
            z: 1.,
        };
        let v = Vector {
            x: 5.,
            y: 6.,
            z: 7.,
        };

        assert_eq!(
            p - v,
            Point {
                x: -2.,
                y: -4.,
                z: -6.,
            }
        )
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector {
            x: 3.,
            y: 2.,
            z: 1.,
        };
        let v2 = Vector {
            x: 5.,
            y: 6.,
            z: 7.,
        };

        assert_eq!(
            v1 - v2,
            Vector {
                x: -2.,
                y: -4.,
                z: -6.,
            }
        )
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Vector {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let v = Vector {
            x: 1.,
            y: -2.,
            z: 3.,
        };

        assert_eq!(
            zero - v,
            Vector {
                x: -1.,
                y: 2.,
                z: -3.,
            }
        )
    }

    #[test]
    fn negating_a_vector() {
        let v = Vector {
            x: 1.,
            y: -2.,
            z: 3.,
        };

        assert_eq!(
            -v,
            Vector {
                x: -1.,
                y: 2.,
                z: -3.,
            }
        )
    }

    #[test]
    fn mutiplying_a_vector_by_a_scalar() {
        let v = Vector {
            x: 1.,
            y: -2.,
            z: 3.,
        };
        assert_eq!(
            v * Scalar(3.5),
            Vector {
                x: 3.5,
                y: -7.,
                z: 10.5
            }
        )
    }

    #[test]
    fn mutiplying_a_vector_by_a_fraction() {
        let v = Vector {
            x: 1.,
            y: -2.,
            z: 3.,
        };
        assert_eq!(
            v * Scalar(0.5),
            Vector {
                x: 0.5,
                y: -1.,
                z: 1.5
            }
        )
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let v = Vector {
            x: 1.,
            y: -2.,
            z: 3.,
        };
        assert_eq!(
            v / Scalar(2.),
            Vector {
                x: 0.5,
                y: -1.,
                z: 1.5
            }
        )
    }

    #[test]
    fn computing_the_magnitude_of_vector() {
        let v = Vector {
            x: 1.,
            y: 0.,
            z: 0.,
        };
        assert_eq!(v.magnitude(), Scalar(1.));

        let v = Vector {
            x: 0.,
            y: 1.,
            z: 0.,
        };
        assert_eq!(v.magnitude(), Scalar(1.));

        let v = Vector {
            x: 0.,
            y: 0.,
            z: 1.,
        };
        assert_eq!(v.magnitude(), Scalar(1.));

        let v = Vector {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        assert_eq!(v.magnitude(), Scalar(14f64.sqrt()));

        let v = Vector {
            x: -1.,
            y: -2.,
            z: -3.,
        };
        assert_eq!(v.magnitude(), Scalar(14f64.sqrt()));
    }

    #[test]
    fn normalizing_vector() {
        let v = Vector {
            x: 4.,
            y: 0.,
            z: 0.,
        };
        assert_eq!(
            v.normalize(),
            Vector {
                x: 1.,
                y: 0.,
                z: 0.
            }
        );

        let v = Vector {
            x: 1.,
            y: 2.,
            z: 3.,
        };
            let norm = v.normalize();
        assert_eq!(
            norm,
            Vector {
                x: 1. / 14f64.sqrt(),
                y: 2. / 14f64.sqrt(),
                z: 3. / 14f64.sqrt(),
            }
        );
        assert_eq!(norm.magnitude(), Scalar(1.));
    }
}
