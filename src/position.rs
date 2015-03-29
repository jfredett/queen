pub struct Position {
  x: u16,
  y: u16,
}


enum PositionEnumClass {
  Column,
  Rank,
  Diagonal,
}

struct PositionIterator {
  class: PositionEnumClass,
  size: usize,
}

impl Position {
  fn new(x: u16, y: u16) {
    Position { x: x, y: y }
  }


// position iterator struct, given <size>, yield iterator of all positions
// wihtin that size
  fn column(&self) -> Fn<PositionIterator> {
    |s| { PositionIterator { class: PositionEnumClass::Column, size: s }.iter() }
  }

  fn rank(&self) -> Fn<PositionIterator> {
    |s| { PositionIterator { class: PositionEnumClass::Rank, size: s }.iter() }
  }

  fn diagonals(&self) -> Fn<PositionIterator> {
    |s| { PositionIterator { class: PositionEnumClass::Diagonal, size: s }.iter() }
  }
}



impl Iterator for PositionIterator {
  type Item = Position;

  fn next(&self) -> Option<Position> {

  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, self.size)
  }

}

#[test]
fn test() {
  Position::new(1,2);
}

#[test]
fn test2() {
  PositionIterator { size: 1, class: PositionEnumClass::Column }.iter();
}
