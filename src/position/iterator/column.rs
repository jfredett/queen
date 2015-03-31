use position::*;
use position::iterator::*;


struct ColumnIterator {
    current_position: Position,
    size: usize,
}

impl ColumnIterator {
    fn new(source_pos: &Position, size: usize) -> ColumnIterator {
        ColumnIterator {
            current_position: Position::new((*source_pos).x, 0),
            size: size
        }
    }

    fn iterator_vector() -> (u16, u16) { (0,1) }
}

impl Iterator for ColumnIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Position> {
        PositionIteratorImpl::next(self)
    }
}

impl PositionIterator for ColumnIterator {
    fn current_position(&self) -> Position { self.current_position }

    fn adjust(&mut self) {
        // (0,1) is the unit column vector
        self.current_position = self.current_position.adjust(0,1)
    }

    fn size(&self) -> usize { self.size }
}

#[test]
fn column_iter_creation() {
    let col_iter = ColumnIterator::new(&Position::new(1,2), 8);
    assert!(col_iter.current_position.x == 1);
    assert!(col_iter.current_position.y == 0);
}

#[test]
fn column_iterator() {
    let mut col_iter = ColumnIterator::new(&Position::new(1,2), 3);
    assert!(col_iter.next().unwrap() == Position::new(1,0));
    assert!(col_iter.next().unwrap() == Position::new(1,1));
    assert!(col_iter.next().unwrap() == Position::new(1,2));
    assert!(col_iter.next() == None);
}

