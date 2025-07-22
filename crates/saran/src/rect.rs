use crate::point::Point;
use crate::size::Size;

/// A rectangle defined by an origin point and a size.
///
/// # Type Parameters
/// * `T` - The numeric type for the rectangle's coordinates and dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect<T> {
    /// The origin (top-left corner) of the rectangle.
    origin: Point<T>,
    /// The size (width and height) of the rectangle.
    size: Size<T>,
}

impl<T> Rect<T> {
    /// Creates a new `Rect` from an origin and a size.
    ///
    /// # Arguments
    /// * `origin` - The top-left corner of the rectangle.
    /// * `size` - The width and height of the rectangle.
    ///
    /// # Returns
    /// A new `Rect` instance.
    pub fn new(origin: Point<T>, size: Size<T>) -> Self {
        Self { origin, size }
    }

    /// Returns the x-coordinate of the rectangle's origin.
    ///
    /// # Returns
    /// The x-coordinate as type `T`.
    pub fn x(&self) -> T
    where
        T: Copy,
    {
        self.origin.x()
    }

    /// Returns the y-coordinate of the rectangle's origin.
    ///
    /// # Returns
    /// The y-coordinate as type `T`.
    pub fn y(&self) -> T
    where
        T: Copy,
    {
        self.origin.y()
    }

    /// Returns the width of the rectangle.
    ///
    /// # Returns
    /// The width as type `T`.
    pub fn width(&self) -> T
    where
        T: Copy,
    {
        self.size.width()
    }

    /// Returns the height of the rectangle.
    ///
    /// # Returns
    /// The height as type `T`.
    pub fn height(&self) -> T
    where
        T: Copy,
    {
        self.size.height()
    }

    /// Returns the size (width and height) of the rectangle.
    ///
    /// # Returns
    /// The size as a `Size<T>`.
    pub fn size(&self) -> Size<T>
    where
        T: Copy,
    {
        self.size
    }

    /// Returns the origin (top-left corner) of the rectangle.
    ///
    /// # Returns
    /// The origin as a `Point<T>`.
    pub fn origin(&self) -> Point<T>
    where
        T: Copy,
    {
        self.origin
    }

    /// Calculates the area of the rectangle.
    ///
    /// # Returns
    /// The area as type `T`.
    pub fn area(&self) -> T
    where
        T: Copy + std::ops::Mul<Output = T>,
    {
        self.width() * self.height()
    }

    /// Calculates the perimeter of the rectangle.
    ///
    /// # Returns
    /// The perimeter as type `T`.
    pub fn perimeter(&self) -> T
    where
        T: Copy + std::ops::Add<Output = T>,
    {
        (self.width() + self.height()) + (self.width() + self.height())
    }

    /// Returns the center point of the rectangle.
    ///
    /// # Returns
    /// The center as a `Point<T>`.
    pub fn center(&self) -> Point<T>
    where
        T: Copy + std::ops::Add<Output = T> + std::ops::Div<Output = T> + From<u8>,
    {
        let half_width = self.width() / T::from(2u8);
        let half_height = self.height() / T::from(2u8);
        Point::new(self.origin.x() + half_width, self.origin.y() + half_height)
    }

    /// Checks if the rectangle contains a given point.
    ///
    /// # Arguments
    /// * `point` - The point to check.
    ///
    /// # Returns
    /// `true` if the point is inside the rectangle, `false` otherwise.
    pub fn contains(&self, point: Point<T>) -> bool
    where
        T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialOrd,
    {
        let x_min = self.origin.x();
        let y_min = self.origin.y();
        let x_max = x_min + self.width();
        let y_max = y_min + self.height();

        point.x() >= x_min && point.x() < x_max && point.y() >= y_min && point.y() < y_max
    }

    /// Checks if this rectangle intersects with another rectangle.
    ///
    /// # Arguments
    /// * `other` - The other rectangle to check intersection with.
    ///
    /// # Returns
    /// `true` if the rectangles intersect, `false` otherwise.
    pub fn intersects(&self, other: &Rect<T>) -> bool
    where
        T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialOrd,
    {
        let x_min = self.x();
        let y_min = self.y();
        let x_max = x_min + self.width();
        let y_max = y_min + self.height();

        let other_x_min = other.x();
        let other_y_min = other.y();
        let other_x_max = other_x_min + other.width();
        let other_y_max = other_y_min + other.height();

        !(x_max < other_x_min || x_min > other_x_max || y_max < other_y_min || y_min > other_y_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use crate::size::Size;

    fn rect_i32(x: i32, y: i32, w: i32, h: i32) -> Rect<i32> {
        Rect::new(Point::new(x, y), Size::new(w, h))
    }

    #[test]
    fn creates_rect_with_given_origin_and_size() {
        let origin = Point::new(1, 2);
        let size = Size::new(3, 4);
        let rect = Rect::new(origin, size);
        assert_eq!(rect.origin(), origin);
        assert_eq!(rect.size(), size);
    }

    #[test]
    fn width_and_height_return_correct_values() {
        let rect = rect_i32(0, 0, 10, 20);
        assert_eq!(rect.width(), 10);
        assert_eq!(rect.height(), 20);
    }

    #[test]
    fn area_calculates_correctly() {
        let rect = rect_i32(0, 0, 5, 4);
        assert_eq!(rect.area(), 20);
    }

    #[test]
    fn perimeter_calculates_correctly() {
        let rect = rect_i32(0, 0, 3, 7);
        assert_eq!(rect.perimeter(), 20);
    }

    #[test]
    fn center_returns_correct_point_for_even_dimensions() {
        let rect = rect_i32(2, 4, 6, 8);
        assert_eq!(rect.center(), Point::new(5, 8));
    }

    #[test]
    fn center_returns_correct_point_for_odd_dimensions() {
        let rect = rect_i32(1, 1, 5, 3);
        assert_eq!(rect.center(), Point::new(3, 2));
    }

    #[test]
    fn contains_returns_true_for_point_inside() {
        let rect = rect_i32(0, 0, 10, 10);
        assert!(rect.contains(Point::new(5, 5)));
    }

    #[test]
    fn contains_returns_false_for_point_on_right_or_bottom_edge() {
        let rect = rect_i32(0, 0, 10, 10);
        assert!(!rect.contains(Point::new(10, 5)));
        assert!(!rect.contains(Point::new(5, 10)));
    }

    #[test]
    fn contains_returns_true_for_point_on_left_or_top_edge() {
        let rect = rect_i32(0, 0, 10, 10);
        assert!(rect.contains(Point::new(0, 0)));
        assert!(rect.contains(Point::new(0, 5)));
        assert!(rect.contains(Point::new(5, 0)));
    }

    #[test]
    fn contains_returns_false_for_point_outside() {
        let rect = rect_i32(0, 0, 10, 10);
        assert!(!rect.contains(Point::new(-1, 5)));
        assert!(!rect.contains(Point::new(5, -1)));
        assert!(!rect.contains(Point::new(11, 5)));
        assert!(!rect.contains(Point::new(5, 11)));
    }

    #[test]
    fn intersects_returns_true_for_overlapping_rects() {
        let a = rect_i32(0, 0, 10, 10);
        let b = rect_i32(5, 5, 10, 10);
        assert!(a.intersects(&b));
    }

    #[test]
    fn intersects_returns_false_for_non_overlapping_rects() {
        let a = rect_i32(0, 0, 10, 10);
        let b = rect_i32(20, 20, 5, 5);
        assert!(!a.intersects(&b));
    }

    #[test]
    fn intersects_returns_true_for_touching_edges() {
        let a = rect_i32(0, 0, 10, 10);
        let b = rect_i32(10, 0, 5, 10);
        assert!(a.intersects(&b));
    }

    #[test]
    fn intersects_returns_true_for_touching_corners() {
        let a = rect_i32(0, 0, 10, 10);
        let b = rect_i32(10, 10, 5, 5);
        assert!(a.intersects(&b));
    }

    #[test]
    fn area_is_zero_for_zero_width_or_height() {
        let rect = rect_i32(0, 0, 0, 10);
        assert_eq!(rect.area(), 0);
        let rect = rect_i32(0, 0, 10, 0);
        assert_eq!(rect.area(), 0);
    }
}
