//! Point module for generic 2D point operations.
//!
//! This module defines a generic `Point<T>` struct representing a 2D point with x and y coordinates,
//! along with a variety of utility methods for mapping, zipping, and operating on points. It also
//! provides operator overloads for `Point<String>` to support string-specific arithmetic.

/// A generic 2D point type with x and y coordinates.
///
/// # Type Parameters
/// - `T`: The type of the coordinates.
///
/// # Examples
/// ```
/// use crate::saran::point::Point;
/// let p = Point::new(1, 2);
/// assert_eq!(p.x(), 1);
/// assert_eq!(p.y(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    /// The x coordinate.
    x: T,
    /// The y coordinate.
    y: T,
}

impl<T> Point<T> {
    /// Creates a new `Point` with the given x and y coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Returns a reference to the x coordinate.
    pub fn x(&self) -> T where T: Copy{
        self.x
    }

    /// Returns a reference to the y coordinate.
    pub fn y(&self) -> T where T: Copy{
        self.y
    }

    /// Converts the point into a tuple `(x, y)`.
    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    /// Creates a `Point` from a tuple `(x, y)`.
    pub fn from_tuple(tuple: (T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1 }
    }

    /// Calculates the Euclidean distance from this point to another point.
    ///
    /// # Type Parameters
    /// - `U`: A type that can be converted into a `Point<T>`.
    ///
    /// # Arguments
    /// * `other` - The other point, which can be converted into a `Point<T>`.
    ///
    /// # Returns
    /// The Euclidean distance between `self` and `other` as an `f64`.
    ///
    /// # Requirements
    /// - `T` must implement `Into<f64>` and `Copy`.
    ///
    /// # Example
    /// ```
    /// use crate::saran::point::Point;
    /// let p1 = Point::new(0.0, 0.0);
    /// let p2 = Point::new(3.0, 4.0);
    /// let dist = p1.distance_from(p2);
    /// assert_eq!(dist, 5.0);
    /// ```
    pub fn distance_from<U: Into<Point<T>>>(self, other: U) -> f64
    where
        T: Into<f64> + Copy,
    {
        let other_point = other.into();
        let dx = self.x.into() - other_point.x.into();
        let dy = self.y.into() - other_point.y.into();
        (dx * dx + dy * dy).sqrt()
    }

    /// Applies a function to both coordinates, consuming the point.
    ///
    /// # Arguments
    /// * `f` - Function to apply to each coordinate.
    pub fn map<U, F>(self, f: F) -> Point<U>
    where
        F: Fn(T) -> U,
    {
        Point { x: f(self.x), y: f(self.y) }
    }

    /// Applies a function to references of both coordinates.
    ///
    /// # Arguments
    /// * `f` - Function to apply to each coordinate reference.
    pub fn map_ref<U, F>(&self, f: F) -> Point<U>
    where
        F: Fn(&T) -> U,
    {
        Point { x: f(&self.x), y: f(&self.y) }
    }

    /// Applies a mutable function to both coordinates, consuming the point.
    ///
    /// # Arguments
    /// * `f` - Mutable function to apply to each coordinate.
    pub fn map_mut<U, F>(self, mut f: F) -> Point<U>
    where
        F: FnMut(T) -> U,
    {
        Point { x: f(self.x), y: f(self.y) }
    }

    /// Applies a mutable function to mutable references of both coordinates.
    ///
    /// # Arguments
    /// * `f` - Mutable function to apply to each coordinate reference.
    pub fn map_mut_ref<U, F>(&mut self, mut f: F) -> Point<U>
    where
        F: FnMut(&mut T) -> U,
    {
        Point { x: f(&mut self.x), y: f(&mut self.y) }
    }

    /// Combines two points by applying a function to each pair of coordinates, consuming both.
    ///
    /// # Arguments
    /// * `other` - The other point.
    /// * `f` - Function to apply to each pair of coordinates.
    pub fn zip<U, V, F>(self, other: Point<U>, f: F) -> Point<V>
    where
        F: Fn(T, U) -> V,
    {
        Point { x: f(self.x, other.x), y: f(self.y, other.y) }
    }

    /// Combines two points by applying a function to references of each pair of coordinates.
    ///
    /// # Arguments
    /// * `other` - Reference to the other point.
    /// * `f` - Function to apply to each pair of coordinate references.
    pub fn zip_ref<U, V, F>(&self, other: &Point<U>, f: F) -> Point<V>
    where
        F: Fn(&T, &U) -> V,
    {
        Point { x: f(&self.x, &other.x), y: f(&self.y, &other.y) }
    }

    /// Combines two points by applying a mutable function to each pair of coordinates, consuming both.
    ///
    /// # Arguments
    /// * `other` - The other point.
    /// * `f` - Mutable function to apply to each pair of coordinates.
    pub fn zip_mut<U, V, F>(self, other: Point<U>, mut f: F) -> Point<V>
    where
        F: FnMut(T, U) -> V,
    {
        Point { x: f(self.x, other.x), y: f(self.y, other.y) }
    }

    /// Combines two points by applying a mutable function to a mutable reference and a reference of each coordinate.
    ///
    /// # Arguments
    /// * `other` - Reference to the other point.
    /// * `f` - Mutable function to apply to each pair of coordinate references.
    pub fn zip_mut_ref<U, V, F>(&mut self, other: &Point<U>, mut f: F) -> Point<V>
    where
        F: FnMut(&mut T, &U) -> V,
    {
        Point { x: f(&mut self.x, &other.x), y: f(&mut self.y, &other.y) }
    }

    /// Applies a function to the x coordinate, cloning y.
    ///
    /// # Arguments
    /// * `f` - Function to apply to x.
    pub fn map_x<U, F>(self, f: F) -> Point<U>
    where
        F: Fn(T) -> U,
        T: Clone,
        U: Clone,
    {
        Point { x: f(self.x), y: f(self.y.clone()) }
    }

    /// Applies a function to the y coordinate, cloning x.
    ///
    /// # Arguments
    /// * `f` - Function to apply to y.
    pub fn map_y<U, F>(self, f: F) -> Point<U>
    where
        F: Fn(T) -> U,
        T: Clone,
        U: Clone,
    {
        Point { x: f(self.x.clone()), y: f(self.y) }
    }

    /// Applies a function to a reference of the x coordinate.
    ///
    /// # Arguments
    /// * `f` - Function to apply to reference of x.
    pub fn map_x_ref<U, F>(&self, f: F) -> Point<U>
    where
        F: Fn(&T) -> U,
        T: Clone,
        U: Clone,
    {
        Point { x: f(&self.x), y: f(&self.y) }
    }

    /// Applies a function to a reference of the y coordinate.
    ///
    /// # Arguments
    /// * `f` - Function to apply to reference of y.
    pub fn map_y_ref<U, F>(&self, f: F) -> Point<U>
    where
        F: Fn(&T) -> U,
        T: Clone,
        U: Clone,
    {
        Point { x: f(&self.x), y: f(&self.y) }
    }

    /// Applies a mutable function to the x coordinate, cloning y.
    ///
    /// # Arguments
    /// * `f` - Mutable function to apply to x.
    pub fn map_x_mut<U, F>(self, mut f: F) -> Point<U>
    where
        F: FnMut(T) -> U,
        T: Clone,
        U: Clone,
    {
        Point { x: f(self.x), y: f(self.y.clone()) }
    }

    /// Applies a mutable function to the y coordinate, cloning x.
    ///
    /// # Arguments
    /// * `f` - Mutable function to apply to y.
    pub fn map_y_mut<U, F>(self, mut f: F) -> Point<U>
    where
        F: FnMut(T) -> U,
        T: Clone,
        U: Clone,
    {
        Point { x: f(self.x.clone()), y: f(self.y) }
    }

    /// Applies a mutable function to a mutable reference of the x coordinate.
    ///
    /// # Arguments
    /// * `f` - Mutable function to apply to mutable reference of x.
    pub fn map_x_mut_ref<U, F>(&mut self, mut f: F) -> Point<U>
    where
        F: FnMut(&mut T) -> U,
        T: Clone,
        U: Clone,
    {
        Point { x: f(&mut self.x), y: f(&mut self.y) }
    }

    /// Applies a mutable function to a mutable reference of the y coordinate.
    ///
    /// # Arguments
    /// * `f` - Mutable function to apply to mutable reference of y.
    pub fn map_y_mut_ref<U, F>(&mut self, mut f: F) -> Point<U>
    where
        F: FnMut(&mut T) -> U,
        T: Clone,
        U: Clone,
    {
        Point { x: f(&mut self.x), y: f(&mut self.y) }
    }
}

/// Implements `Display` for `Point<T>` where `T: Display`.
impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    /// Formats the point as `(x, y)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// --- String-specific operator overloads for Point<String> ---

/// Adds two `Point<String>` by concatenating their x and y coordinates.
impl std::ops::Add for Point<String> {
    type Output = Point<String>;
    /// Concatenates the x and y coordinates of two points.
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: format!("{}{}", self.x, other.x),
            y: format!("{}{}", self.y, other.y),
        }
    }
}

/// Adds and assigns to a `Point<String>` by concatenating x and y coordinates.
impl std::ops::AddAssign for Point<String> {
    /// Concatenates and assigns the x and y coordinates of another point.
    fn add_assign(&mut self, other: Self) {
        self.x = format!("{}{}", self.x, other.x);
        self.y = format!("{}{}", self.y, other.y);
    }
}

/// Multiplies a `Point<String>` by a scalar, repeating both coordinates.
impl std::ops::Mul<usize> for Point<String> {
    type Output = Point<String>;
    /// Repeats the x and y coordinates by the given scalar.
    fn mul(self, scalar: usize) -> Self::Output {
        Point {
            x: self.x.repeat(scalar),
            y: self.y.repeat(scalar),
        }
    }
}

/// Multiplies and assigns to a `Point<String>` by a scalar.
impl std::ops::MulAssign<usize> for Point<String> {
    /// Repeats and assigns the x and y coordinates by the given scalar.
    fn mul_assign(&mut self, scalar: usize) {
        self.x = self.x.repeat(scalar);
        self.y = self.y.repeat(scalar);
    }
}

/// Negates a `Point<String>` by concatenating its coordinates.
impl std::ops::Neg for Point<String> {
    type Output = String;
    /// Concatenates the x and y coordinates into a single string.
    fn neg(self) -> Self::Output {
        format!("{}{}", self.x, self.y)
    }
}

// impl from for tuple
impl<T> From<(T, T)> for Point<T> {
    /// Converts a tuple `(x, y)` into a `Point<T>`.
    fn from(tuple: (T, T)) -> Self {
        Point::from_tuple(tuple)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_point_and_accesses_coordinates() {
        let p = Point::new(3, 4);
        assert_eq!(p.x(), 3);
        assert_eq!(p.y(), 4);
        assert_eq!(p.into_tuple(), (3, 4));
    }

    #[test]
    fn creates_point_from_tuple() {
        let p = Point::from_tuple((5, 6));
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 6);
    }

    #[test]
    fn maps_coordinates_with_map() {
        let p = Point::new(2, 3);
        let p2 = p.map(|v| v * 2);
        assert_eq!(p2, Point::new(4, 6));
    }

    #[test]
    fn maps_coordinates_with_map_ref() {
        let p = Point::new(2, 3);
        let p2 = p.map_ref(|v| v + 1);
        assert_eq!(p2, Point::new(3, 4));
    }

    #[test]
    fn maps_coordinates_with_map_mut() {
        let p = Point::new(2, 3);
        let p2 = p.map_mut(|v| v * 3);
        assert_eq!(p2, Point::new(6, 9));
    }

    #[test]
    fn maps_coordinates_with_map_mut_ref() {
        let mut p = Point::new(2, 3);
        let p2 = p.map_mut_ref(|v| {
            *v += 1;
            *v
        });
        assert_eq!(p2, Point::new(3, 4));
    }

    #[test]
    fn zips_two_points_with_zip() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = p1.zip(p2, |a, b| a + b);
        assert_eq!(p3, Point::new(4, 6));
    }

    #[test]
    fn zips_two_points_with_zip_ref() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = p1.zip_ref(&p2, |a, b| a * b);
        assert_eq!(p3, Point::new(3, 8));
    }

    #[test]
    fn zips_two_points_with_zip_mut() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = p1.zip_mut(p2, |a, b| a - b);
        assert_eq!(p3, Point::new(-2, -2));
    }

    #[test]
    fn zips_two_points_with_zip_mut_ref() {
        let mut p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = p1.zip_mut_ref(&p2, |a, b| {
            *a += *b;
            *a
        });
        assert_eq!(p3, Point::new(4, 6));
    }

    #[test]
    fn map_x_and_map_y_apply_function_to_single_coordinate() {
        let p = Point::new(2, 3);
        let px = p.clone().map_x(|v| v * 10);
        let py = p.clone().map_y(|v| v * 100);
        assert_eq!(px, Point::new(20, 30)); // Both x and y multiplied by 10
        assert_eq!(py, Point::new(200, 300)); // Both x and y multiplied by 100
    }

    #[test]
    fn map_x_ref_and_map_y_ref_apply_function_to_single_coordinate() {
        let p = Point::new(2, 3);
        let px = p.map_x_ref(|v| v + 1);
        let py = p.map_y_ref(|v| v + 2);
        assert_eq!(px, Point::new(3, 4)); // Both x and y incremented by 1
        assert_eq!(py, Point::new(4, 5)); // Both x and y incremented by 2
    }

    #[test]
    fn map_x_mut_and_map_y_mut_apply_mutable_function() {
        let p = Point::new(2, 3);
        let px = p.clone().map_x_mut(|v| v * 2);
        let py = p.map_y_mut(|v| v * 3);
        assert_eq!(px, Point::new(4, 6)); // Both x and y multiplied by 2
        assert_eq!(py, Point::new(6, 9)); // Both x and y multiplied by 3
    }

    #[test]
    fn map_x_mut_ref_and_map_y_mut_ref_apply_mutable_function() {
        let mut p = Point::new(2, 3);
        let px = p.clone().map_x_mut_ref(|v| {
            *v += 1;
            *v
        });
        let mut p2 = Point::new(2, 3);
        let py = p2.map_y_mut_ref(|v| {
            *v *= 2;
            *v
        });
        assert_eq!(px, Point::new(3, 4));
        assert_eq!(py, Point::new(4, 6));
    }

    #[test]
    fn display_formats_point_correctly() {
        let p = Point::new(7, 8);
        assert_eq!(format!("{}", p), "(7, 8)");
    }

    #[test]
    fn add_points_of_string_concatenates_coordinates() {
        let p1 = Point::new("a".to_string(), "b".to_string());
        let p2 = Point::new("c".to_string(), "d".to_string());
        let p3 = p1 + p2;
        assert_eq!(p3, Point::new("ac".to_string(), "bd".to_string()));
    }

    #[test]
    fn add_assign_points_of_string_concatenates_and_assigns() {
        let mut p1 = Point::new("x".to_string(), "y".to_string());
        let p2 = Point::new("z".to_string(), "w".to_string());
        p1 += p2;
        assert_eq!(p1, Point::new("xz".to_string(), "yw".to_string()));
    }

    #[test]
    fn mul_point_string_repeats_coordinates() {
        let p = Point::new("hi".to_string(), "ok".to_string());
        let p2 = p * 3;
        assert_eq!(p2, Point::new("hihihi".to_string(), "okokok".to_string()));
    }

    #[test]
    fn mul_assign_point_string_repeats_and_assigns() {
        let mut p = Point::new("a".to_string(), "b".to_string());
        p *= 2;
        assert_eq!(p, Point::new("aa".to_string(), "bb".to_string()));
    }

    #[test]
    fn neg_point_string_concatenates_coordinates() {
        let p = Point::new("foo".to_string(), "bar".to_string());
        let s = -p;
        assert_eq!(s, "foobar".to_string());
    }

    #[test]
    fn edge_case_empty_strings_in_point_string_operations() {
        let p1 = Point::new("".to_string(), "".to_string());
        let p2 = Point::new("x".to_string(), "y".to_string());
        assert_eq!(p1.clone() + p2.clone(), Point::new("x".to_string(), "y".to_string()));
        assert_eq!(p2.clone() + p1.clone(), Point::new("x".to_string(), "y".to_string()));
        assert_eq!(p1.clone() * 0, Point::new("".to_string(), "".to_string()));
        assert_eq!(-p1, "".to_string());
    }

    #[test]
    fn edge_case_zero_and_negative_numbers() {
        let p = Point::new(0i32, -1i32);
        let p2 = p.map(|v| v.abs());
        assert_eq!(p2, Point::new(0, 1));
    }

    #[test]
    fn distance_from_returns_zero_for_same_point() {
        let p = Point::new(1.0, 2.0);
        let dist = p.distance_from(Point::new(1.0, 2.0));
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn distance_from_works_for_positive_coordinates() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        let dist = p1.distance_from(p2);
        assert_eq!(dist, 5.0);
    }

    #[test]
    fn distance_from_works_for_negative_coordinates() {
        let p1 = Point::new(-1.0, -2.0);
        let p2 = Point::new(-4.0, -6.0);
        let dist = p1.distance_from(p2);
        assert!((dist - 5.0).abs() < 1e-10);
    }

    #[test]
    fn distance_from_works_for_mixed_sign_coordinates() {
        let p1 = Point::new(-1.0, 2.0);
        let p2 = Point::new(3.0, -2.0);
        let dist = p1.distance_from(p2);
        assert!((dist - 5.656854249).abs() < 1e-6);
    }

    #[test]
    fn distance_from_accepts_tuple_as_other() {
        let p = Point::new(1.0, 1.0);
        let dist = p.distance_from((4.0, 5.0));
        assert_eq!(dist, 5.0);
    }

    #[test]
    fn distance_from_works_with_integer_coordinates() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(6, 8);
        let dist = p1.distance_from(p2);
        assert_eq!(dist, 10.0);
    }
}