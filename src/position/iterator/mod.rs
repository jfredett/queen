pub mod column;
pub mod rank;
pub mod diagonal;

use position::*;

pub trait PositionIterator {
    fn current_position(&self) -> Position;
    fn adjust(&mut self);
    fn size(&self) -> usize;
    fn copy_position(&self) -> Position {
        self.current_position().clone()
    }

    fn bounded_by(&self, size: usize) -> bool {
        self.current_position().bounded_by(size)
    }
}

pub struct PositionIteratorImpl;
impl PositionIteratorImpl {
    fn next(iter: &mut PositionIterator) -> Option<Position> {
        if ! iter.bounded_by(iter.size()) { return None; }

        let ret = Some(iter.copy_position());

        iter.adjust();

        return ret
    }
}
