use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, DivAssign};

use num_traits::real::Real;

use crate::vectors::{Vector, Vector2, Vector3, Vector4};

use self::traits::Pi;

mod traits;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T> {
    #[inline]
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Rect { x, y, width, height }
    }

    #[inline]
    pub fn new_vectors(position: Vector2<T>, size: Vector2<T>) -> Self {
        Self::new(position.x, position.y, size.x, size.y)
    }

    #[inline]
    pub fn set(&mut self, x: T, y: T, width: T, height: T) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    #[inline]
    pub fn set_vectors(&mut self, position: Vector2<T>, size: Vector2<T>) {
        self.x = position.x;
        self.y = position.y;
        self.width = size.x;
        self.height = size.y;
    }

    #[inline]
    pub fn get_position(&self) -> Vector2<T>
    where T: Copy {
        Vector2::new_comp(self.x, self.y)
    }

    #[inline]
    pub fn set_position(&mut self, position: Vector2<T>)
    where T: Copy {
        self.x = position.x;
        self.y = position.y;
    }

    #[inline]
    pub fn get_size(&self) -> Vector2<T>
    where T: Copy {
        Vector2::new_comp(self.width, self.height)
    }

    #[inline]
    pub fn set_size(&mut self, size: Vector2<T>)
    where T: Copy {
        self.width = size.x;
        self.height = size.y;
    }

    #[inline]
    pub fn get_center(&self) -> Vector2<T>
    where T: Real {
        let two = T::one() + T::one();
        Vector2::new_comp(self.x + self.width / two, self.y + self.height / two)
    }

    #[inline]
    pub fn set_center(&mut self, center: Vector2<T>)
    where T: Real {
        let two = T::one() + T::one();
        self.x = center.x - self.width / two ;
        self.y = center.y - self.height / two ;
    }

    #[inline]
    pub fn get_x_min(&self) -> T
    where T: Copy {
        self.x
    }

    #[inline]
    pub fn set_x_min(&mut self, x_min: T)
    where T: Copy {
        self.x = x_min;
    }

    #[inline]
    pub fn get_x_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.x + self.width
    }

    #[inline]
    pub fn set_x_max(&mut self, x_max: T)
    where T: Sub<Output = T> + Copy {
        self.width = x_max - self.x;
    }

    #[inline]
    pub fn get_y_min(&self) -> T
    where T: Copy {
        self.y
    }

    #[inline]
    pub fn set_y_min(&mut self, y_min: T)
    where T: Copy {
        self.y = y_min;
    }

    #[inline]
    pub fn get_y_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.y + self.height
    }

    #[inline]
    pub fn set_y_max(&mut self, y_max: T)
    where T: Sub<Output = T> + Copy {
        self.height = y_max - self.y;
    }

    #[inline]
    pub fn contains(&self, point: Vector2<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Copy {
        point.x >= self.x &&
        point.x <= self.x + self.width &&
        point.y >= self.y &&
        point.y <= self.y + self.height
    }

    #[inline]
    pub fn overlaps(&self, other: &Rect<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Copy {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    #[inline]
    pub fn overlaps_area(&self, area: Area2D<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy {
        let other = Rect::from(area);
        self.overlaps(&other)
    }

    #[inline]
    pub fn overlaps_bounds(&self, bounds: Bounds2D<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy {
        let other = Rect::from(bounds);
        self.overlaps(&other)
    }

    #[inline]
    pub fn overlaps_circle(&self, circle: Circle<T>) -> bool
    where T: Real {
        let xn: T = self.x.max(circle.center.x.min(self.get_x_max()));
        let yn: T = self.y.max(circle.center.y.min(self.get_y_max()));

        let dx = xn - circle.center.x;
        let dy = yn - circle.center.y;
        (dx * dx + dy * dy) <= circle.radius * circle.radius
    }
}

impl<T> From<Area2D<T>> for Rect<T>
where T: Sub<Output = T> + Copy {
    #[inline]
    fn from(area: Area2D<T>) -> Self {
        Rect::new_vectors(area.lower_left, area.upper_right - area.lower_left)
    }
}

impl<T> From<Bounds2D<T>> for Rect<T>
where T: Add<Output = T> + Sub<Output = T> + Copy {
    #[inline]
    fn from(bounds: Bounds2D<T>) -> Self {
        let position = bounds.center - bounds.extents;
        let size = bounds.get_size();
        
        Rect::new_vectors(position, size)
    }
}

impl<T> From<Area3D<T>> for Rect<T> 
where T: Sub<Output = T> + Copy {
    #[inline]
    fn from(area: Area3D<T>) -> Self {
        Rect::new(area.lower_left.x, area.lower_left.y, area.upper_right.x - area.lower_left.x, area.upper_right.y - area.lower_left.y)
    }
}

impl<T> From<Bounds3D<T>> for Rect<T>
where T: Add<Output = T> + Sub<Output = T> + Copy {
    #[inline]
    fn from(bounds: Bounds3D<T>) -> Self {
        let position = Vector2::from(bounds.center - bounds.extents);
        let size = Vector2::from(bounds.get_size());
        
        Rect::new_vectors(position, size)
    }
}

impl<T> From<Area4D<T>> for Rect<T> 
where T: Sub<Output = T> + Copy {
    #[inline]
    fn from(area: Area4D<T>) -> Self {
        Rect::new(area.lower_left.x, area.lower_left.y, area.upper_right.x - area.lower_left.x, area.upper_right.y - area.lower_left.y)
    }
}

impl<T> From<Bounds4D<T>> for Rect<T>
where T: Add<Output = T> + Sub<Output = T> + Copy {
    #[inline]
    fn from(bounds: Bounds4D<T>) -> Self {
        let position = Vector2::from(bounds.center - bounds.extents);
        let size = Vector2::from(bounds.get_size());
        
        Rect::new_vectors(position, size)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Area2D<T> {
    pub lower_left: Vector2<T>,
    pub upper_right: Vector2<T>,
}

impl<T> Area2D<T> {
    #[inline]
    pub fn new(lower_left_x: T, lower_left_y: T, upper_right_x: T, upper_right_y: T) -> Self {
        Self::new_vectors(Vector2::new_comp(lower_left_x, lower_left_y), Vector2::new_comp(upper_right_x, upper_right_y))
    }
    
    #[inline]
    pub fn new_vectors(lower_left: Vector2<T>, upper_right: Vector2<T>) -> Self {
        Area2D { lower_left, upper_right }
    }

    #[inline]
    pub fn set(&mut self, lower_left_x: T, lower_left_y: T, upper_right_x: T, upper_right_y: T) {
        self.lower_left.x = lower_left_x;
        self.lower_left.y = lower_left_y;
        self.upper_right.x = upper_right_x;
        self.upper_right.y = upper_right_y;
    }

    #[inline]
    pub fn set_vectors(&mut self, lower_left: Vector2<T>, upper_right: Vector2<T>) {
        self.lower_left = lower_left;
        self.upper_right = upper_right;
    }

    #[inline]
    pub fn get_x_min(&self) -> T
    where T: Copy {
        self.lower_left.x
    }

    #[inline]
    pub fn set_x_min(&mut self, x_min: T)
    where T: Copy {
        self.lower_left.x = x_min;
    }

    #[inline]
    pub fn get_x_max(&self) -> T
    where T: Copy {
        self.upper_right.x
    }

    #[inline]
    pub fn set_x_max(&mut self, x_max: T)
    where T: Copy {
        self.upper_right.x = x_max;
    }

    #[inline]
    pub fn get_y_min(&self) -> T
    where T: Copy {
        self.lower_left.y
    }

    #[inline]
    pub fn set_y_min(&mut self, y_min: T)
    where T: Copy {
        self.lower_left.y = y_min;
    }

    #[inline]
    pub fn get_y_max(&self) -> T
    where T: Copy {
        self.upper_right.y
    }

    #[inline]
    pub fn set_y_max(&mut self, y_max: T)
    where T: Copy {
        self.upper_right.y = y_max;
    }

    #[inline]
    pub fn get_width(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.upper_right.x - self.lower_left.x
    }

    #[inline]
    pub fn set_width(&mut self, width: T)
    where T: AddAssign + SubAssign + Real {
        let current_width = self.upper_right.x - self.lower_left.x;
        let delta = current_width - width;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left.x += half_delta;
        self.upper_right.x -= half_delta;
    }

    #[inline]
    pub fn get_height(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.upper_right.y - self.lower_left.y
    }

    #[inline]
    pub fn set_height(&mut self, height: T)
    where T: AddAssign + SubAssign + Real {
        let current_height = self.upper_right.y - self.lower_left.y;
        let delta = current_height - height;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left.y += half_delta;
        self.upper_right.y -= half_delta;
    }
    
    #[inline]
    pub fn get_size(&self) -> Vector2<T>
    where T: Sub<Output = T> + Copy {
        Vector2::new_comp(self.get_width(), self.get_height())
    }

    #[inline]
    pub fn set_size(&mut self, size: Vector2<T>)
    where T: AddAssign + SubAssign + Real {
        let current_size = self.get_size();
        let delta = current_size - size;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left += half_delta;
        self.upper_right -= half_delta;
    }

    #[inline]
    pub fn get_center(&self) -> Vector2<T>
    where T: Real {
        let half_size = self.get_size() / (T::one() + T::one());
        self.lower_left + half_size
    }

    #[inline]
    pub fn set_center(&mut self, center: Vector2<T>)
    where T: AddAssign + Real {
        let current_center = self.get_center();
        let delta = center - current_center;
        self.lower_left += delta;
        self.upper_right += delta;
    }

    #[inline]
    pub fn contains(&self, point: Vector2<T>) -> bool
    where T: PartialOrd + Copy {
        point.x >= self.lower_left.x &&
        point.x <= self.upper_right.x &&
        point.y >= self.lower_left.y &&
        point.y <= self.upper_right.y
    }

    #[inline]
    pub fn overlaps(&self, other: &Area2D<T>) -> bool
    where T: PartialOrd + Copy {
        self.lower_left.x < other.upper_right.x &&
        self.upper_right.x > other.lower_left.x &&
        self.lower_left.y < other.upper_right.y &&
        self.upper_right.y > other.lower_left.y
    }

    #[inline]
    pub fn overlaps_rect(&self, other: &Rect<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Copy {
        self.lower_left.x < other.x + other.width &&
        self.upper_right.x > other.x &&
        self.lower_left.y < other.y + other.height &&
        self.upper_right.y > other.y
    }

    #[inline]
    pub fn overlaps_bounds(&self, bounds: &Bounds2D<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy {
        self.lower_left.x < bounds.center.x + bounds.extents.x &&
        self.upper_right.x > bounds.center.x - bounds.extents.x &&
        self.lower_left.y < bounds.center.y + bounds.extents.y &&
        self.upper_right.y > bounds.center.y - bounds.extents.y
    }

    #[inline]
    pub fn overlaps_circle(&self, circle: &Circle<T>) -> bool
    where T: Real {
        let rect = Rect::from(*self);
        rect.overlaps_circle(*circle)
    }
}

impl<T> From<Rect<T>> for Area2D<T>
where T: Add<Output = T> + Copy {
    #[inline]
    fn from(rect: Rect<T>) -> Self {
        Self::new(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height)
    }
}

impl<T> From<Bounds2D<T>> for Area2D<T>
where T: Add<Output = T> + Sub<Output = T> + Copy {
    #[inline]
    fn from(bounds: Bounds2D<T>) -> Self {
        Self::new(bounds.center.x - bounds.extents.x, bounds.center.y - bounds.extents.y, bounds.center.x + bounds.extents.x, bounds.center.y + bounds.extents.y)
    }
}

impl<T> From<Area3D<T>> for Area2D<T> {
    #[inline]
    fn from(area: Area3D<T>) -> Self {
        Self::new(area.lower_left.x, area.lower_left.y, area.upper_right.x, area.upper_right.y)
    }
}

impl<T> From<Bounds3D<T>> for Area2D<T>
where T: Add<Output = T> + Sub<Output = T> + Copy {
    #[inline]
    fn from(bounds: Bounds3D<T>) -> Self {
        Self::new(bounds.center.x - bounds.extents.x, bounds.center.y - bounds.extents.y, bounds.center.x + bounds.extents.x, bounds.center.y + bounds.extents.y)
    }
}

impl<T> From<Area4D<T>> for Area2D<T> {
    #[inline]
    fn from(area: Area4D<T>) -> Self {
        Self::new(area.lower_left.x, area.lower_left.y, area.upper_right.x, area.upper_right.y)
    }
}

impl<T> From<Bounds4D<T>> for Area2D<T>
where T: Add<Output = T> + Sub<Output = T> + Copy {
    #[inline]
    fn from(bounds: Bounds4D<T>) -> Self {
        Self::new(bounds.center.x - bounds.extents.x, bounds.center.y - bounds.extents.y, bounds.center.x + bounds.extents.x, bounds.center.y + bounds.extents.y)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Bounds2D<T> {
    pub center: Vector2<T>,
    pub extents: Vector2<T>,
}

impl<T> Bounds2D<T> {
    #[inline]
    pub fn new(center_x: T, center_y: T, extents_x: T, extents_y: T) -> Self
    where T: Copy {
        Self::new_vectors(Vector2::new_comp(center_x, center_y), Vector2::new_comp(extents_x, extents_y))
    }
    
    #[inline]
    pub fn new_vectors(center: Vector2<T>, extents: Vector2<T>) -> Self {
        Bounds2D { center, extents }
    }

    #[inline]
    pub fn set(&mut self, center_x: T, center_y: T, extents_x: T, extents_y: T)
    where T: Copy {
        self.center.x = center_x;
        self.center.y = center_y;
        self.extents.x = extents_x;
        self.extents.y = extents_y;
    }

    #[inline]
    pub fn set_vectors(&mut self, center: Vector2<T>, extents: Vector2<T>) {
        self.center = center;
        self.extents = extents;
    }

    #[inline]
    pub fn get_x_min(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.center.x - self.extents.x
    }

    #[inline]
    pub fn set_x_min(&mut self, x_min: T)
    where T: Add<Output = T> + Copy {
        self.center.x = x_min + self.extents.x;
    }

    #[inline]
    pub fn get_x_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.center.x + self.extents.x
    }

    #[inline]
    pub fn set_x_max(&mut self, x_max: T)
    where T: Sub<Output = T> + Copy {
        self.center.x = x_max - self.extents.x;
    }

    #[inline]
    pub fn get_y_min(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.center.y - self.extents.y
    }

    #[inline]
    pub fn set_y_min(&mut self, y_min: T)
    where T: Add<Output = T> + Copy {
        self.center.y = y_min + self.extents.y;
    }

    #[inline]
    pub fn get_y_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.center.y + self.extents.y
    }

    #[inline]
    pub fn set_y_max(&mut self, y_max: T)
    where T: Sub<Output = T> + Copy {
        self.center.y = y_max - self.extents.y;
    }

    #[inline]
    pub fn get_width(&self) -> T
    where T: Add<Output = T> + Copy {
        self.extents.x + self.extents.x
    }

    #[inline]
    pub fn set_width(&mut self, width: T)
    where T: SubAssign + Real {
        let current_width = self.extents.x + self.extents.x;
        let delta = current_width - width;
        let half_delta = delta / (T::one() + T::one());
        self.extents.x -= half_delta;
    }

    #[inline]
    pub fn get_height(&self) -> T
    where T: Add<Output = T> + Copy {
        self.extents.y + self.extents.y
    }

    #[inline]
    pub fn set_height(&mut self, height: T)
    where T: SubAssign + Real {
        let current_height = self.extents.y + self.extents.y;
        let delta = current_height - height;
        let half_delta = delta / (T::one() + T::one());
        self.extents.y -= half_delta;
    }

    #[inline]
    pub fn get_size(&self) -> Vector2<T>
    where T: Add<Output = T> + Copy {
        Vector2::new_comp(self.extents.x + self.extents.x, self.extents.y + self.extents.y)
    }

    #[inline]
    pub fn set_size(&mut self, size_x: T, size_y: T)
    where T: SubAssign + Copy + Real {
        let current_size = Vector2::new_comp(self.extents.x + self.extents.x, self.extents.y + self.extents.y);
        let delta = current_size - Vector2::new_comp(size_x, size_y);
        let half_delta = delta / (T::one() + T::one());
        self.extents -= half_delta;
    }

    #[inline]
    pub fn contains(&self, point: Vector2<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < point.x &&
        self.center.x + self.extents.x > point.x &&
        self.center.y - self.extents.y < point.y &&
        self.center.y + self.extents.y > point.y
    }

    #[inline]
    pub fn overlaps(&self, other: &Bounds2D<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < other.center.x + other.extents.x &&
        self.center.x + self.extents.x > other.center.x - other.extents.x &&
        self.center.y - self.extents.y < other.center.y + other.extents.y &&
        self.center.y + self.extents.y > other.center.y - other.extents.y
    }

    #[inline]
    pub fn overlaps_rect(&self, rect: &Rect<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < rect.get_x_max() &&
        self.center.x + self.extents.x > rect.get_x_min() &&
        self.center.y - self.extents.y < rect.get_y_max() &&
        self.center.y + self.extents.y > rect.get_y_min()
    }

    #[inline]
    pub fn overlaps_area(&self, area: &Area2D<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < area.get_x_max() &&
        self.center.x + self.extents.x > area.get_x_min() &&
        self.center.y - self.extents.y < area.get_y_max() &&
        self.center.y + self.extents.y > area.get_y_min()
    }

    pub fn overlaps_circle(&self, circle: Circle<T>)
    where T: Real {
        let rect = Rect::from(*self);
        rect.overlaps_circle(circle);
    }
}

impl<T> From<Rect<T>> for Bounds2D<T>
where T: Real {
    #[inline]
    fn from(rect: Rect<T>) -> Self {
        Self::new(
            rect.get_center().x,
            rect.get_center().y,
            rect.width / (T::one() + T::one()),
            rect.height / (T::one() + T::one()))
    }
}

impl<T> From<Area2D<T>> for Bounds2D<T>
where T: Real {
    #[inline]
    fn from(area: Area2D<T>) -> Self {
        Self::new(
            area.get_center().x,
            area.get_center().y,
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()))
    }
}

impl<T> From<Bounds3D<T>> for Bounds2D<T>
where T: Copy {
    #[inline]
    fn from(bounds: Bounds3D<T>) -> Self {
        Self::new(
            bounds.center.x,
            bounds.center.y,
            bounds.extents.x,
            bounds.extents.y)
    }
}

impl<T> From<Area3D<T>> for Bounds2D<T>
where T: Real {
    #[inline]
    fn from(area: Area3D<T>) -> Self {
        Self::new(
            area.get_center().x,
            area.get_center().y,
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()))
    }
}

impl<T> From<Bounds4D<T>> for Bounds2D<T>
where T: Copy {
    #[inline]
    fn from(bounds: Bounds4D<T>) -> Self {
        Self::new(
            bounds.center.x,
            bounds.center.y,
            bounds.extents.x,
            bounds.extents.y)
    }
}

impl<T> From<Area4D<T>> for Bounds2D<T>
where T: Real {
    #[inline]
    fn from(area: Area4D<T>) -> Self {
        Self::new(
            area.get_center().x,
            area.get_center().y,
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()))
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Circle<T> {
    pub center: Vector2<T>,
    pub radius: T,
}

impl<T> Circle<T> {
    #[inline]
    pub fn new(center_x: T, center_y: T, radius: T) -> Circle<T> {
        Self::new_vector(Vector2::new_comp(center_x, center_y), radius)
    }
    
    #[inline]
    pub fn new_vector(center: Vector2<T>, radius: T) -> Circle<T> {
        Circle { center, radius, }
    }

    #[inline]
    pub fn get_diameter(&self) -> T
    where T: Add<Output = T> + Copy {
        self.radius + self.radius
    }

    #[inline]
    pub fn set_diameter(&mut self, diameter: T)
    where T: Real {
        self.radius = diameter / (T::one() + T::one());
    }

    #[inline]
    pub fn get_circumference(&self) -> T
    where T: Real + Pi<Output = T> {
        self.radius * (T::one() + T::one()) * T::pi()
    }

    #[inline]
    pub fn set_circumference(&mut self, circumference: T)
    where T: Real + Pi<Output = T> {
        self.radius = circumference / ((T::one() + T::one()) * T::pi());
    }

    #[inline]
    pub fn get_area(&self) -> T
    where T: Mul<Output = T> + Pi<Output = T> + Copy {
        self.radius * self.radius * T::pi()
    }

    #[inline]
    pub fn set_area(&mut self, area: T)
    where T: Real + Pi<Output = T> {
        self.radius = (area / T::pi()).sqrt();
    }

    #[inline]
    pub fn contains(&self, point: Vector2<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd + Copy {
        let delta = point - self.center;
        let distance_squared = delta.sqr_magnitude();
        distance_squared <= self.radius * self.radius
    }

    #[inline]
    pub fn overlaps(&self, other: &Circle<T>) -> bool
    where T: Real {
        let delta = other.center - self.center;
        let distance_squared = delta.sqr_magnitude();
        let radius_sum = self.radius + other.radius;
        distance_squared < radius_sum * radius_sum
    }
}

impl<T> From<Sphere<T>> for Circle<T> {
    #[inline]
    fn from(sphere: Sphere<T>) -> Self {
        Self::new_vector(Vector2::from(sphere.center), sphere.radius)
    }
}

impl<T> From<HyperSphere<T>> for Circle<T> {
    #[inline]
    fn from(hyper_sphere: HyperSphere<T>) -> Self {
        Self::new_vector(Vector2::from(hyper_sphere.center), hyper_sphere.radius)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Line2D<T> {
    pub start: Vector2<T>,
    pub end: Vector2<T>,
}

impl<T> Line2D<T> {
    #[inline]
    pub fn new(start_x: T, start_y: T, end_x: T, end_y: T) -> Line2D<T> {
        Self::new_vectors(Vector2::new_comp(start_x, start_y), Vector2::new_comp(end_x, end_y))
    }

    #[inline]
    pub fn new_vectors(start: Vector2<T>, end: Vector2<T>) -> Line2D<T> {
        Line2D { start, end, }
    }

    #[inline]
    pub fn set(&mut self, start_x: T, start_y: T, end_x: T, end_y: T) {
        self.start.x = start_x;
        self.start.y = start_y;
        self.end.x = end_x;
        self.end.y = end_y;
    }

    #[inline]
    pub fn set_vectors(&mut self, start: Vector2<T>, end: Vector2<T>) {
        self.start = start;
        self.end = end;
    }

    #[inline]
    pub fn get_length(&self) -> T
    where T: Sub<Output = T> + Mul<Output = T> + Real {
        let delta = self.end - self.start;
        delta.magnitude()
    }

    #[inline]
    pub fn set_length(&mut self, length: T)
    where T: DivAssign + Real {
        let delta = self.end - self.start;
        let delta_normalized = delta.normalized();
        self.end = self.start + delta_normalized * length;
    }

    #[inline]
    pub fn get_sqr_length(&self) -> T
    where T: Sub<Output = T> + Mul<Output = T> + Real {
        let delta = self.end - self.start;
        delta.sqr_magnitude()
    }

    #[inline]
    pub fn set_sqr_length(&mut self, sqr_length: T)
    where T: DivAssign + Real {
        self.set_length(sqr_length.sqrt());
    }

    #[inline]
    pub fn get_direction(&self) -> Vector2<T>
    where T: DivAssign + Real {
        let delta = self.end - self.start;
        delta.normalized()
    }

    #[inline]
    pub fn set_direction(&mut self, direction: Vector2<T>)
    where T: Real {
        let delta = self.end - self.start;
        let length = delta.magnitude();
        self.end = self.start + direction * length;
    }

    #[inline]
    pub fn get_delta(&self) -> Vector2<T>
    where T: Sub<Output = T> + Copy {
        self.end - self.start
    }

    #[inline]
    pub fn set_delta(&mut self, delta: Vector2<T>)
    where T: Add<Output = T> + Copy {
        self.end = self.start + delta;
    }

    #[inline]
    pub fn get_center(&self) -> Vector2<T>
    where T: Real {
        (self.start + self.end) / (T::one() + T::one())
    }

    #[inline]
    pub fn set_center(&mut self, center: Vector2<T>)
    where T: Real {
        let delta = self.end - self.start;
        self.start = center - delta / (T::one() + T::one());
        self.end = center + delta / (T::one() + T::one());
    }

    // Ported from https://forum.unity.com/threads/line-intersection.17384/
    #[inline]
    pub fn intersects(&self, other: &Line2D<T>) -> Option<Vector2<T>>
    where T: Real + PartialOrd {
        let p1 = self.start;
        let p2 = self.end;
        let p3 = other.start;
        let p4 = other.end;
        
        let ax = p2.x-p1.x;
        let bx = p3.x-p4.x;
        let x1lo;
        let x1hi;
        
        if ax < T::zero() {
            x1lo = p2.x;
            x1hi = p1.x;
        } else {
            x1lo = p1.x;
            x1hi = p2.x;
        }

        if bx > T::zero() {
            if x1hi < p4.x || p3.x < x1lo {
                return None;
            }
        } else {
            if x1hi < p3.x || p4.x < x1lo {
                return None;
            }
        }

        let ay = p2.y-p1.y;
        let by = p3.y-p4.y;
        let y1lo;
        let y1hi;

        if ay < T::zero() {
            y1lo = p2.y;
            y1hi = p1.y;
        } else {
            y1lo = p1.y;
            y1hi = p2.y;
        }

        if by > T::zero() {
            if y1hi < p4.y || p3.y < y1lo {
                return None;
            }
        } else {
            if y1hi < p3.y || p4.y < y1lo {
                return None;
            }
        }

        let cx = p1.x-p3.x;
        let cy = p1.y-p3.y;
        let d = by*cx - bx*cy; // alpha numerator
        let f = ay*bx - ax*by; // both denominator

        // alpha tests
        if f > T::zero() {
            if d < T::zero() || d > f {
                return None;
            }
        } else {
            if d > T::zero() || d < f {
                return None;
            }
        }

        let e = ax*cy - ay*cx; // beta numerator

        // beta tests
        if f > T::zero() {
            if e < T::zero() || e > f {
                return None;
            }
        } else {
            if e > T::zero() || e < f {
                return None;
            }
        }

        // check if they are parallel
        if f == T::zero() {
            return None;
        }
        
        // compute intersection coordinates
        let mut num = d*ax; // numerator
        let x = p1.x + num / f;
        num = d*ay;
        let y = p1.y + num / f;

        Some(Vector2::new(x, y))
    }
}

impl<T> From<Line3D<T>> for Line2D<T> {
    #[inline]
    fn from(line: Line3D<T>) -> Self {
        Line2D::new(line.start.x, line.start.y, line.end.x, line.end.y)
    }
}

impl<T> From<Line4D<T>> for Line2D<T> {
    #[inline]
    fn from(line: Line4D<T>) -> Self {
        Line2D::new(line.start.x, line.start.y, line.end.x, line.end.y)
    }
}




struct Cube<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub width: T,
    pub height: T,
    pub depth: T,
}

impl<T> Cube<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T, width: T, height: T, depth: T) -> Self {
        Self { x, y, z, width, height, depth }
    }

    #[inline]
    pub fn new_vectors(position: Vector3<T>, size: Vector3<T>) -> Self {
        Self::new(position.x, position.y, position.z, size.x, size.y, size.z)
    }

    #[inline]
    pub fn set(&mut self, x: T, y: T, z: T, width: T, height: T, depth: T) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.width = width;
        self.height = height;
        self.depth = depth;
    }

    #[inline]
    pub fn set_vectors(&mut self, position: Vector3<T>, size: Vector3<T>) {
        self.x = position.x;
        self.y = position.y;
        self.z = position.z;
        self.width = size.x;
        self.height = size.y;
        self.depth = size.z;
    }

    #[inline]
    pub fn get_position(&self) -> Vector3<T>
    where T: Copy {
        Vector3::new(self.x, self.y, self.z)
    }

    #[inline]
    pub fn set_position(&mut self, position: Vector3<T>)
    where T: Copy {
        self.x = position.x;
        self.y = position.y;
        self.z = position.z;
    }

    #[inline]
    pub fn get_size(&self) -> Vector3<T>
    where T: Copy {
        Vector3::new(self.width, self.height, self.depth)
    }

    #[inline]
    pub fn set_size(&mut self, size: Vector3<T>)
    where T: Copy {
        self.width = size.x;
        self.height = size.y;
        self.depth = size.z;
    }

    #[inline]
    pub fn get_center(&self) -> Vector3<T>
    where T: Real {
        let two = T::one() + T::one();
        Vector3::new(self.x + self.width / two , self.y + self.height / two, self.z + self.depth / two)
    }

    #[inline]
    pub fn set_center(&mut self, center: Vector3<T>)
    where T: Real {
        let two = T::one() + T::one();
        self.x = center.x - self.width / two;
        self.y = center.y - self.height / two;
        self.z = center.z - self.depth / two;
    }

    #[inline]
    pub fn get_x_min(&self) -> T
    where T: Copy {
        self.x
    }

    #[inline]
    pub fn set_x_min(&mut self, x_min: T)
    where T: Copy {
        self.x = x_min;
    }

    #[inline]
    pub fn get_x_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.x + self.width
    }

    #[inline]
    pub fn set_x_max(&mut self, x_max: T)
    where T: Sub<Output = T> + Copy {
        self.width = x_max - self.x;
    }

    #[inline]
    pub fn get_y_min(&self) -> T
    where T: Copy {
        self.y
    }

    #[inline]
    pub fn set_y_min(&mut self, y_min: T)
    where T: Copy {
        self.y = y_min;
    }

    #[inline]
    pub fn get_y_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.y + self.height
    }

    #[inline]
    pub fn set_y_max(&mut self, y_max: T)
    where T: Sub<Output = T> + Copy {
        self.height = y_max - self.y;
    }

    #[inline]
    pub fn get_z_min(&self) -> T
    where T: Copy {
        self.z
    }

    #[inline]
    pub fn set_z_min(&mut self, z_min: T)
    where T: Copy {
        self.z = z_min;
    }

    #[inline]
    pub fn get_z_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.z + self.depth
    }

    #[inline]
    pub fn set_z_max(&mut self, z_max: T)
    where T: Sub<Output = T> + Copy {
        self.depth = z_max - self.z;
    }

    #[inline]
    pub fn contains(&self, point: Vector3<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Copy {
        point.x >= self.x &&
        point.x <= self.x + self.width &&
        point.y >= self.y &&
        point.y <= self.y + self.height &&
        point.z >= self.z &&
        point.z <= self.z + self.depth
    }

    #[inline]
    pub fn overlaps(&self, other: &Cube<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Copy {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y &&
        self.z < other.z + other.depth &&
        self.z + self.depth > other.z
    }

    #[inline]
    pub fn overlaps_area(&self, area: Area3D<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy {
        let other = Cube::from(area);
        self.overlaps(&other)
    }

    #[inline]
    pub fn overlaps_bounds(&self, bounds: Bounds3D<T>) -> bool
    where T: PartialOrd + Real {
        let other = Cube::from(bounds);
        self.overlaps(&other)
    }

    // #[inline]
    // pub fn overlaps_sphere(&self, circle: Circle<T>) -> bool {
    //    
    // }
}

impl<T> From<Area3D<T>> for Cube<T>
where T: Sub<Output = T> + Copy {
    #[inline]
    fn from(area: Area3D<T>) -> Self {
        Cube::new(area.lower_left.x, area.lower_left.y, area.lower_left.z, area.upper_right.x - area.lower_left.x, area.upper_right.y - area.lower_left.y, area.upper_right.z - area.lower_left.z)
    }
}

impl<T> From<Bounds3D<T>> for Cube<T>
where T: Real {
    #[inline]
    fn from(bounds: Bounds3D<T>) -> Self {
        let position = bounds.center - bounds.extents;
        let size = bounds.extents * (T::one() + T::one());
        Cube::new_vectors(position, size)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Area3D<T> {
    pub lower_left: Vector3<T>,
    pub upper_right: Vector3<T>,
}

impl<T> Area3D<T> {
    #[inline]
    pub fn new(lower_left_x: T, lower_left_y: T, lower_left_z: T, upper_right_x: T, upper_right_y: T, upper_right_z: T) -> Self {
        Self::new_vectors(Vector3::new_comp(lower_left_x, lower_left_y, lower_left_z), Vector3::new_comp(upper_right_x, upper_right_y, upper_right_z))
    }
    
    #[inline]
    pub fn new_vectors(lower_left: Vector3<T>, upper_right: Vector3<T>) -> Self {
        Area3D { lower_left, upper_right }
    }

    #[inline]
    pub fn set(&mut self, lower_left_x: T, lower_left_y: T, lower_left_z: T, upper_right_x: T, upper_right_y: T, upper_right_z: T) {
        self.lower_left.x = lower_left_x;
        self.lower_left.y = lower_left_y;
        self.lower_left.z = lower_left_z;
        self.upper_right.x = upper_right_x;
        self.upper_right.y = upper_right_y;
        self.upper_right.z = upper_right_z;
    }

    #[inline]
    pub fn set_vectors(&mut self, lower_left: Vector3<T>, upper_right: Vector3<T>) {
        self.lower_left = lower_left;
        self.upper_right = upper_right;
    }

    #[inline]
    pub fn get_x_min(&self) -> T
    where T: Copy {
        self.lower_left.x
    }

    #[inline]
    pub fn set_x_min(&mut self, x_min: T)
    where T: Copy {
        self.lower_left.x = x_min;
    }

    #[inline]
    pub fn get_x_max(&self) -> T
    where T: Copy {
        self.upper_right.x
    }

    #[inline]
    pub fn set_x_max(&mut self, x_max: T)
    where T: Copy {
        self.upper_right.x = x_max;
    }

    #[inline]
    pub fn get_y_min(&self) -> T
    where T: Copy {
        self.lower_left.y
    }

    #[inline]
    pub fn set_y_min(&mut self, y_min: T)
    where T: Copy {
        self.lower_left.y = y_min;
    }

    #[inline]
    pub fn get_y_max(&self) -> T
    where T: Copy {
        self.upper_right.y
    }

    #[inline]
    pub fn set_y_max(&mut self, y_max: T)
    where T: Copy {
        self.upper_right.y = y_max;
    }

    #[inline]
    pub fn get_z_min(&self) -> T
    where T: Copy {
        self.lower_left.z
    }

    #[inline]
    pub fn set_z_min(&mut self, z_min: T)
    where T: Copy {
        self.lower_left.z = z_min;
    }

    #[inline]
    pub fn get_z_max(&self) -> T
    where T: Copy {
        self.upper_right.z
    }

    #[inline]
    pub fn set_z_max(&mut self, z_max: T)
    where T: Copy {
        self.upper_right.z = z_max;
    }

    #[inline]
    pub fn get_width(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.upper_right.x - self.lower_left.x
    }

    #[inline]
    pub fn set_width(&mut self, width: T)
    where T: AddAssign + SubAssign + Real {
        let current_width = self.upper_right.x - self.lower_left.x;
        let delta = current_width - width;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left.x += half_delta;
        self.upper_right.x -= half_delta;
    }

    #[inline]
    pub fn get_height(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.upper_right.y - self.lower_left.y
    }

    #[inline]
    pub fn set_height(&mut self, height: T)
    where T: AddAssign + SubAssign + Real {
        let current_height = self.upper_right.y - self.lower_left.y;
        let delta = current_height - height;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left.y += half_delta;
        self.upper_right.y -= half_delta;
    }
    
    #[inline]
    pub fn get_depth(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.upper_right.z - self.lower_left.z
    }

    #[inline]
    pub fn set_depth(&mut self, depth: T)
    where T: AddAssign + SubAssign + Real {
        let current_height = self.upper_right.z - self.lower_left.z;
        let delta = current_height - depth;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left.z += half_delta;
        self.upper_right.z -= half_delta;
    }
    
    #[inline]
    pub fn get_size(&self) -> Vector3<T>
    where T: Sub<Output = T> + Copy {
        Vector3::new_comp(self.upper_right.x - self.lower_left.x, self.upper_right.y - self.lower_left.y, self.upper_right.z - self.lower_left.z)
    }

    #[inline]
    pub fn set_size(&mut self, size: Vector3<T>)
    where T: AddAssign + SubAssign + Real {
        let current_size = Vector3::new_comp(self.upper_right.x - self.lower_left.x, self.upper_right.y - self.lower_left.y, self.upper_right.z - self.lower_left.z);
        let delta = current_size - size;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left += half_delta;
        self.upper_right -= half_delta;
    }

    #[inline]
    pub fn get_center(&self) -> Vector3<T>
    where T: Real {
        Vector3::new_comp(
        (self.lower_left.x + self.upper_right.x) / (T::one() + T::one()),
        (self.lower_left.y + self.upper_right.y) / (T::one() + T::one()),
        (self.lower_left.z + self.upper_right.z) / (T::one() + T::one()))
    }

    #[inline]
    pub fn set_center(&mut self, center: Vector3<T>)
    where T: AddAssign + Real {
        let current_center = self.get_center();
        let delta = center - current_center;
        self.lower_left += delta;
        self.upper_right += delta;
    }

    #[inline]
    pub fn contains(&self, point: Vector3<T>) -> bool
    where T: PartialOrd + Copy {
        point.x >= self.lower_left.x &&
        point.x <= self.upper_right.x &&
        point.y >= self.lower_left.y &&
        point.y <= self.upper_right.y &&
        point.z >= self.lower_left.z &&
        point.z <= self.upper_right.z
    }

    #[inline]
    pub fn overlaps(&self, other: &Area3D<T>) -> bool
    where T: PartialOrd + Copy {
        self.lower_left.x < other.upper_right.x &&
        self.upper_right.x > other.lower_left.x &&
        self.lower_left.y < other.upper_right.y &&
        self.upper_right.y > other.lower_left.y &&
        self.lower_left.z < other.upper_right.z &&
        self.upper_right.z > other.lower_left.z
    }

    #[inline]
    pub fn overlaps_bounds(&self, bounds: &Bounds3D<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy {
        self.lower_left.x < bounds.center.x + bounds.extents.x &&
        self.upper_right.x > bounds.center.x - bounds.extents.x &&
        self.lower_left.y < bounds.center.y + bounds.extents.y &&
        self.upper_right.y > bounds.center.y - bounds.extents.y &&
        self.lower_left.z < bounds.center.z + bounds.extents.z &&
        self.upper_right.z > bounds.center.z - bounds.extents.z
    }
}

impl<T> From<Bounds3D<T>> for Area3D<T>
where T: Add<Output = T> + Sub<Output = T> + Copy {
    #[inline]
    fn from(bounds: Bounds3D<T>) -> Self {
        Self::new(
            bounds.center.x - bounds.extents.x,
            bounds.center.y - bounds.extents.y,
            bounds.center.z - bounds.extents.z,
            bounds.center.x + bounds.extents.x,
            bounds.center.y + bounds.extents.y,
            bounds.center.z + bounds.extents.z)
    }
}

impl<T> From<Area4D<T>> for Area3D<T> {
    #[inline]
    fn from(area: Area4D<T>) -> Self {
        Self::new(area.lower_left.x, area.lower_left.y, area.lower_left.z, area.upper_right.x, area.upper_right.y, area.upper_right.z)
    }
}

impl<T> From<Bounds4D<T>> for Area3D<T>
where T: Add<Output = T> + Sub<Output = T> + Copy {
    #[inline]
    fn from(bounds: Bounds4D<T>) -> Self {
        Self::new(
            bounds.center.x - bounds.extents.x,
            bounds.center.y - bounds.extents.y,
            bounds.center.z - bounds.extents.z,
            bounds.center.x + bounds.extents.x,
            bounds.center.y + bounds.extents.y,
            bounds.center.z + bounds.extents.z)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Bounds3D<T> {
    pub center: Vector3<T>,
    pub extents: Vector3<T>,
}

impl<T> Bounds3D<T> {
    #[inline]
    pub fn new(center_x: T, center_y: T, center_z: T, extents_x: T, extents_y: T, extents_z: T) -> Self
    where T: Copy {
        Self::new_vectors(Vector3::new(center_x, center_y,  center_z), Vector3::new(extents_x, extents_y, extents_z))
    }
    
    #[inline]
    pub fn new_vectors(center: Vector3<T>, extents: Vector3<T>) -> Self {
        Bounds3D { center, extents }
    }

    #[inline]
    pub fn set(&mut self, center_x: T, center_y: T, center_z: T, extents_x: T, extents_y: T, extents_z: T)
    where T: Copy {
        self.center.x = center_x;
        self.center.y = center_y;
        self.center.z = center_z;
        self.extents.x = extents_x;
        self.extents.y = extents_y;
        self.extents.z = extents_z;
    }

    #[inline]
    pub fn set_vectors(&mut self, center: Vector3<T>, extents: Vector3<T>) {
        self.center = center;
        self.extents = extents;
    }

    #[inline]
    pub fn get_x_min(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.center.x - self.extents.x
    }

    #[inline]
    pub fn set_x_min(&mut self, x_min: T)
    where T: Add<Output = T> + Copy {
        self.center.x = x_min + self.extents.x;
    }

    #[inline]
    pub fn get_x_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.center.x + self.extents.x
    }

    #[inline]
    pub fn set_x_max(&mut self, x_max: T)
    where T: Sub<Output = T> + Copy {
        self.center.x = x_max - self.extents.x;
    }

    #[inline]
    pub fn get_y_min(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.center.y - self.extents.y
    }

    #[inline]
    pub fn set_y_min(&mut self, y_min: T)
    where T: Add<Output = T> + Copy {
        self.center.y = y_min + self.extents.y;
    }

    #[inline]
    pub fn get_y_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.center.y + self.extents.y
    }

    #[inline]
    pub fn set_y_max(&mut self, y_max: T)
    where T: Sub<Output = T> + Copy {
        self.center.y = y_max - self.extents.y;
    }

    #[inline]
    pub fn get_z_min(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.center.z - self.extents.z
    }

    #[inline]
    pub fn set_z_min(&mut self, z_min: T)
    where T: Add<Output = T> + Copy {
        self.center.z = z_min + self.extents.z;
    }

    #[inline]
    pub fn get_z_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.center.z + self.extents.z
    }

    #[inline]
    pub fn set_z_max(&mut self, z_max: T)
    where T: Sub<Output = T> + Copy {
        self.center.z = z_max - self.extents.z;
    }

    #[inline]
    pub fn get_width(&self) -> T
    where T: Add<Output = T> + Copy {
        self.extents.x + self.extents.x
    }

    #[inline]
    pub fn set_width(&mut self, width: T)
    where T: SubAssign + Real {
        let current_width = self.extents.x + self.extents.x;
        let delta = current_width - width;
        let half_delta = delta / (T::one() + T::one());
        self.extents.x -= half_delta;
    }

    #[inline]
    pub fn get_height(&self) -> T
    where T: Add<Output = T> + Copy {
        self.extents.y + self.extents.y
    }

    #[inline]
    pub fn set_height(&mut self, height: T)
    where T: SubAssign + Real {
        let current_height = self.extents.y + self.extents.y;
        let delta = current_height - height;
        let half_delta = delta / (T::one() + T::one());
        self.extents.y -= half_delta;
    }

    #[inline]
    pub fn get_depth(&self) -> T
    where T: Add<Output = T> + Copy {
        self.extents.z + self.extents.z
    }

    #[inline]
    pub fn set_depth(&mut self, depth: T)
    where T: SubAssign + Real {
        let current_height = self.extents.z + self.extents.z;
        let delta = current_height - depth;
        let half_delta = delta / (T::one() + T::one());
        self.extents.z -= half_delta;
    }

    #[inline]
    pub fn get_size(&self) -> Vector3<T>
    where T: Add<Output = T> + Copy {
        Vector3::new(self.extents.x + self.extents.x, self.extents.y + self.extents.y, self.extents.z + self.extents.z)
    }

    #[inline]
    pub fn set_size(&mut self, size_x: T, size_y: T, size_z: T)
    where T: SubAssign + Copy + Real {
        let current_size = Vector3::new(self.extents.x + self.extents.x, self.extents.y + self.extents.y, self.extents.z + self.extents.z);
        let delta = current_size - Vector3::new(size_x, size_y, size_z);
        let half_delta = delta / (T::one() + T::one());
        self.extents -= half_delta;
    }

    #[inline]
    pub fn contains(&self, point: Vector3<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < point.x &&
        self.center.x + self.extents.x > point.x &&
        self.center.y - self.extents.y < point.y &&
        self.center.y + self.extents.y > point.y &&
        self.center.z - self.extents.z < point.z &&
        self.center.z + self.extents.z > point.z
    }

    #[inline]
    pub fn overlaps(&self, other: &Bounds3D<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < other.center.x + other.extents.x &&
        self.center.x + self.extents.x > other.center.x - other.extents.x &&
        self.center.y - self.extents.y < other.center.y + other.extents.y &&
        self.center.y + self.extents.y > other.center.y - other.extents.y &&
        self.center.z - self.extents.z < other.center.z + other.extents.z &&
        self.center.z + self.extents.z > other.center.z - other.extents.z
    }

    #[inline]
    pub fn overlaps_area(&self, area: &Area3D<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < area.get_x_max() &&
        self.center.x + self.extents.x > area.get_x_min() &&
        self.center.y - self.extents.y < area.get_y_max() &&
        self.center.y + self.extents.y > area.get_y_min() &&
        self.center.z - self.extents.z < area.get_z_max() &&
        self.center.z + self.extents.z > area.get_z_min()
    }
}

impl<T> From<Area3D<T>> for Bounds3D<T>
where T: Real {
    #[inline]
    fn from(area: Area3D<T>) -> Self {
        Self::new(
            area.get_center().x,
            area.get_center().y,
            area.get_center().z,
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()))
    }
}

impl<T> From<Bounds2D<T>> for Bounds3D<T>
where T: Real {
    #[inline]
    fn from(bounds: Bounds2D<T>) -> Self {
        Self::new(
            bounds.center.x,
            bounds.center.y,
            T::zero(),
            bounds.extents.x,
            bounds.extents.y,
            T::zero())
    }
}

impl<T> From<Area2D<T>> for Bounds3D<T>
where T: Real {
    #[inline]
    fn from(area: Area2D<T>) -> Self {
        Self::new(
            area.get_center().x,
            area.get_center().y,
            T::zero(),
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()),
            T::zero())
    }
}

impl<T> From<Bounds4D<T>> for Bounds3D<T>
where T: Copy {
    #[inline]
    fn from(bounds: Bounds4D<T>) -> Self {
        Self::new(
            bounds.center.x,
            bounds.center.y,
            bounds.center.z,
            bounds.extents.x,
            bounds.extents.y,
            bounds.extents.z)
    }
}

impl<T> From<Area4D<T>> for Bounds3D<T>
where T: Real {
    #[inline]
    fn from(area: Area4D<T>) -> Self {
        Self::new(
            area.get_center().x,
            area.get_center().y,
            area.get_center().z,
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()))
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Sphere<T> {
    pub center: Vector3<T>,
    pub radius: T,
}

impl<T> Sphere<T> {
    #[inline]
    pub fn new(center_x: T, center_y: T, center_z: T, radius: T) -> Self {
        Self::new_vector(Vector3::new(center_x, center_y, center_z), radius)
    }
    
    #[inline]
    pub fn new_vector(center: Vector3<T>, radius: T) -> Self {
        Self { center, radius, }
    }

    #[inline]
    pub fn get_diameter(&self) -> T
    where T: Add<Output = T> + Copy {
        self.radius + self.radius
    }

    #[inline]
    pub fn set_diameter(&mut self, diameter: T)
    where T: Real {
        self.radius = diameter / (T::one() + T::one());
    }

    #[inline]
    pub fn get_circumference(&self) -> T
    where T: Real + Pi<Output = T> {
        self.radius * (T::one() + T::one()) * T::pi()
    }

    #[inline]
    pub fn set_circumference(&mut self, circumference: T)
    where T: Real + Pi<Output = T> {
        self.radius = circumference / ((T::one() + T::one()) * T::pi());
    }

    #[inline]
    pub fn get_area(&self) -> T
    where T: Real + Pi<Output = T> {
        (T::one() + T::one() + T::one() + T::one()) * T::pi() * self.radius * (T::one() + T::one())
    }

    #[inline]
    pub fn set_area(&mut self, area: T)
    where T: Real + Pi<Output = T> {
        self.radius = (area / ((T::one() + T::one() + T::one() + T::one()) * T::pi())).sqrt();
    }

    #[inline]
    pub fn get_volume(&self) -> T
    where T: Real + Pi<Output = T> {
        ((T::one() + T::one() + T::one() + T::one()) / (T::one() + T::one() + T::one())) * T::pi() * self.radius * self.radius * self.radius
    }

    #[inline]
    pub fn set_volume(&mut self, area: T)
    where T: Real + Pi<Output = T> {
        self.radius = (area / ((T::one() + T::one() + T::one() + T::one()) / (T::one() + T::one() + T::one())) * T::pi()).cbrt();
    }

    #[inline]
    pub fn contains(&self, point: Vector3<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd + Copy {
        let delta = point - self.center;
        let distance_squared = delta.sqr_magnitude();
        distance_squared <= self.radius * self.radius
    }

    #[inline]
    pub fn overlaps(&self, other: &Sphere<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd + Copy {
        let delta = other.center - self.center;
        let distance_squared = delta.sqr_magnitude();
        let radius_sum = self.radius + other.radius;
        distance_squared < radius_sum * radius_sum
    }
}

impl<T> From<Circle<T>> for Sphere<T>
where T: Real {
    #[inline]
    fn from(circle: Circle<T>) -> Self {
        Self::new_vector(Vector3::from(circle.center), circle.radius)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Line3D<T> {
    pub start: Vector3<T>,
    pub end: Vector3<T>,
}

impl<T> Line3D<T> {
    #[inline]
    pub fn new(start_x: T, start_y: T, start_z: T, end_x: T, end_y: T, end_z: T) -> Self {
        Self::new_vectors(Vector3::new(start_x, start_y, start_z), Vector3::new(end_x, end_y, end_z))
    }

    #[inline]
    pub fn new_vectors(start: Vector3<T>, end: Vector3<T>) -> Self {
        Self { start, end, }
    }

    #[inline]
    pub fn set(&mut self, start_x: T, start_y: T, start_z: T, end_x: T, end_y: T, end_z: T) {
        self.start.x = start_x;
        self.start.y = start_y;
        self.start.z = start_z;
        self.end.x = end_x;
        self.end.y = end_y;
        self.end.z = end_z;
    }

    #[inline]
    pub fn set_vectors(&mut self, start: Vector3<T>, end: Vector3<T>) {
        self.start = start;
        self.end = end;
    }

    #[inline]
    pub fn get_length(&self) -> T
    where T: Sub<Output = T> + Mul<Output = T> + Real {
        let delta = self.end - self.start;
        delta.magnitude()
    }

    #[inline]
    pub fn set_length(&mut self, length: T)
    where T: DivAssign + Real {
        let delta = self.end - self.start;
        let delta_normalized = delta.normalized();
        self.end = self.start + delta_normalized * length;
    }

    #[inline]
    pub fn get_sqr_length(&self) -> T
    where T: Sub<Output = T> + Mul<Output = T> + Real {
        let delta = self.end - self.start;
        delta.sqr_magnitude()
    }

    #[inline]
    pub fn set_sqr_length(&mut self, sqr_length: T)
    where T: DivAssign + Real {
        self.set_length(sqr_length.sqrt());
    }

    #[inline]
    pub fn get_direction(&self) -> Vector3<T>
    where T: DivAssign + Real {
        let delta = self.end - self.start;
        delta.normalized()
    }

    #[inline]
    pub fn set_direction(&mut self, direction: Vector3<T>)
    where T: Real {
        let delta = self.end - self.start;
        let length = delta.magnitude();
        self.end = self.start + direction * length;
    }

    #[inline]
    pub fn get_delta(&self) -> Vector3<T>
    where T: Sub<Output = T> + Copy {
        self.end - self.start
    }

    #[inline]
    pub fn set_delta(&mut self, delta: Vector3<T>)
    where T: Add<Output = T> + Copy {
        self.end = self.start + delta;
    }

    #[inline]
    pub fn get_center(&self) -> Vector3<T>
    where T: Real {
        (self.start + self.end) / (T::one() + T::one())
    }

    #[inline]
    pub fn set_center(&mut self, center: Vector3<T>)
    where T: Real {
        let delta = self.end - self.start;
        self.start = center - delta / (T::one() + T::one());
        self.end = center + delta / (T::one() + T::one());
    }

    // #[inline]
    // pub fn intersects(&self, other: &Line3D<T>) -> bool {
    //     todo!()
    // }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Area4D<T> {
    pub lower_left: Vector4<T>,
    pub upper_right: Vector4<T>,
}

impl<T> Area4D<T> {
    #[inline]
    pub fn new(lower_left_x: T, lower_left_y: T, lower_left_z: T, lower_left_w: T, upper_right_x: T, upper_right_y: T, upper_right_z: T, upper_right_w: T) -> Self {
        Self::new_vectors(
            Vector4::new(lower_left_x, lower_left_y, lower_left_z, lower_left_w),
            Vector4::new(upper_right_x, upper_right_y, upper_right_z, upper_right_w))
    }
    
    #[inline]
    pub fn new_vectors(lower_left: Vector4<T>, upper_right: Vector4<T>) -> Self {
        Self { lower_left, upper_right }
    }

    #[inline]
    pub fn set(&mut self, lower_left_x: T, lower_left_y: T, lower_left_z: T, lower_left_w: T, upper_right_x: T, upper_right_y: T, upper_right_z: T, upper_right_w: T) {
        self.lower_left.x = lower_left_x;
        self.lower_left.y = lower_left_y;
        self.lower_left.z = lower_left_z;
        self.lower_left.w = lower_left_w;
        self.upper_right.x = upper_right_x;
        self.upper_right.y = upper_right_y;
        self.upper_right.z = upper_right_z;
        self.upper_right.w = upper_right_w;
    }

    #[inline]
    pub fn set_vectors(&mut self, lower_left: Vector4<T>, upper_right: Vector4<T>) {
        self.lower_left = lower_left;
        self.upper_right = upper_right;
    }

    #[inline]
    pub fn get_x_min(&self) -> T
    where T: Copy {
        self.lower_left.x
    }

    #[inline]
    pub fn set_x_min(&mut self, x_min: T)
    where T: Copy {
        self.lower_left.x = x_min;
    }

    #[inline]
    pub fn get_x_max(&self) -> T
    where T: Copy {
        self.upper_right.x
    }

    #[inline]
    pub fn set_x_max(&mut self, x_max: T)
    where T: Copy {
        self.upper_right.x = x_max;
    }

    #[inline]
    pub fn get_y_min(&self) -> T
    where T: Copy {
        self.lower_left.y
    }

    #[inline]
    pub fn set_y_min(&mut self, y_min: T)
    where T: Copy {
        self.lower_left.y = y_min;
    }

    #[inline]
    pub fn get_y_max(&self) -> T
    where T: Copy {
        self.upper_right.y
    }

    #[inline]
    pub fn set_y_max(&mut self, y_max: T)
    where T: Copy {
        self.upper_right.y = y_max;
    }

    #[inline]
    pub fn get_z_min(&self) -> T
    where T: Copy {
        self.lower_left.z
    }

    #[inline]
    pub fn set_z_min(&mut self, z_min: T)
    where T: Copy {
        self.lower_left.z = z_min;
    }

    #[inline]
    pub fn get_z_max(&self) -> T
    where T: Copy {
        self.upper_right.z
    }

    #[inline]
    pub fn set_z_max(&mut self, z_max: T)
    where T: Copy {
        self.upper_right.z = z_max;
    }

    #[inline]
    pub fn get_w_min(&self) -> T
    where T: Copy {
        self.lower_left.w
    }

    #[inline]
    pub fn set_w_min(&mut self, w_min: T)
    where T: Copy {
        self.lower_left.w = w_min;
    }

    #[inline]
    pub fn get_w_max(&self) -> T
    where T: Copy {
        self.upper_right.w
    }

    #[inline]
    pub fn set_w_max(&mut self, w_max: T)
    where T: Copy {
        self.upper_right.w = w_max;
    }

    #[inline]
    pub fn get_width(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.upper_right.w - self.lower_left.w
    }

    #[inline]
    pub fn set_width(&mut self, width: T)
    where T: AddAssign + SubAssign + Real {
        let current_width = self.upper_right.x - self.lower_left.x;
        let delta = current_width - width;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left.x += half_delta;
        self.upper_right.x -= half_delta;
    }

    #[inline]
    pub fn get_height(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.upper_right.y - self.lower_left.y
    }

    #[inline]
    pub fn set_height(&mut self, height: T)
    where T: AddAssign + SubAssign + Real {
        let current_height = self.upper_right.y - self.lower_left.y;
        let delta = current_height - height;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left.y += half_delta;
        self.upper_right.y -= half_delta;
    }
    
    #[inline]
    pub fn get_depth(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.upper_right.z - self.lower_left.z
    }

    #[inline]
    pub fn set_depth(&mut self, depth: T)
    where T: AddAssign + SubAssign + Real {
        let current_height = self.upper_right.z - self.lower_left.z;
        let delta = current_height - depth;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left.z += half_delta;
        self.upper_right.z -= half_delta;
    }
    
    #[inline]
    pub fn get_hyper_depth(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.upper_right.w - self.lower_left.w
    }

    #[inline]
    pub fn set_hyper_depth(&mut self, depth: T)
    where T: AddAssign + SubAssign + Real {
        let current_height = self.upper_right.w - self.lower_left.w;
        let delta = current_height - depth;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left.w += half_delta;
        self.upper_right.w -= half_delta;
    }
    
    #[inline]
    pub fn get_size(&self) -> Vector4<T>
    where T: Sub<Output = T> + Copy {
        Vector4::new(self.upper_right.x - self.lower_left.x, self.upper_right.y - self.lower_left.y, self.upper_right.z - self.lower_left.z, self.upper_right.w - self.lower_left.w)
    }

    #[inline]
    pub fn set_size(&mut self, size: Vector4<T>)
    where T: AddAssign + SubAssign + Real {
        let current_size = Vector4::new(
            self.upper_right.x - self.lower_left.x,
            self.upper_right.y - self.lower_left.y,
            self.upper_right.z - self.lower_left.z,
            self.upper_right.w - self.lower_left.w);

        let delta = current_size - size;
        let half_delta = delta / (T::one() + T::one());
        self.lower_left += half_delta;
        self.upper_right -= half_delta;
    }

    #[inline]
    pub fn get_center(&self) -> Vector4<T>
    where T: Real {
        Vector4::new(
        (self.lower_left.x + self.upper_right.x) / (T::one() + T::one()),
        (self.lower_left.y + self.upper_right.y) / (T::one() + T::one()),
        (self.lower_left.z + self.upper_right.z) / (T::one() + T::one()),
        (self.lower_left.w + self.upper_right.w) / (T::one() + T::one()))
    }

    #[inline]
    pub fn set_center(&mut self, center: Vector4<T>)
    where T: AddAssign + Real {
        let current_center = self.get_center();
        let delta = center - current_center;
        self.lower_left += delta;
        self.upper_right += delta;
    }

    #[inline]
    pub fn contains(&self, point: Vector4<T>) -> bool
    where T: PartialOrd + Copy {
        point.x >= self.lower_left.x &&
        point.x <= self.upper_right.x &&
        point.y >= self.lower_left.y &&
        point.y <= self.upper_right.y &&
        point.z >= self.lower_left.z &&
        point.z <= self.upper_right.z
    }

    #[inline]
    pub fn overlaps(&self, other: &Area4D<T>) -> bool
    where T: PartialOrd + Copy {
        self.lower_left.x < other.upper_right.x &&
        self.upper_right.x > other.lower_left.x &&
        self.lower_left.y < other.upper_right.y &&
        self.upper_right.y > other.lower_left.y &&
        self.lower_left.z < other.upper_right.z &&
        self.upper_right.z > other.lower_left.z &&
        self.lower_left.w < other.upper_right.w &&
        self.upper_right.w > other.lower_left.w
    }

    #[inline]
    pub fn overlaps_bounds(&self, bounds: &Bounds4D<T>) -> bool
    where T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy {
        self.lower_left.x < bounds.center.x + bounds.extents.x &&
        self.upper_right.x > bounds.center.x - bounds.extents.x &&
        self.lower_left.y < bounds.center.y + bounds.extents.y &&
        self.upper_right.y > bounds.center.y - bounds.extents.y &&
        self.lower_left.z < bounds.center.z + bounds.extents.z &&
        self.upper_right.z > bounds.center.z - bounds.extents.z &&
        self.lower_left.w < bounds.center.w + bounds.extents.w &&
        self.upper_right.w > bounds.center.w - bounds.extents.w
    }
}

impl<T> From<Bounds4D<T>> for Area4D<T>
where T: Add<Output = T> + Sub<Output = T> + Copy {
    #[inline]
    fn from(bounds: Bounds4D<T>) -> Self {
        Self::new(
            bounds.center.x - bounds.extents.x,
            bounds.center.y - bounds.extents.y,
            bounds.center.z - bounds.extents.z,
            bounds.center.x + bounds.extents.x,
            bounds.center.y + bounds.extents.y,
            bounds.center.z + bounds.extents.z,
            bounds.center.w - bounds.extents.w,
            bounds.center.w + bounds.extents.w)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Bounds4D<T> {
    pub center: Vector4<T>,
    pub extents: Vector4<T>,
}

impl<T> Bounds4D<T> {
    #[inline]
    pub fn new(center_x: T, center_y: T, center_z: T, center_w: T, extents_x: T, extents_y: T, extents_z: T, extents_w: T) -> Self
    where T: Copy {
        Self::new_vectors(
            Vector4::new(center_x, center_y,  center_z, center_w),
            Vector4::new(extents_x, extents_y, extents_z, extents_w))
    }
    
    #[inline]
    pub fn new_vectors(center: Vector4<T>, extents: Vector4<T>) -> Self {
        Self { center, extents }
    }

    #[inline]
    pub fn set(&mut self, center_x: T, center_y: T, center_z: T, center_w: T, extents_x: T, extents_y: T, extents_z: T, extents_w: T)
    where T: Copy {
        self.center.x = center_x;
        self.center.y = center_y;
        self.center.z = center_z;
        self.center.w = center_w;
        self.extents.x = extents_x;
        self.extents.y = extents_y;
        self.extents.z = extents_z;
        self.extents.w = extents_w;
    }

    #[inline]
    pub fn set_vectors(&mut self, center: Vector4<T>, extents: Vector4<T>) {
        self.center = center;
        self.extents = extents;
    }

    #[inline]
    pub fn get_x_min(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.center.x - self.extents.x
    }

    #[inline]
    pub fn set_x_min(&mut self, x_min: T)
    where T: Add<Output = T> + Copy {
        self.center.x = x_min + self.extents.x;
    }

    #[inline]
    pub fn get_x_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.center.x + self.extents.x
    }

    #[inline]
    pub fn set_x_max(&mut self, x_max: T)
    where T: Sub<Output = T> + Copy {
        self.center.x = x_max - self.extents.x;
    }

    #[inline]
    pub fn get_y_min(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.center.y - self.extents.y
    }

    #[inline]
    pub fn set_y_min(&mut self, y_min: T)
    where T: Add<Output = T> + Copy {
        self.center.y = y_min + self.extents.y;
    }

    #[inline]
    pub fn get_y_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.center.y + self.extents.y
    }

    #[inline]
    pub fn set_y_max(&mut self, y_max: T)
    where T: Sub<Output = T> + Copy {
        self.center.y = y_max - self.extents.y;
    }

    #[inline]
    pub fn get_z_min(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.center.z - self.extents.z
    }

    #[inline]
    pub fn set_z_min(&mut self, z_min: T)
    where T: Add<Output = T> + Copy {
        self.center.z = z_min + self.extents.z;
    }

    #[inline]
    pub fn get_z_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.center.z + self.extents.z
    }

    #[inline]
    pub fn set_z_max(&mut self, z_max: T)
    where T: Sub<Output = T> + Copy {
        self.center.z = z_max - self.extents.z;
    }

    #[inline]
    pub fn get_w_min(&self) -> T
    where T: Sub<Output = T> + Copy {
        self.center.w - self.extents.w
    }

    #[inline]
    pub fn set_w_min(&mut self, w_min: T)
    where T: Add<Output = T> + Copy {
        self.center.w = w_min + self.extents.w;
    }

    #[inline]
    pub fn get_w_max(&self) -> T
    where T: Add<Output = T> + Copy {
        self.center.w + self.extents.w
    }

    #[inline]
    pub fn set_w_max(&mut self, w_max: T)
    where T: Sub<Output = T> + Copy {
        self.center.w = w_max - self.extents.w;
    }

    #[inline]
    pub fn get_width(&self) -> T
    where T: Add<Output = T> + Copy {
        self.extents.x + self.extents.x
    }

    #[inline]
    pub fn set_width(&mut self, width: T)
    where T: SubAssign + Real {
        let current_width = self.extents.x + self.extents.x;
        let delta = current_width - width;
        let half_delta = delta / (T::one() + T::one());
        self.extents.x -= half_delta;
    }

    #[inline]
    pub fn get_height(&self) -> T
    where T: Add<Output = T> + Copy {
        self.extents.y + self.extents.y
    }

    #[inline]
    pub fn set_height(&mut self, height: T)
    where T: SubAssign + Real {
        let current_height = self.extents.y + self.extents.y;
        let delta = current_height - height;
        let half_delta = delta / (T::one() + T::one());
        self.extents.y -= half_delta;
    }

    #[inline]
    pub fn get_depth(&self) -> T
    where T: Add<Output = T> + Copy {
        self.extents.z + self.extents.z
    }

    #[inline]
    pub fn set_depth(&mut self, depth: T)
    where T: SubAssign + Real {
        let current_height = self.extents.z + self.extents.z;
        let delta = current_height - depth;
        let half_delta = delta / (T::one() + T::one());
        self.extents.z -= half_delta;
    }

    #[inline]
    pub fn get_hyper_depth(&self) -> T
    where T: Add<Output = T> + Copy {
        self.extents.w + self.extents.w
    }

    #[inline]
    pub fn set_hyper_depth(&mut self, depth: T)
    where T: SubAssign + Real {
        let current_height = self.extents.w + self.extents.w;
        let delta = current_height - depth;
        let half_delta = delta / (T::one() + T::one());
        self.extents.w -= half_delta;
    }

    #[inline]
    pub fn get_size(&self) -> Vector4<T>
    where T: Add<Output = T> + Copy {
        Vector4::new(
            self.extents.x + self.extents.x,
            self.extents.y + self.extents.y,
            self.extents.z + self.extents.z,
            self.extents.w + self.extents.w)
    }

    #[inline]
    pub fn set_size(&mut self, size_x: T, size_y: T, size_z: T, size_w: T)
    where T: SubAssign + Copy + Real {
        let current_size = Vector4::new(self.extents.x + self.extents.x, self.extents.y + self.extents.y, self.extents.z + self.extents.z, self.extents.w + self.extents.w);
        let delta = current_size - Vector4::new(size_x, size_y, size_z, size_w);
        let half_delta = delta / (T::one() + T::one());
        self.extents -= half_delta;
    }

    #[inline]
    pub fn contains(&self, point: Vector4<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < point.x &&
        self.center.x + self.extents.x > point.x &&
        self.center.y - self.extents.y < point.y &&
        self.center.y + self.extents.y > point.y &&
        self.center.z - self.extents.z < point.z &&
        self.center.z + self.extents.z > point.z &&
        self.center.w - self.extents.w < point.w &&
        self.center.w + self.extents.w > point.w
    }

    #[inline]
    pub fn overlaps(&self, other: &Bounds4D<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < other.center.x + other.extents.x &&
        self.center.x + self.extents.x > other.center.x - other.extents.x &&
        self.center.y - self.extents.y < other.center.y + other.extents.y &&
        self.center.y + self.extents.y > other.center.y - other.extents.y &&
        self.center.z - self.extents.z < other.center.z + other.extents.z &&
        self.center.z + self.extents.z > other.center.z - other.extents.z &&
        self.center.w - self.extents.w < other.center.w + other.extents.w &&
        self.center.w + self.extents.w > other.center.w - other.extents.w
    }

    #[inline]
    pub fn overlaps_area(&self, area: &Area4D<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        self.center.x - self.extents.x < area.get_x_max() &&
        self.center.x + self.extents.x > area.get_x_min() &&
        self.center.y - self.extents.y < area.get_y_max() &&
        self.center.y + self.extents.y > area.get_y_min() &&
        self.center.z - self.extents.z < area.get_z_max() &&
        self.center.z + self.extents.z > area.get_z_min() &&
        self.center.w - self.extents.w < area.get_w_max() &&
        self.center.w + self.extents.w > area.get_w_min()
    }
}

impl<T> From<Area4D<T>> for Bounds4D<T>
where T: Real {
    #[inline]
    fn from(area: Area4D<T>) -> Self {
        Self::new(
            area.get_center().x,
            area.get_center().y,
            area.get_center().z,
            area.get_center().w,
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()),
            area.get_width() / (T::one() + T::one()))
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct HyperSphere<T> {
    pub center: Vector4<T>,
    pub radius: T,
}

impl<T> HyperSphere<T> {
    #[inline]
    pub fn new(center_x: T, center_y: T, center_z: T, center_w: T, radius: T) -> Self {
        Self::new_vector(Vector4::new(center_x, center_y, center_z, center_w), radius)
    }
    
    #[inline]
    pub fn new_vector(center: Vector4<T>, radius: T) -> Self {
        Self { center, radius, }
    }

    #[inline]
    pub fn get_diameter(&self) -> T
    where T: Add<Output = T> + Copy {
        self.radius + self.radius
    }

    #[inline]
    pub fn set_diameter(&mut self, diameter: T)
    where T: Real {
        self.radius = diameter / (T::one() + T::one());
    }

    #[inline]
    pub fn get_surface_volume(&self) -> T
    where T: Real + Pi<Output = T> {
        (T::one() + T::one()) * T::pi() * T::pi() * self.radius * self.radius * self.radius
    }

    #[inline]
    pub fn set_surface_volume(&mut self, area: T)
    where T: Real + Pi<Output = T> {
        self.radius = (area / ((T::one() + T::one()) * T::pi() * T::pi())).cbrt();
    }

    #[inline]
    pub fn get_volume(&self) -> T
    where T: Real + Pi<Output = T> {
        T::pi() * T::pi() * self.radius * self.radius * self.radius * self.radius / (T::one() + T::one())
    }

    #[inline]
    pub fn set_volume(&mut self, area: T)
    where T: Real + Pi<Output = T> {
        self.radius = (area / ((T::one() + T::one() + T::one() + T::one()) / (T::one() + T::one() + T::one())) * T::pi()).cbrt();
    }

    #[inline]
    pub fn contains(&self, point: Vector4<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd + Copy {
        let delta = point - self.center;
        let distance_squared = delta.sqr_magnitude();
        distance_squared <= self.radius * self.radius
    }

    #[inline]
    pub fn overlaps(&self, other: &HyperSphere<T>) -> bool
    where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd + Copy {
        let delta = other.center - self.center;
        let distance_squared = delta.sqr_magnitude();
        let radius_sum = self.radius + other.radius;
        distance_squared < radius_sum * radius_sum
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Line4D<T> {
    pub start: Vector4<T>,
    pub end: Vector4<T>,
}

impl<T> Line4D<T> {
    #[inline]
    pub fn new(start_x: T, start_y: T, start_z: T, start_w: T, end_x: T, end_y: T, end_z: T, end_w: T) -> Self {
        Self::new_vectors(
            Vector4::new(start_x, start_y, start_z, start_w),
            Vector4::new(end_x, end_y, end_z, end_w))
    }

    #[inline]
    pub fn new_vectors(start: Vector4<T>, end: Vector4<T>) -> Self {
        Self { start, end, }
    }

    #[inline]
    pub fn set(&mut self, start_x: T, start_y: T, start_z: T, start_w: T, end_x: T, end_y: T, end_z: T, end_w: T) {
        self.start.x = start_x;
        self.start.y = start_y;
        self.start.z = start_z;
        self.start.w = start_w;
        self.end.x = end_x;
        self.end.y = end_y;
        self.end.z = end_z;
        self.end.w = end_w;
    }

    #[inline]
    pub fn set_vectors(&mut self, start: Vector4<T>, end: Vector4<T>) {
        self.start = start;
        self.end = end;
    }

    #[inline]
    pub fn get_length(&self) -> T
    where T: Sub<Output = T> + Mul<Output = T> + Real {
        let delta = self.end - self.start;
        delta.magnitude()
    }

    #[inline]
    pub fn set_length(&mut self, length: T)
    where T: DivAssign + Real {
        let delta = self.end - self.start;
        let delta_normalized = delta.normalized();
        self.end = self.start + delta_normalized * length;
    }

    #[inline]
    pub fn get_sqr_length(&self) -> T
    where T: Sub<Output = T> + Mul<Output = T> + Real {
        let delta = self.end - self.start;
        delta.sqr_magnitude()
    }

    #[inline]
    pub fn set_sqr_length(&mut self, sqr_length: T)
    where T: DivAssign + Real {
        self.set_length(sqr_length.sqrt());
    }

    #[inline]
    pub fn get_direction(&self) -> Vector4<T>
    where T: DivAssign + Real {
        let delta = self.end - self.start;
        delta.normalized()
    }

    #[inline]
    pub fn set_direction(&mut self, direction: Vector4<T>)
    where T: Real {
        let delta = self.end - self.start;
        let length = delta.magnitude();
        self.end = self.start + direction * length;
    }

    #[inline]
    pub fn get_delta(&self) -> Vector4<T>
    where T: Sub<Output = T> + Copy {
        self.end - self.start
    }

    #[inline]
    pub fn set_delta(&mut self, delta: Vector4<T>)
    where T: Add<Output = T> + Copy {
        self.end = self.start + delta;
    }

    #[inline]
    pub fn get_center(&self) -> Vector4<T>
    where T: Real {
        (self.start + self.end) / (T::one() + T::one())
    }

    #[inline]
    pub fn set_center(&mut self, center: Vector4<T>)
    where T: Real {
        let delta = self.end - self.start;
        self.start = center - delta / (T::one() + T::one());
        self.end = center + delta / (T::one() + T::one());
    }

    // #[inline]
    // pub fn intersects(&self, other: &Line3D<T>) -> bool {
    //     todo!()
    // }
}
