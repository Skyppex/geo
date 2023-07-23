use std::ops::{Mul, Add, Sub};

use num_traits::real::Real;
use super::traits::Pi;

fn interpolate<T>(a: T, b: T, t: T) -> T
where T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy {
    a + (b - a) * t
}

struct Ease;

impl Ease {
    pub fn clamp<T>(&self, t: T) -> T
    where T: PartialOrd + Real {
        t.round()
    }

    pub fn linear<T>(&self, t: T) -> T
    where T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy {
        t
    }

    pub fn sine_in<T>(&self, t: T) -> T
    where T: Real + Pi<Output = T> {
        T::one() - (t * T::pi() / (T::one() + T::one())).cos()
    }

    pub fn sine_out<T>(&self, t: T) -> T
    where T: Real + Pi<Output = T> {
        (t * T::pi() / (T::one() + T::one())).sin()
    }

    pub fn sine_inout<T>(&self, t: T) -> T
    where T: Real + Pi<Output = T> {
        -((t * T::pi()).cos() - T::one()) / (T::one() + T::one())
    }

    pub fn quad_in<T>(&self, t: T) -> T
    where T: Mul<Output = T> + Copy {
        t * t
    }

    pub fn quad_out<T>(&self, t: T) -> T
    where T: Real {
        let v = T::one() - t;
        T::one() - v * v
    }

    pub fn quad_inout<T>(&self, t: T) -> T
    where T: Real {
        let two = T::one() + T::one();
        if t < T::one() / two {
            two * t * t
        } else {
            let v = -two * t + two;
            T::one() - v * v / two
        }
    }

    pub fn cubic_in<T>(&self, t: T) -> T
    where T: Mul<Output = T> + Copy {
        t * t * t
    }

    pub fn cubic_out<T>(&self, t: T) -> T
    where T: Real {
        let v = T::one() - t;
        T::one() - v * v * v
    }

    pub fn cubic_inout<T>(&self, t: T) -> T
    where T: Real {
        let two = T::one() + T::one();
        if t < T::one() / two {
            two * t * t * t
        } else {
            let v = -two * t + two;
            T::one() - v * v * v / two
        }
    }

    pub fn quart_in<T>(&self, t: T) -> T
    where T: Mul<Output = T> + Copy {
        t * t * t * t
    }

    pub fn quart_out<T>(&self, t: T) -> T
    where T: Real {
        let v = T::one() - t;
        T::one() - v * v * v * v
    }

    pub fn quart_inout<T>(&self, t: T) -> T
    where T: Real {
        let two = T::one() + T::one();
        if t < T::one() / two {
            two * t * t * t * t
        } else {
            let v = -two * t + two;
            T::one() - v * v * v * v / two
        }
    }

    pub fn quint_in<T>(&self, t: T) -> T
    where T: Mul<Output = T> + Copy {
        t * t * t * t * t
    }

    pub fn quint_out<T>(&self, t: T) -> T
    where T: Real {
        let v = T::one() - t;
        T::one() - v * v * v * v * v
    }

    pub fn quint_inout<T>(&self, t: T) -> T
    where T: Real {
        let two = T::one() + T::one();
        if t < T::one() / two {
            two * t * t * t * t * t
        } else {
            let v = -two * t + two;
            T::one() - v * v * v * v * v / two
        }
    }

    pub fn sext_in<T>(&self, t: T) -> T
    where T: Mul<Output = T> + Copy {
        t * t * t * t * t * t
    }

    pub fn sext_out<T>(&self, t: T) -> T
    where T: Real {
        let v = T::one() - t;
        T::one() - v * v * v * v * v * v
    }

    pub fn sext_inout<T>(&self, t: T) -> T
    where T: Real {
        let two = T::one() + T::one();
        if t < T::one() / two {
            two * t * t * t * t * t * t
        } else {
            let v = -two * t + two;
            T::one() - v * v * v * v * v * v / two
        }
    }

    pub fn sept_in<T>(&self, t: T) -> T
    where T: Mul<Output = T> + Copy {
        t * t * t * t * t * t * t
    }

    pub fn sept_out<T>(&self, t: T) -> T
    where T: Real {
        let v = T::one() - t;
        T::one() - v * v * v * v * v * v * v
    }

    pub fn sept_inout<T>(&self, t: T) -> T
    where T: Real {
        let two = T::one() + T::one();
        if t < T::one() / two {
            two * t * t * t * t * t * t * t
        } else {
            let v = -two * t + two;
            T::one() - v * v * v * v * v * v * v / two
        }
    }

    pub fn oct_in<T>(&self, t: T) -> T
    where T: Mul<Output = T> + Copy {
        t * t * t * t * t * t * t * t
    }

    pub fn oct_out<T>(&self, t: T) -> T
    where T: Real {
        let v = T::one() - t;
        T::one() - v * v * v * v * v * v * v * v
    }

    pub fn oct_inout<T>(&self, t: T) -> T
    where T: Real {
        let two = T::one() + T::one();
        if t < T::one() / two {
            two * t * t * t * t * t * t * t * t
        } else {
            let v = -two * t + two;
            T::one() - v * v * v * v * v * v * v * v / two
        }
    }

    pub fn expo_in<T>(&self, t: T) -> T
    where T: Real {
        if t == T::zero() {
            T::zero()
        } else {
            let two = T::one() + T::one();
            let ten = two + two + two + two + two;
            two.powf(ten * t - ten)
        }
    }

    pub fn expo_out<T>(&self, t: T) -> T
    where T: Real {
        if t == T::one() {
            T::one()
        } else {
            let two = T::one() + T::one();
            let ten = two + two + two + two + two;
            T::one() - two.powf(-ten * t)
        }
    }

    pub fn expo_inout<T>(&self, t: T) -> T
    where T: Real {
        if t == T::zero() {
            T::zero()
        } else if t == T::one() {
            T::one()
        } else {
            let two = T::one() + T::one();
            let ten = two + two + two + two + two;
            if t < T::one() / two {
                two.powf((ten + ten) * t - ten) / two
            } else {
                two - two.powf(-(ten + ten) * t + ten) / two
            }
        }
    }

    pub fn circ_in<T>(&self, t: T) -> T
    where T: Real {
        T::one() - (T::one() - t * t).sqrt()
    }

    pub fn circ_out<T>(&self, t: T) -> T
    where T: Real {
        let v = t - T::one();
        (T::one() - v * v).sqrt()
    }

    pub fn circ_inout<T>(&self, t: T) -> T
    where T: Real {
        let two = T::one() + T::one();
        if t < T::one() / two {
            let v = two * t;
            (T::one() - (T::one() - v * v).sqrt()) / two
        } else {
            let v = -two * t + two;
            ((T::one() - v * v).sqrt() + T::one()) / two
        }
    }

    pub fn back_in<T>(&self, t: T) -> T
    where T: Real {
        let c1 = T::from(1.70158).unwrap();
        let c2 = c1 + T::one();
        c2 * t * t * t - c1 * t * t
    }

    pub fn back_out<T>(&self, t: T) -> T
    where T: Real {
        let c1 = T::from(1.70158).unwrap();
        let c2 = c1 + T::one();
        let v = t - T::one();
        c2 * v * v * v + c1 * v * v + T::one()
    }

    pub fn back_inout<T>(&self, t: T) -> T
    where T: Real {
        let c3 = T::from(3.22658).unwrap();

        let two = T::one() + T::one();

        if t < T::one() / two {
            let v = two * t;
            v * v * ((c3 + T::one()) * two * t - c3) / two
        } else {
            let v = two * t - two;
            v * v * (((c3 + T::one()) * v + c3) + two) / two
        }
    }

    pub fn elastic_in<T>(&self, t: T) -> T
    where T: Real + Pi<Output = T> {
        let two = T::one() + T::one();
        let three = two + T::one();
        let four = two + two;
        let c4 = two * T::pi() / (T::one() / three);
        let ten = four + four + two;
        let ten_and_three_quarters = ten + three / four;

        if t == T::zero() {
            T::zero()
        } else if t == T::one() {
            T::one()
        } else {
            -two.powf(ten * t - ten) * ((t * ten - ten_and_three_quarters) * c4).sin()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn interpolate() {
        use super::interpolate;
        
        assert_eq!(interpolate(0.0, 1.0, 0.5), 0.5);
        assert_eq!(interpolate(0.0, 1.0, 0.0), 0.0);
        assert_eq!(interpolate(0.0, 1.0, 1.0), 1.0);
        assert_eq!(interpolate(0.0, 1.0, 0.25), 0.25);
        assert_eq!(interpolate(0.0, 1.0, 0.75), 0.75);
    }

    fn ease_linear() {
        use super::Ease;
        
        assert_eq!(Ease.linear(0.0), 0.0);
        assert_eq!(Ease.linear(0.5), 0.5);
        assert_eq!(Ease.linear(1.0), 1.0);
        assert_eq!(Ease.linear(0.25), 0.25);
        assert_eq!(Ease.linear(0.75), 0.75);
    }
}