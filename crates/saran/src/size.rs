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