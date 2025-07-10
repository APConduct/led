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
/// assert_eq!(p.x, 1);
/// assert_eq!(p.y, 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    /// The x coordinate.
    pub x: T,
    /// The y coordinate.
    pub y: T,
}

impl<T> Point<T> {
    /// Creates a new `Point` with the given x and y coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Returns a reference to the x coordinate.
    pub fn x(&self) -> &T {
        &self.x
    }

    /// Returns a reference to the y coordinate.
    pub fn y(&self) -> &T {
        &self.y
    }

    /// Converts the point into a tuple `(x, y)`.
    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    /// Creates a `Point` from a tuple `(x, y)`.
    pub fn from_tuple(tuple: (T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1 }
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