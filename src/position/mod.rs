pub mod iterator;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Position {
        Position { x: x, y: y }
    }

    pub fn origin() -> Position {
        Position::new(0,0)
    }

    pub fn adjust(&self, x: u16, y: u16) -> Position {
        Position::new(self.x + x, self.y + y)
    }

    pub fn bounded_by(&self, bound: usize) -> bool {
        (self.x < bound as u16) && (self.y < bound as u16)
    }
}

#[test]
fn position_adjust() {
    assert!(Position::origin().adjust(1,2) == Position::new(1,2))
}

#[test]
fn position_bounded_by() {
    assert!(Position::origin().bounded_by(1));
    assert!(!Position::new(3,3).bounded_by(1));
    assert!(Position::new(3,3).bounded_by(4));
}
