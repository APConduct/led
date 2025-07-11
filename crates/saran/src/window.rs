///! This module contains window management functionality, including the definition of an ID type used for unique identification of windows.

/// A unique identifier for windows.
///
/// The `ID` struct wraps a `u32` value and provides methods for creation,
/// conversion, and incrementing the identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ID(pub u32);

impl ID {
    /// Creates a new `ID` with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value for the ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use saran::window::ID;
    /// let id = ID::new(42);
    /// assert_eq!(id.value(), 42);
    /// ```
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the underlying value of the `ID`.
    ///
    /// # Examples
    ///
    /// ```
    /// use saran::window::ID;
    /// let id = ID::new(7);
    /// assert_eq!(id.value(), 7);
    /// ```
    pub fn value(self) -> u32 {
        self.0
    }

    /// Converts the `ID` to a string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use saran::window::ID;
    /// let id = ID::new(123);
    /// assert_eq!(id.to_string(), "123");
    /// ```
    pub fn to_string(self) -> String {
        self.0.to_string()
    }

    /// Attempts to create an `ID` from a string.
    ///
    /// Returns `Some(ID)` if the string can be parsed as a `u32`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use saran::window::ID;
    /// assert_eq!(ID::from_string("42"), Some(ID(42)));
    /// assert_eq!(ID::from_string("abc"), None);
    /// ```
    pub fn from_string(s: &str) -> Option<Self> {
        s.parse::<u32>().ok().map(Self)
    }

    /// Returns a new unique `ID` by incrementing the current value.
    ///
    /// The current `ID` is incremented in place, and the new value is returned as a new `ID`.
    ///
    /// # Examples
    ///
    /// ```
    /// use saran::window::ID;
    /// let mut id = ID::new(1);
    /// let next_id = id.next();
    /// assert_eq!(id.value(), 2);
    /// assert_eq!(next_id.value(), 2);
    /// ```
    pub fn next(&mut self) -> Self {
        self.0 = self.0.wrapping_add(1);
        Self(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_id_with_given_value() {
        let id = ID::new(10);
        assert_eq!(id.value(), 10);
    }

    #[test]
    fn value_returns_inner_value() {
        let id = ID(99);
        assert_eq!(id.value(), 99);
    }

    #[test]
    fn to_string_returns_string_representation() {
        let id = ID(1234);
        assert_eq!(id.to_string(), "1234");
    }

    #[test]
    fn from_string_parses_valid_string() {
        assert_eq!(ID::from_string("56"), Some(ID(56)));
    }

    #[test]
    fn from_string_returns_none_for_invalid_string() {
        assert_eq!(ID::from_string("notanumber"), None);
        assert_eq!(ID::from_string(""), None);
        assert_eq!(ID::from_string("-1"), None);
    }

    #[test]
    fn next_increments_id_and_returns_new_value() {
        let mut id = ID::new(7);
        let next = id.next();
        assert_eq!(id.value(), 8);
        assert_eq!(next.value(), 8);
    }

    #[test]
    fn next_on_max_value_wraps_to_zero() {
        let mut id = ID::new(u32::MAX);
        let next = id.next();
        assert_eq!(id.value(), 0);
        assert_eq!(next.value(), 0);
    }

    #[test]
    fn ids_with_same_value_are_equal() {
        assert_eq!(ID(42), ID(42));
    }

    #[test]
    fn ids_with_different_values_are_not_equal() {
        assert_ne!(ID(1), ID(2));
    }
}