use crate::point::Point;
use crate::size::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect<T> {
    pub origin: Point<T>,
    pub size: Size<T>,
}

impl<T> Rect<T> {
    pub fn new(origin: Point<T>, size: Size<T>) -> Self {
        Self { origin, size }
    }
}
