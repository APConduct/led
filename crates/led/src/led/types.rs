use serde::{Deserialize, Serialize};

pub mod buffer {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub struct ID(uuid::Uuid);
    impl ID {
        pub fn new() -> Self {
            Self(uuid::Uuid::new_v4())
        }
    }
}

pub mod source {
    #[derive(Debug, Clone, Copy)]
    pub enum ID {
        Original,
        Add,
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

pub mod cursor {
    use crate::led::types::{Position, Range};

    #[derive(Debug, Clone, Copy)]
    pub struct State {
        pub pos: Position,
        pub selection: Option<Range>,
        pub buffer_id: super::buffer::ID,
    }
}
