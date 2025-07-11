/// A generic 2D size with width and height.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size<T> {
    /// The width component.
    pub width: T,
    /// The height component.
    pub height: T,
}

impl<T> Size<T> {
    /// Creates a new `Size` from width and height.
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    /// Returns `true` if both width and height are the default value.
    pub fn is_empty(&self) -> bool
    where
        T: PartialEq + Default,
    {
        self.width == T::default() && self.height == T::default()
    }

    /// Returns the area (width \* height).
    pub fn area(&self) -> T
    where
        T: std::ops::Mul<Output = T> + Copy,
    {
        self.width * self.height
    }

    /// Returns the aspect ratio (width / height) as `f64`, or `None` if height is zero or empty.
    ///
    /// Requires `T: Into<f64> + Copy + PartialEq + Default`.
    pub fn aspect_ratio(&self) -> Option<f64>
    where
        T: Into<f64> + Copy + PartialEq + Default,
    {
        if self.is_empty() || self.height == T::default() {
            None
        } else {
            let height_f64 = self.height.into();
            if height_f64 == 0.0 {
                None
            } else {
                Some(self.width.into() / height_f64)
            }
        }
    }

    /// Converts the size to a tuple (width, height).
    pub fn to_tuple(self) -> (T, T) {
        (self.width, self.height)
    }

    /// Creates a size from a tuple (width, height).
    pub fn from_tuple(tuple: (T, T)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }

    /// Maps both width and height using the provided function.
    pub fn map<U, F>(self, f: F) -> Size<U>
    where
        F: Fn(T) -> U,
    {
        Size {
            width: f(self.width),
            height: f(self.height),
        }
    }

    /// Maps width and height pairwise using the provided function.
    ///
    /// The closure is called as `f(width, height)` for width and `f(height, width)` for height.
    pub fn map_pairwise<U, F>(self, f: F) -> Size<U>
    where
        T: Copy,
        F: Fn(T, T) -> U,
    {
        Size {
            width: f(self.width, self.height),
            height: f(self.height, self.width),
        }
    }

    /// Zips two sizes into a size of tuples.
    pub fn zip<U>(self, other: Size<U>) -> Size<(T, U)> {
        Size {
            width: (self.width, other.width),
            height: (self.height, other.height),
        }
    }

    /// Zips two sizes using a closure.
    ///
    /// The closure is called for width and height separately.
    pub fn zip_with<U, F>(self, other: Size<U>, f: F) -> Size<U>
    where
        F: Fn(T, U) -> U,
    {
        Size {
            width: f(self.width, other.width),
            height: f(self.height, other.height),
        }
    }
}

impl<T: Default> Default for Size<T> {
    fn default() -> Self {
        Size {
            width: T::default(),
            height: T::default(),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Size<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size({}, {})", self.width, self.height)
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Size<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Size {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Size<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Size {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Size<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Size {
            width: self.width * scalar,
            height: self.height * scalar,
        }
    }
}

impl<T: std::ops::Div<Output = T> + Copy + PartialEq + Default> std::ops::Div<T> for Size<T> {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        if scalar == T::default() {
            panic!("Division by zero in Size::div");
        }
        Size {
            width: self.width / scalar,
            height: self.height / scalar,
        }
    }
}

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Size<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Size {
            width: -self.width,
            height: -self.height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_size_with_given_dimensions() {
        let s = Size::new(3, 4);
        assert_eq!(s.width, 3);
        assert_eq!(s.height, 4);
    }

    #[test]
    fn is_empty_returns_true_for_default_values() {
        let s: Size<i32> = Size::default();
        assert!(s.is_empty());
    }

    #[test]
    fn is_empty_returns_false_for_non_default_values() {
        let s = Size::new(1, 0);
        assert!(!s.is_empty());
    }

    #[test]
    fn area_returns_product_of_width_and_height() {
        let s = Size::new(3, 4);
        assert_eq!(s.area(), 12);
    }

    #[test]
    fn aspect_ratio_returns_none_for_empty_size() {
        let s: Size<f64> = Size::default();
        assert_eq!(s.aspect_ratio(), None);
    }

    #[test]
    fn aspect_ratio_returns_none_for_zero_height() {
        let s = Size::new(5.0, 0.0);
        assert_eq!(s.aspect_ratio(), None);
    }

    #[test]
    fn aspect_ratio_returns_some_for_valid_size() {
        let s = Size::new(8.0, 2.0);
        assert_eq!(s.aspect_ratio(), Some(4.0));
    }

    #[test]
    fn to_tuple_and_from_tuple_are_inverse() {
        let s = Size::new(7, 9);
        let tuple = s.to_tuple();
        let s2 = Size::from_tuple(tuple);
        assert_eq!(s, s2);
    }

    #[test]
    fn map_applies_function_to_both_dimensions() {
        let s = Size::new(2, 3);
        let mapped = s.map(|x| x * 2);
        assert_eq!(mapped, Size::new(4, 6));
    }

    #[test]
    fn map_pairwise_applies_function_pairwise() {
        let s = Size::new(2, 3);
        let mapped = s.map_pairwise(|a, b| a + b);
        assert_eq!(mapped, Size::new(5, 5));
    }

    #[test]
    fn zip_combines_two_sizes_into_tuples() {
        let a = Size::new(1, 2);
        let b = Size::new(3, 4);
        let zipped = a.zip(b);
        assert_eq!(zipped, Size::new((1, 3), (2, 4)));
    }

    #[test]
    fn zip_with_combines_two_sizes_with_function() {
        let a = Size::new(1, 2);
        let b = Size::new(3, 4);
        let zipped = a.zip_with(b, |x, y| x + y);
        assert_eq!(zipped, Size::new(4, 6));
    }

    #[test]
    fn default_returns_size_with_default_components() {
        let s: Size<u8> = Size::default();
        assert_eq!(s, Size::new(0, 0));
    }

    #[test]
    fn display_formats_size_correctly() {
        let s = Size::new(5, 6);
        assert_eq!(format!("{}", s), "Size(5, 6)");
    }

    #[test]
    fn add_adds_corresponding_components() {
        let a = Size::new(1, 2);
        let b = Size::new(3, 4);
        assert_eq!(a + b, Size::new(4, 6));
    }

    #[test]
    fn sub_subtracts_corresponding_components() {
        let a = Size::new(5, 7);
        let b = Size::new(2, 3);
        assert_eq!(a - b, Size::new(3, 4));
    }

    #[test]
    fn mul_multiplies_both_components_by_scalar() {
        let s = Size::new(2, 3);
        assert_eq!(s * 4, Size::new(8, 12));
    }

    #[test]
    #[should_panic(expected = "Division by zero in Size::div")]
    fn div_panics_on_zero_scalar() {
        let s = Size::new(2, 3);
        let _ = s / 0;
    }

    #[test]
    fn div_divides_both_components_by_scalar() {
        let s = Size::new(8, 12);
        assert_eq!(s / 4, Size::new(2, 3));
    }

    #[test]
    fn neg_negates_both_components() {
        let s = Size::new(5, -7);
        assert_eq!(-s, Size::new(-5, 7));
    }
}