use crate::toolbox::{Grid, Position};
use std::sync::LazyLock;

pub struct Pad {
    pad: Grid,
}

impl Pad {
    pub fn new(width: isize, height: isize, buttons: Vec<(Position, char)>) -> Pad {
        let mut pad = Grid::new(width, height);

        for (position, value) in buttons {
            pad.set_element_at(&position, value);
        }

        Pad { pad }
    }

    pub fn is_a_button(&self, position: &Position) -> bool {
        self.pad.is_inbound(position) && self.pad.element_at(position) != &' '
    }

    pub fn value_at(&self, position: &Position) -> char {
        *self.pad.element_at(position)
    }
}

pub static NUMERIC_PAD: LazyLock<Pad> = LazyLock::new(|| {
    Pad::new(
        3,
        4,
        vec![
            (Position::new(0, 0), '7'),
            (Position::new(1, 0), '8'),
            (Position::new(2, 0), '9'),
            (Position::new(0, 1), '4'),
            (Position::new(1, 1), '5'),
            (Position::new(2, 1), '6'),
            (Position::new(0, 2), '1'),
            (Position::new(1, 2), '2'),
            (Position::new(2, 2), '3'),
            (Position::new(0, 3), ' '),
            (Position::new(1, 3), '0'),
            (Position::new(2, 3), 'A'),
        ],
    )
});

pub static DIRECTIONAL_PAD: LazyLock<Pad> = LazyLock::new(|| {
    Pad::new(3, 2, vec![(Position::new(0, 0), ' '), (Position::new(1, 0), '^'), (Position::new(2, 0), 'A'), (Position::new(0, 1), '<'), (Position::new(1, 1), 'v'), (Position::new(2, 1), '>')])
});
