pub mod context;
pub mod event;
pub mod key;
pub mod layout;
pub mod point;
pub mod rect;
pub mod size;
pub mod style;
pub mod theme;
pub mod widget;
pub mod window;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
