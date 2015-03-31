use position::*;
use position::iterator::*;

struct RankIterator {
    current_position: Position,
    size: usize,
}

impl RankIterator {
    fn new(source_pos: &Position, size: usize) -> RankIterator {
        RankIterator {
            current_position: Position::new(0, (*source_pos).y),
            size: size
        }
    }
}

impl Iterator for RankIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Position> {
        PositionIteratorImpl::next(self)
    }
}

impl PositionIterator for RankIterator {
    fn current_position(&self) -> Position { self.current_position }

    fn adjust(&mut self) {
        // (0,1) is the unit rank vector
        self.current_position = self.current_position.adjust(1,0)
    }

    fn size(&self) -> usize { self.size }
}


#[test]
fn rank_iter_creation() {
    let rank_iter = RankIterator::new(&Position::new(1,2), 8);
    assert!(rank_iter.current_position.x == 0);
    assert!(rank_iter.current_position.y == 2);
}

#[test]
fn rank_iterator() {
    let mut rank_iter = RankIterator::new(&Position::new(1,2), 3);
    assert!(rank_iter.next().unwrap() == Position::new(0,2));
    assert!(rank_iter.next().unwrap() == Position::new(1,2));
    assert!(rank_iter.next().unwrap() == Position::new(2,2));
    assert!(rank_iter.next() == None);
}

