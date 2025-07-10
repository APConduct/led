use crate::led::types::{Position, Range};

pub struct State {
    pub(crate) position: Position,
    pub(crate) selection: Option<Range>,
    pub(crate) buffer_id: super::buffer::ID,
}