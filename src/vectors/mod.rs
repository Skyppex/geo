use std::ops::{DivAssign, Add, Mul, Neg, Index, IndexMut, Sub, Div, AddAssign, SubAssign, MulAssign, Deref};

use num_traits::real::Real;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    pub fn normalized(&self) -> Self
    where T: Add<Output = T> + Mul<Output = T> + Real + DivAssign {
        Self::normalize(self)
    }

    pub fn sqr_magnitude(&self) -> T
    where T: Mul<Output = T> + Add<Output = T> + Copy {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> T
    where T: Mul<Output = T> + Add<Output = T> + Real {
        self.sqr_magnitude().sqrt()
    }

    pub fn distance(left: Self, right: Self) -> T
    where T: Mul<Output = T> + Add<Output = T> + Real {
        (left - right).magnitude()
    }

    pub fn sqr_distance(left: Self, right: Self) -> T
    where T: Mul<Output = T> + Add<Output = T> + Real {
        (left - right).sqr_magnitude()
    }

    pub fn scale(a: Self, b: Self) -> Self
    where T: Mul<Output = T> {
        Self { x: a.x * b.x, y: a.y * b.y }
    }

    pub fn normalize(vector: &Self) -> Self
    where T: Add<Output = T> + Mul<Output = T> + Real + DivAssign {
        let mut x = vector.x;
        let mut y = vector.y;

        let length = (x * x + y * y).sqrt();

        x /= length;
        y /= length;

        Self { x, y }
    }

    pub fn dot(left: Self, right: Self) -> T
    where T: Mul<Output = T> + Add<Output = T> {
        left.x * right.x + left.y * right.y
    }

    pub fn reflect(direction: Self, normal: Self) -> Self
    where T: Mul<Output = T> + Add<Output = T> + Real + Copy {
        let factor = Self::dot(direction, normal) * (T::one() + T::one());
        Self {x: normal.x * factor + direction.x, y: normal.y * factor + direction.y }
    }

    pub fn perpendicular(vector: Self) -> Self
    where T: Neg<Output = T> {
        Self { x: -vector.y, y: vector.x }
    }

    pub fn move_towards(current: Self, target: Self, max_distance_delta: T) -> Self
    where T: Sub<Output = T> + Mul<Output = T> + Add<Output = T> + DivAssign + MulAssign + Real + Copy {
        let mut movement = target - current;
        let sqr_magnitude = movement.sqr_magnitude();

        if sqr_magnitude > max_distance_delta * max_distance_delta {
            let magnitude = sqr_magnitude.sqrt();
            movement.x /= magnitude;
            movement.y /= magnitude;

            movement.x *= max_distance_delta;
            movement.y *= max_distance_delta;

            current + movement
        } else {
            target
        }
    }
}

impl<T> Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds")
        }
    }
}

impl<T> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds")
        }
    }
}

impl<T> Neg for Vector2<T>
where T: Neg<Output = T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl<T> Add<Vector2<T>> for Vector2<T>
where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<'a, T> Add<&'a Vector2<T>> for Vector2<T>
where T: Add<Output = T> + Copy {
    type Output = Self;

    fn add(self, rhs: &'a Vector2<T>) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T> Sub<Vector2<T>> for Vector2<T>
where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<'a, T> Sub<&'a Vector2<T>> for Vector2<T>
where T: Sub<Output = T> + Copy {
    type Output = Self;

    fn sub(self, rhs: &'a Vector2<T>) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T> Mul<T> for Vector2<T>
where T: Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<'a, T> Mul<&'a T> for Vector2<T>
where T: Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: &'a T) -> Self::Output {
        Self { x: self.x * *rhs, y: self.y * *rhs }
    }
}

impl<T> Mul<Vector2<T>> for Vector2<T>
where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl<'a, T> Mul<&'a Vector2<T>> for Vector2<T>
where T: Mul<Output = T> + Deref<Target = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: &'a Vector2<T>) -> Self::Output {
        Self { x: self.x * *rhs.x, y: self.y * *rhs.y }
    }
}

impl<T> Div<T> for Vector2<T>
where T: Div<Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<'a, T> Div<&'a T> for Vector2<T>
where T: Div<Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: &'a T) -> Self::Output {
        Self { x: self.x / *rhs, y: self.y / *rhs }
    }
}

impl<T> Div<Vector2<T>> for Vector2<T>
where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl<'a, T> Div<&'a Vector2<T>> for Vector2<T>
where T: Div<Output = T> + Deref<Target = T> + Copy {
    type Output = Self;

    fn div(self, rhs: &'a Vector2<T>) -> Self::Output {
        Self { x: self.x / *rhs.x, y: self.y / *rhs.y }
    }
}

impl<T> AddAssign<Vector2<T>> for Vector2<T>
where T: AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<'a, T> AddAssign<&'a Vector2<T>> for Vector2<T>
where T: AddAssign + Copy {
    fn add_assign(&mut self, rhs: &'a Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign<Vector2<T>> for Vector2<T>
where T: SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<'a, T> SubAssign<&'a Vector2<T>> for Vector2<T>
where T: SubAssign + Copy {
    fn sub_assign(&mut self, rhs: &'a Vector2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign<T> for Vector2<T>
where T: MulAssign + Copy {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<'a, T> MulAssign<&'a T> for Vector2<T>
where T: MulAssign + Copy {
    fn mul_assign(&mut self, rhs: &'a T) {
        self.x *= *rhs;
        self.y *= *rhs;
    }
}

impl<T> MulAssign<Vector2<T>> for Vector2<T>
where T: MulAssign {
    fn mul_assign(&mut self, rhs: Vector2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<'a, T> MulAssign<&'a Vector2<T>> for Vector2<T>
where T: MulAssign + Deref<Target = T> + Copy {
    fn mul_assign(&mut self, rhs: &'a Vector2<T>) {
        self.x *= *rhs.x;
        self.y *= *rhs.y;
    }
}

impl<T> DivAssign<T> for Vector2<T>
where T: DivAssign + Copy {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<'a, T> DivAssign<&'a T> for Vector2<T>
where T: DivAssign + Copy {
    fn div_assign(&mut self, rhs: &'a T) {
        self.x /= *rhs;
        self.y /= *rhs;
    }
}

impl<T> DivAssign<Vector2<T>> for Vector2<T>
where T: DivAssign {
    fn div_assign(&mut self, rhs: Vector2<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<'a, T> DivAssign<&'a Vector2<T>> for Vector2<T>
where T: DivAssign + Deref<Target = T> + Copy {
    fn div_assign(&mut self, rhs: &'a Vector2<T>) {
        self.x /= *rhs.x;
        self.y /= *rhs.y;
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from(tuple: (T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1 }
    }
}

impl<T> From<[T; 2]> for Vector2<T>
where T: Copy {
    fn from(array: [T; 2]) -> Self {
        Self { x: array[0], y: array[1] }
    }
}

impl<T> From<[[T; 1]; 2]> for Vector2<T>
where T: Copy {
    fn from(array: [[T; 1]; 2]) -> Self {
        Self { x: array[0][0], y: array[0][1] }
    }
}

impl<T> From<Vector2<T>> for (T, T) {
    fn from(vector: Vector2<T>) -> Self {
        (vector.x, vector.y)
    }
}

impl<T> From<Vector2<T>> for [T; 2]
where T: Copy {
    fn from(vector: Vector2<T>) -> Self {
        [vector.x, vector.y]
    }
}

impl<T> From<Vector2<T>> for [[T; 1]; 2]
where T: Copy {
    fn from(vector: Vector2<T>) -> Self {
        [[vector.x], [vector.y]]
    }
}

impl<T> IntoIterator for Vector2<T> {
    type Item = T;

    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y].into_iter()
    }
}

impl<T> FromIterator<T> for Vector2<T>
where T: Copy {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        if let Some(x) = iter.next() {
            if let Some(y) = iter.next() {
                return Self { x, y };
            }
        }
        
        panic!("Vector2::from_iter needs at least 2 elements")
    }
}





#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T
}





#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector4<T> {
    x: T,
    y: T,
    z: T,
    w: T
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector2_set() {
        let mut vector = Vector2::new(2, 2);
        vector.set(2, 3);
        assert_eq!(vector.x, 2);
        assert_eq!(vector.y, 3);

        vector.set(5, 5);
        assert_eq!(vector.x, 5);
        assert_eq!(vector.y, 5);
    }
}