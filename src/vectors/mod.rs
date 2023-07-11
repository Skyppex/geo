use std::ops::{DivAssign, Add, Mul, Neg, Index, IndexMut, Sub, Div, AddAssign, SubAssign, MulAssign, Deref};

use num_traits::{real::Real, Float};

pub type Vector2f32 = Vector2<f32>;
pub type Vector2f64 = Vector2<f64>;
pub type Vector2i32 = Vector2<i32>;
pub type Vector2i64 = Vector2<i64>;
pub type Vector2u32 = Vector2<u32>;
pub type Vector2u64 = Vector2<u64>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vector2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn zero() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::zero() }
    }

    #[inline]
    pub fn one() -> Self
    where T: Real {
        Self { x: T::one(), y: T::one() }
    }

    #[inline]
    pub fn right() -> Self
    where T: Real {
        Self { x: T::one(), y: T::zero() }
    }

    #[inline]
    pub fn left() -> Self
    where T: Real {
        Self { x: -T::one(), y: T::zero() }
    }

    #[inline]
    pub fn up() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::one() }
    }

    #[inline]
    pub fn down() -> Self
    where T: Real {
        Self { x: T::zero(), y: -T::one() }
    }

    #[inline]
    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    #[inline]
    pub fn normalized(&self) -> Self
    where T: Real + DivAssign {
        Self::normalize(self)
    }

    #[inline]
    pub fn sqr_magnitude(&self) -> T
    where T: Mul<Output = T> + Add<Output = T> + Copy {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn magnitude(&self) -> T
    where T: Real {
        self.sqr_magnitude().sqrt()
    }

    #[inline]
    pub fn distance(left: Self, right: Self) -> T
    where T: Real {
        (left - right).magnitude()
    }

    #[inline]
    pub fn sqr_distance(left: Self, right: Self) -> T
    where T: Real {
        (left - right).sqr_magnitude()
    }

    #[inline]
    pub fn scale(a: Self, b: Self) -> Self
    where T: Mul<Output = T> {
        Self { x: a.x * b.x, y: a.y * b.y }
    }

    #[inline]
    pub fn normalize(vector: &Self) -> Self
    where T: Real + DivAssign {
        let mut x = vector.x;
        let mut y = vector.y;

        let length = (x * x + y * y).sqrt();

        x /= length;
        y /= length;

        Self { x, y }
    }

    #[inline]
    pub fn dot(left: Self, right: Self) -> T
    where T: Mul<Output = T> + Add<Output = T> {
        left.x * right.x + left.y * right.y
    }

    #[inline]
    pub fn reflect(direction: Self, normal: Self) -> Self
    where T: Real + Copy {
        let factor = Self::dot(direction, normal) * (T::one() + T::one());
        Self {x: normal.x * factor + direction.x, y: normal.y * factor + direction.y }
    }

    #[inline]
    pub fn perpendicular(vector: Self) -> Self
    where T: Neg<Output = T> {
        Self { x: -vector.y, y: vector.x }
    }

    #[inline]
    pub fn move_towards(current: Self, target: Self, max_distance_delta: T) -> Self
    where T:         
        DivAssign + MulAssign +
        Real + Copy {
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

impl<T> Vector2<T>
where T: Float {
    #[inline]
    pub fn positive_infinity() -> Self {
        Self { x: T::infinity(), y: T::infinity() }
    }

    #[inline]
    pub fn negative_infinity() -> Self {
        Self { x: T::neg_infinity(), y: T::neg_infinity() }
    }
}

impl<T> Index<usize> for Vector2<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds")
        }
    }
}

impl<T> IndexMut<usize> for Vector2<T> {
    #[inline]
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

    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl<T> Add<Vector2<T>> for Vector2<T>
where T: Add<Output = T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<'a, T> Add<&'a Vector2<T>> for Vector2<T>
where T: Add<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &'a Vector2<T>) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T> Sub<Vector2<T>> for Vector2<T>
where T: Sub<Output = T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<'a, T> Sub<&'a Vector2<T>> for Vector2<T>
where T: Sub<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &'a Vector2<T>) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T> Mul<T> for Vector2<T>
where T: Mul<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<'a, T> Mul<&'a T> for Vector2<T>
where T: Mul<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &'a T) -> Self::Output {
        Self { x: self.x * *rhs, y: self.y * *rhs }
    }
}

impl<T> Mul<Vector2<T>> for Vector2<T>
where T: Mul<Output = T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl<'a, T> Mul<&'a Vector2<T>> for Vector2<T>
where T: Mul<Output = T> + Deref<Target = T> + Copy {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &'a Vector2<T>) -> Self::Output {
        Self { x: self.x * *rhs.x, y: self.y * *rhs.y }
    }
}

impl<T> Div<T> for Vector2<T>
where T: Div<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<'a, T> Div<&'a T> for Vector2<T>
where T: Div<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn div(self, rhs: &'a T) -> Self::Output {
        Self { x: self.x / *rhs, y: self.y / *rhs }
    }
}

impl<T> Div<Vector2<T>> for Vector2<T>
where T: Div<Output = T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl<'a, T> Div<&'a Vector2<T>> for Vector2<T>
where T: Div<Output = T> + Deref<Target = T> + Copy {
    type Output = Self;

    #[inline]
    fn div(self, rhs: &'a Vector2<T>) -> Self::Output {
        Self { x: self.x / *rhs.x, y: self.y / *rhs.y }
    }
}

impl<T> AddAssign<Vector2<T>> for Vector2<T>
where T: AddAssign {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<'a, T> AddAssign<&'a Vector2<T>> for Vector2<T>
where T: AddAssign + Copy {
    #[inline]
    fn add_assign(&mut self, rhs: &'a Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign<Vector2<T>> for Vector2<T>
where T: SubAssign {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<'a, T> SubAssign<&'a Vector2<T>> for Vector2<T>
where T: SubAssign + Copy {
    #[inline]
    fn sub_assign(&mut self, rhs: &'a Vector2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign<T> for Vector2<T>
where T: MulAssign + Copy {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<'a, T> MulAssign<&'a T> for Vector2<T>
where T: MulAssign + Copy {
    #[inline]
    fn mul_assign(&mut self, rhs: &'a T) {
        self.x *= *rhs;
        self.y *= *rhs;
    }
}

impl<T> MulAssign<Vector2<T>> for Vector2<T>
where T: MulAssign {
    #[inline]
    fn mul_assign(&mut self, rhs: Vector2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<'a, T> MulAssign<&'a Vector2<T>> for Vector2<T>
where T: MulAssign + Deref<Target = T> + Copy {
    #[inline]
    fn mul_assign(&mut self, rhs: &'a Vector2<T>) {
        self.x *= *rhs.x;
        self.y *= *rhs.y;
    }
}

impl<T> DivAssign<T> for Vector2<T>
where T: DivAssign + Copy {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<'a, T> DivAssign<&'a T> for Vector2<T>
where T: DivAssign + Copy {
    #[inline]
    fn div_assign(&mut self, rhs: &'a T) {
        self.x /= *rhs;
        self.y /= *rhs;
    }
}

impl<T> DivAssign<Vector2<T>> for Vector2<T>
where T: DivAssign {
    #[inline]
    fn div_assign(&mut self, rhs: Vector2<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<'a, T> DivAssign<&'a Vector2<T>> for Vector2<T>
where T: DivAssign + Deref<Target = T> + Copy {
    #[inline]
    fn div_assign(&mut self, rhs: &'a Vector2<T>) {
        self.x /= *rhs.x;
        self.y /= *rhs.y;
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    #[inline]
    fn from(tuple: (T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1 }
    }
}

impl<T> From<[T; 2]> for Vector2<T>
where T: Copy {
    #[inline]
    fn from(array: [T; 2]) -> Self {
        Self { x: array[0], y: array[1] }
    }
}

impl<T> From<[[T; 1]; 2]> for Vector2<T>
where T: Copy {
    #[inline]
    fn from(array: [[T; 1]; 2]) -> Self {
        Self { x: array[0][0], y: array[0][1] }
    }
}

impl<T> From<Vector2<T>> for (T, T) {
    #[inline]
    fn from(vector: Vector2<T>) -> Self {
        (vector.x, vector.y)
    }
}

impl<T> From<Vector2<T>> for [T; 2]
where T: Copy {
    #[inline]
    fn from(vector: Vector2<T>) -> Self {
        [vector.x, vector.y]
    }
}

impl<T> From<Vector2<T>> for [[T; 1]; 2]
where T: Copy {
    #[inline]
    fn from(vector: Vector2<T>) -> Self {
        [[vector.x], [vector.y]]
    }
}

impl<T> From<Vector3<T>> for Vector2<T> {
    #[inline]
    fn from(vector: Vector3<T>) -> Self {
        Self { x: vector.x, y: vector.y }
    }
}

impl<T> From<Vector4<T>> for Vector2<T> {
    #[inline]
    fn from(vector: Vector4<T>) -> Self {
        Self { x: vector.x, y: vector.y }
    }
}

impl<T> IntoIterator for Vector2<T> {
    type Item = T;
    
    type IntoIter = std::array::IntoIter<Self::Item, 2>;
    
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y].into_iter()
    }
}

impl<T> FromIterator<T> for Vector2<T>
where T: Copy + Default {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        if let Some(x) = iter.next() {
            if let Some(y) = iter.next() {
                return Self { x, y };
            }

            return Self { x, y: T::default() };
        }
        
        Self { x: T::default(), y: T::default() }
    }
}




pub type Vector3f32 = Vector3<f32>;
pub type Vector3f64 = Vector3<f64>;
pub type Vector3i32 = Vector3<i32>;
pub type Vector3i64 = Vector3<i64>;
pub type Vector3u32 = Vector3<u32>;
pub type Vector3u64 = Vector3<u64>;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}


impl<T> Vector3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn zero() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::zero(), z: T::zero() }
    }

    #[inline]
    pub fn one() -> Self
    where T: Real {
        Self { x: T::one(), y: T::one(), z: T::one() }
    }

    #[inline]
    pub fn right() -> Self
    where T: Real {
        Self { x: T::one(), y: T::zero(), z: T::zero() }
    }

    #[inline]
    pub fn left() -> Self
    where T: Real {
        Self { x: -T::one(), y: T::zero(), z: T::zero() }
    }

    #[inline]
    pub fn up() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::one(), z: T::zero() }
    }

    #[inline]
    pub fn down() -> Self
    where T: Real {
        Self { x: T::zero(), y: -T::one(), z: T::zero() }
    }

    #[inline]
    pub fn forward() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::zero(), z: T::one() }
    }

    #[inline]
    pub fn back() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::zero(), z: -T::one() }
    }

    #[inline]
    pub fn set(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    #[inline]
    pub fn normalized(&self) -> Self
    where T: Real + DivAssign {
        Self::normalize(self)
    }

    #[inline]
    pub fn sqr_magnitude(&self) -> T
    where T: Mul<Output = T> + Add<Output = T> + Copy {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn magnitude(&self) -> T
    where T: Real {
        self.sqr_magnitude().sqrt()
    }

    #[inline]
    pub fn distance(left: Self, right: Self) -> T
    where T: Real {
        (left - right).magnitude()
    }

    #[inline]
    pub fn sqr_distance(left: Self, right: Self) -> T
    where T: Real {
        (left - right).sqr_magnitude()
    }

    #[inline]
    pub fn scale(a: Self, b: Self) -> Self
    where T: Mul<Output = T> {
        Self { x: a.x * b.x, y: a.y * b.y, z: a.z * b.z }
    }

    #[inline]
    pub fn normalize(vector: &Self) -> Self
    where T: DivAssign + Real {
        let mut x = vector.x;
        let mut y = vector.y;
        let mut z = vector.z;

        let length = (x * x + y * y + z * z).sqrt();

        x /= length;
        y /= length;
        z /= length;

        Self { x, y, z }
    }

    #[inline]
    pub fn dot(left: Self, right: Self) -> T
    where T: Mul<Output = T> + Add<Output = T> {
        left.x * right.x + left.y * right.y + left.z * right.z
    }

    #[inline]
    pub fn reflect(direction: Self, normal: Self) -> Self
    where T: Real + Copy {
        let factor = Self::dot(direction, normal) * (T::one() + T::one());
        Self {x: normal.x * factor + direction.x, y: normal.y * factor + direction.y, z: normal.z * factor + direction.z }
    }

    #[inline]
    pub fn cross(left: Self, right: Self) -> Self
    where T: Sub<Output = T> + Mul<Output = T> + Copy {
        Self {
            x: left.y * right.z - left.z * right.y,
            y: left.z * right.x - left.x * right.z,
            z: left.x * right.y - left.y * right.x
        }
    }

    #[inline]
    pub fn project(vector: Self, normal: Self) -> Self
    where T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Copy {
        let sqr_mag = normal.sqr_magnitude();

        let dot = Self::dot(vector, normal);
        Self { 
            x: normal.x * dot / sqr_mag,
            y: normal.y * dot / sqr_mag,
            z: normal.z * dot / sqr_mag
        }
    }

    #[inline]
    pub fn project_on_plane(vector: Self, plane_normal: Self) -> Self
    where T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Sub<Output = T> + Copy {
        vector - Self::project(vector, plane_normal)
    }

    #[inline]
    pub fn move_towards(current: Self, target: Self, max_distance_delta: T) -> Self
    where T: 
        DivAssign + MulAssign +
        Real + Copy {
        let mut movement = target - current;
        let sqr_magnitude = movement.sqr_magnitude();

        if sqr_magnitude > max_distance_delta * max_distance_delta {
            let magnitude = sqr_magnitude.sqrt();
            movement.x /= magnitude;
            movement.y /= magnitude;
            movement.z /= magnitude;

            movement.x *= max_distance_delta;
            movement.y *= max_distance_delta;
            movement.z *= max_distance_delta;

            current + movement
        } else {
            target
        }
    }
}

impl<T> Vector3<T>
where T: Float {
    #[inline]
    pub fn positive_infinity() -> Self {
        Self { x: T::infinity(), y: T::infinity(), z: T::infinity() }
    }

    #[inline]
    pub fn negative_infinity() -> Self {
        Self { x: T::neg_infinity(), y: T::neg_infinity(), z: T::neg_infinity() }
    }
}

impl<T> Index<usize> for Vector3<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds")
        }
    }
}

impl<T> IndexMut<usize> for Vector3<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds")
        }
    }
}

impl<T> Neg for Vector3<T>
where T: Neg<Output = T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl<T> Add<Vector3<T>> for Vector3<T>
where T: Add<Output = T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<'a, T> Add<&'a Vector3<T>> for Vector3<T>
where T: Add<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &'a Vector3<T>) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<T> Sub<Vector3<T>> for Vector3<T>
where T: Sub<Output = T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<'a, T> Sub<&'a Vector3<T>> for Vector3<T>
where T: Sub<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &'a Vector3<T>) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<T> Mul<T> for Vector3<T>
where T: Mul<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl<'a, T> Mul<&'a T> for Vector3<T>
where T: Mul<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &'a T) -> Self::Output {
        Self { x: self.x * *rhs, y: self.y * *rhs, z: self.z * *rhs }
    }
}

impl<T> Mul<Vector3<T>> for Vector3<T>
where T: Mul<Output = T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl<'a, T> Mul<&'a Vector3<T>> for Vector3<T>
where T: Mul<Output = T> + Deref<Target = T> + Copy {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &'a Vector3<T>) -> Self::Output {
        Self { x: self.x * *rhs.x, y: self.y * *rhs.y, z: self.z * *rhs.z }
    }
}

impl<T> Div<T> for Vector3<T>
where T: Div<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl<'a, T> Div<&'a T> for Vector3<T>
where T: Div<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn div(self, rhs: &'a T) -> Self::Output {
        Self { x: self.x / *rhs, y: self.y / *rhs, z: self.z / *rhs }
    }
}

impl<T> Div<Vector3<T>> for Vector3<T>
where T: Div<Output = T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl<'a, T> Div<&'a Vector3<T>> for Vector3<T>
where T: Div<Output = T> + Deref<Target = T> + Copy {
    type Output = Self;

    #[inline]
    fn div(self, rhs: &'a Vector3<T>) -> Self::Output {
        Self { x: self.x / *rhs.x, y: self.y / *rhs.y, z: self.z / *rhs.z }
    }
}

impl<T> AddAssign<Vector3<T>> for Vector3<T>
where T: AddAssign {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<'a, T> AddAssign<&'a Vector3<T>> for Vector3<T>
where T: AddAssign + Copy {
    #[inline]
    fn add_assign(&mut self, rhs: &'a Vector3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign<Vector3<T>> for Vector3<T>
where T: SubAssign {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<'a, T> SubAssign<&'a Vector3<T>> for Vector3<T>
where T: SubAssign + Copy {
    #[inline]
    fn sub_assign(&mut self, rhs: &'a Vector3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign<T> for Vector3<T>
where T: MulAssign + Copy {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<'a, T> MulAssign<&'a T> for Vector3<T>
where T: MulAssign + Copy {
    #[inline]
    fn mul_assign(&mut self, rhs: &'a T) {
        self.x *= *rhs;
        self.y *= *rhs;
    }
}

impl<T> MulAssign<Vector3<T>> for Vector3<T>
where T: MulAssign {
    #[inline]
    fn mul_assign(&mut self, rhs: Vector3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<'a, T> MulAssign<&'a Vector3<T>> for Vector3<T>
where T: MulAssign + Deref<Target = T> + Copy {
    #[inline]
    fn mul_assign(&mut self, rhs: &'a Vector3<T>) {
        self.x *= *rhs.x;
        self.y *= *rhs.y;
    }
}

impl<T> DivAssign<T> for Vector3<T>
where T: DivAssign + Copy {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<'a, T> DivAssign<&'a T> for Vector3<T>
where T: DivAssign + Copy {
    #[inline]
    fn div_assign(&mut self, rhs: &'a T) {
        self.x /= *rhs;
        self.y /= *rhs;
    }
}

impl<T> DivAssign<Vector3<T>> for Vector3<T>
where T: DivAssign {
    #[inline]
    fn div_assign(&mut self, rhs: Vector3<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<'a, T> DivAssign<&'a Vector3<T>> for Vector3<T>
where T: DivAssign + Deref<Target = T> + Copy {
    #[inline]
    fn div_assign(&mut self, rhs: &'a Vector3<T>) {
        self.x /= *rhs.x;
        self.y /= *rhs.y;
    }
}

impl<T> From<(T, T, T)> for Vector3<T> {
    #[inline]
    fn from(tuple: (T, T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1, z: tuple.2 }
    }
}

impl<T> From<[T; 3]> for Vector3<T>
where T: Copy {
    #[inline]
    fn from(array: [T; 3]) -> Self {
        Self { x: array[0], y: array[1], z: array[2] }
    }
}

impl<T> From<[[T; 1]; 3]> for Vector3<T>
where T: Copy {
    #[inline]
    fn from(array: [[T; 1]; 3]) -> Self {
        Self { x: array[0][0], y: array[0][1], z: array[0][2] }
    }
}

impl<T> From<Vector3<T>> for (T, T, T) {
    fn from(vector: Vector3<T>) -> Self {
        (vector.x, vector.y, vector.z)
    }
}

impl<T> From<Vector3<T>> for [T; 3]
where T: Copy {
    #[inline]
    fn from(vector: Vector3<T>) -> Self {
        [vector.x, vector.y, vector.z]
    }
}

impl<T> From<Vector3<T>> for [[T; 1]; 3]
where T: Copy {
    #[inline]
    fn from(vector: Vector3<T>) -> Self {
        [[vector.x], [vector.y], [vector.z]]
    }
}

impl<T> From<Vector2<T>> for Vector3<T>
where T: Default {
    #[inline]
    fn from(vector: Vector2<T>) -> Self {
        Self { x: vector.x, y: vector.y, z: T::default() }
    }
}

impl<T> From<Vector4<T>> for Vector3<T> {
    #[inline]
    fn from(vector: Vector4<T>) -> Self {
        Self { x: vector.x, y: vector.y, z: vector.z }
    }
}

impl<T> IntoIterator for Vector3<T> {
    type Item = T;

    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y].into_iter()
    }
}

impl<T> FromIterator<T> for Vector3<T>
where T: Copy + Default {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        if let Some(x) = iter.next() {
            if let Some(y) = iter.next() {
                if let Some(z) = iter.next() {
                    return Self { x, y, z };
                }

                return Self { x, y, z: T::default() };
            }

            return Self { x, y: T::default(), z: T::default() };
        }

        Self { x: T::default(), y: T::default(), z: T::default() }
    }
}




pub type Vector4f32 = Vector4<f32>;
pub type Vector4f64 = Vector4<f64>;
pub type Vector4i32 = Vector4<i32>;
pub type Vector4i64 = Vector4<i64>;
pub type Vector4u32 = Vector4<u32>;
pub type Vector4u64 = Vector4<u64>;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> Vector4<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn zero() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero() }
    }

    #[inline]
    pub fn one() -> Self
    where T: Real {
        Self { x: T::one(), y: T::one(), z: T::one(), w: T::one() }
    }

    #[inline]
    pub fn right() -> Self
    where T: Real {
        Self { x: T::one(), y: T::zero(), z: T::zero(), w: T::zero() }
    }

    #[inline]
    pub fn left() -> Self
    where T: Real {
        Self { x: -T::one(), y: T::zero(), z: T::zero(), w: T::zero() }
    }

    #[inline]
    pub fn up() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::one(), z: T::zero(), w: T::zero() }
    }

    #[inline]
    pub fn down() -> Self
    where T: Real {
        Self { x: T::zero(), y: -T::one(), z: T::zero(), w: T::zero() }
    }

    #[inline]
    pub fn forward() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::zero(), z: T::one(), w: T::zero() }
    }

    #[inline]
    pub fn back() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::zero(), z: -T::one(), w: T::zero() }
    }

    #[inline]
    pub fn hyper_forward() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::zero(), z: T::zero(), w: T::one() }
    }

    #[inline]
    pub fn hyper_back() -> Self
    where T: Real {
        Self { x: T::zero(), y: T::zero(), z: T::zero(), w: -T::one() }
    }

    #[inline]
    pub fn set(&mut self, x: T, y: T, z: T, w: T) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    #[inline]
    pub fn normalized(&self) -> Self
    where T: Real + DivAssign {
        Self::normalize(self)
    }

    #[inline]
    pub fn sqr_magnitude(&self) -> T
    where T: Mul<Output = T> + Add<Output = T> + Copy {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    #[inline]
    pub fn magnitude(&self) -> T
    where T: Real {
        self.sqr_magnitude().sqrt()
    }

    #[inline]
    pub fn distance(left: Self, right: Self) -> T
    where T: Real {
        (left - right).magnitude()
    }

    #[inline]
    pub fn sqr_distance(left: Self, right: Self) -> T
    where T: Real {
        (left - right).sqr_magnitude()
    }

    #[inline]
    pub fn scale(a: Self, b: Self) -> Self
    where T: Mul<Output = T> {
        Self { x: a.x * b.x, y: a.y * b.y, z: a.z * b.z, w: a.w * b.w }
    }

    #[inline]
    pub fn normalize(vector: &Self) -> Self
    where T: Real + DivAssign {
        let mut x = vector.x;
        let mut y = vector.y;
        let mut z = vector.z;
        let mut w = vector.w;

        let length = (x * x + y * y + z * z + w * w).sqrt();

        x /= length;
        y /= length;
        z /= length;
        w /= length;

        Self { x, y, z, w }
    }

    #[inline]
    pub fn dot(left: Self, right: Self) -> T
    where T: Mul<Output = T> + Add<Output = T> {
        left.x * right.x + left.y * right.y + left.z * right.z + left.w * right.w
    }

    #[inline]
    pub fn reflect(direction: Self, normal: Self) -> Self
    where T: Real + Copy {
        let factor = Self::dot(direction, normal) * (T::one() + T::one());
        Self {x: normal.x * factor + direction.x, y: normal.y * factor + direction.y, z: normal.z * factor + direction.z, w: normal.w * factor + direction.w }
    }

    #[inline]
    pub fn project(vector: Self, normal: Self) -> Self
    where T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Copy {
        normal * Self::dot(vector, normal) / normal.sqr_magnitude()
    }

    #[inline]
    pub fn move_towards(current: Self, target: Self, max_distance_delta: T) -> Self
    where T: 
        DivAssign + MulAssign +
        Real + Copy {
        let mut movement = target - current;
        let sqr_magnitude = movement.sqr_magnitude();

        if sqr_magnitude > max_distance_delta * max_distance_delta {
            let magnitude = sqr_magnitude.sqrt();
            movement.x /= magnitude;
            movement.y /= magnitude;
            movement.z /= magnitude;

            movement.x *= max_distance_delta;
            movement.y *= max_distance_delta;
            movement.z *= max_distance_delta;

            current + movement
        } else {
            target
        }
    }
}

impl<T> Vector4<T>
where T: Float {
    #[inline]
    pub fn positive_infinity() -> Self {
        Self { x: T::infinity(), y: T::infinity(), z: T::infinity(), w: T::infinity() }
    }

    #[inline]
    pub fn negative_infinity() -> Self {
        Self { x: T::neg_infinity(), y: T::neg_infinity(), z: T::neg_infinity(), w: T::neg_infinity() }
    }
}

impl<T> Index<usize> for Vector4<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds")
        }
    }
}

impl<T> IndexMut<usize> for Vector4<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds")
        }
    }
}

impl<T> Neg for Vector4<T>
where T: Neg<Output = T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}

impl<T> Add<Vector4<T>> for Vector4<T>
where T: Add<Output = T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl<'a, T> Add<&'a Vector4<T>> for Vector4<T>
where T: Add<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &'a Vector4<T>) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl<T> Sub<Vector4<T>> for Vector4<T>
where T: Sub<Output = T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl<'a, T> Sub<&'a Vector4<T>> for Vector4<T>
where T: Sub<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &'a Vector4<T>) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl<T> Mul<T> for Vector4<T>
where T: Mul<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}

impl<'a, T> Mul<&'a T> for Vector4<T>
where T: Mul<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &'a T) -> Self::Output {
        Self { x: self.x * *rhs, y: self.y * *rhs, z: self.z * *rhs, w: self.w * *rhs }
    }
}

impl<T> Mul<Vector4<T>> for Vector4<T>
where T: Mul<Output = T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w }
    }
}

impl<'a, T> Mul<&'a Vector4<T>> for Vector4<T>
where T: Mul<Output = T> + Deref<Target = T> + Copy {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &'a Vector4<T>) -> Self::Output {
        Self { x: self.x * *rhs.x, y: self.y * *rhs.y, z: self.z * *rhs.z, w: self.w * *rhs.w }
    }
}

impl<T> Div<T> for Vector4<T>
where T: Div<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}

impl<'a, T> Div<&'a T> for Vector4<T>
where T: Div<Output = T> + Copy {
    type Output = Self;

    #[inline]
    fn div(self, rhs: &'a T) -> Self::Output {
        Self { x: self.x / *rhs, y: self.y / *rhs, z: self.z / *rhs, w: self.w / *rhs }
    }
}

impl<T> Div<Vector4<T>> for Vector4<T>
where T: Div<Output = T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w }
    }
}

impl<'a, T> Div<&'a Vector4<T>> for Vector4<T>
where T: Div<Output = T> + Deref<Target = T> + Copy {
    type Output = Self;

    #[inline]
    fn div(self, rhs: &'a Vector4<T>) -> Self::Output {
        Self { x: self.x / *rhs.x, y: self.y / *rhs.y, z: self.z / *rhs.z, w: self.w / *rhs.w }
    }
}

impl<T> AddAssign<Vector4<T>> for Vector4<T>
where T: AddAssign {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<'a, T> AddAssign<&'a Vector4<T>> for Vector4<T>
where T: AddAssign + Copy {
    #[inline]
    fn add_assign(&mut self, rhs: &'a Vector4<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign<Vector4<T>> for Vector4<T>
where T: SubAssign {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<'a, T> SubAssign<&'a Vector4<T>> for Vector4<T>
where T: SubAssign + Copy {
    #[inline]
    fn sub_assign(&mut self, rhs: &'a Vector4<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign<T> for Vector4<T>
where T: MulAssign + Copy {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<'a, T> MulAssign<&'a T> for Vector4<T>
where T: MulAssign + Copy {
    #[inline]
    fn mul_assign(&mut self, rhs: &'a T) {
        self.x *= *rhs;
        self.y *= *rhs;
    }
}

impl<T> MulAssign<Vector4<T>> for Vector4<T>
where T: MulAssign {
    #[inline]
    fn mul_assign(&mut self, rhs: Vector4<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<'a, T> MulAssign<&'a Vector4<T>> for Vector4<T>
where T: MulAssign + Deref<Target = T> + Copy {
    #[inline]
    fn mul_assign(&mut self, rhs: &'a Vector4<T>) {
        self.x *= *rhs.x;
        self.y *= *rhs.y;
    }
}

impl<T> DivAssign<T> for Vector4<T>
where T: DivAssign + Copy {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<'a, T> DivAssign<&'a T> for Vector4<T>
where T: DivAssign + Copy {
    #[inline]
    fn div_assign(&mut self, rhs: &'a T) {
        self.x /= *rhs;
        self.y /= *rhs;
    }
}

impl<T> DivAssign<Vector4<T>> for Vector4<T>
where T: DivAssign {
    #[inline]
    fn div_assign(&mut self, rhs: Vector4<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<'a, T> DivAssign<&'a Vector4<T>> for Vector4<T>
where T: DivAssign + Deref<Target = T> + Copy {
    #[inline]
    fn div_assign(&mut self, rhs: &'a Vector4<T>) {
        self.x /= *rhs.x;
        self.y /= *rhs.y;
    }
}

impl<T> From<(T, T, T, T)> for Vector4<T> {
    #[inline]
    fn from(tuple: (T, T, T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1, z: tuple.2, w: tuple.3 }
    }
}

impl<T> From<[T; 4]> for Vector4<T>
where T: Copy {
    #[inline]
    fn from(array: [T; 4]) -> Self {
        Self { x: array[0], y: array[1], z: array[2], w: array[3] }
    }
}

impl<T> From<[[T; 1]; 4]> for Vector4<T>
where T: Copy {
    #[inline]
    fn from(array: [[T; 1]; 4]) -> Self {
        Self { x: array[0][0], y: array[0][1], z: array[0][2], w: array[0][3] }
    }
}

impl<T> From<Vector4<T>> for (T, T, T, T) {
    #[inline]
    fn from(vector: Vector4<T>) -> Self {
        (vector.x, vector.y, vector.z, vector.w)
    }
}

impl<T> From<Vector4<T>> for [T; 4]
where T: Copy {
    #[inline]
    fn from(vector: Vector4<T>) -> Self {
        [vector.x, vector.y, vector.z, vector.w]
    }
}

impl<T> From<Vector4<T>> for [[T; 1]; 4]
where T: Copy {
    #[inline]
    fn from(vector: Vector4<T>) -> Self {
        [[vector.x], [vector.y], [vector.z], [vector.w]]
    }
}

impl<T> From<Vector2<T>> for Vector4<T>
where T: Default {
    #[inline]
    fn from(vector: Vector2<T>) -> Self {
        Self { x: vector.x, y: vector.y, z: T::default(), w: T::default() }
    }
}

impl<T> From<Vector3<T>> for Vector4<T>
where T: Default {
    #[inline]
    fn from(vector: Vector3<T>) -> Self {
        Self { x: vector.x, y: vector.y, z: vector.z, w: T::default() }
    }
}

impl<T> IntoIterator for Vector4<T> {
    type Item = T;

    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y].into_iter()
    }
}

impl<T> FromIterator<T> for Vector4<T>
where T: Copy + Default {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        if let Some(x) = iter.next() {
            if let Some(y) = iter.next() {
                if let Some(z) = iter.next() {
                    if let Some(w) = iter.next() {
                        return Self { x, y, z, w };
                    }

                    return Self { x, y, z, w: T::default() };
                }

                return Self { x, y, z: T::default(), w: T::default() };
            }

            return Self { x, y: T::default(), z: T::default(), w: T::default() };
        }

        Self { x: T::default(), y: T::default(), z: T::default(), w: T::default() }
    }
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
