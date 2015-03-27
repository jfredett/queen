#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Queen {
  x: i32,
  y: i32,
}

impl Queen {
  pub fn new(x: i32, y: i32) -> Queen {
    Queen { x: x, y: y }
  }

  pub fn x(&self) -> i32 {
    self.x
  }

  pub fn y(&self) -> i32 {
    self.y
  }

  pub fn collides(&self, other: &Queen) -> bool {
    self._collides_vertically(other) ||
      self._collides_horizontally(other) ||
      self._collides_diagonally(other)
  }

  /*
   * two positions collide vertically iff there exists `r` such that:
   *    (x,y) + r(1,0) = (x', y')
   * this is equivalent to:
   *    y == y'
   *    x == x'
   */
  fn _collides_vertically(&self, other: &Queen) -> bool {
    self.x() == other.x()
  }

  /*
   * two positions collide horizontally iff there exists `r` such that:
   *    (x,y) + r(0,1) = (x', y')
   * and
   *    y == y'
   */
  fn _collides_horizontally(&self, other: &Queen) -> bool {
    self.y() == other.y()
  }

  /*
   * two positions collide diagonally iff there exists `r` such that
   *    (x,y) + r(1,1) = (x', y')
   * or
   *    (x,y) + r(-1,1) = (x', y')
   * this is equivalent to:
   *    (x' - x) == (y' - y)
   * and
   *   -(x' - x) == (y' - y)
   */
  fn _collides_diagonally(&self, other: &Queen) -> bool {
    let x0x1 = (self.x() - other.x());
    let y0y1 = (self.y() - other.y());

    (x0x1 == y0y1) || (-x0x1 == y0y1)
  }
}

#[test]
fn test_x() {
  assert!(Queen::new(1,2).x() == 1)
}

#[test]
fn test_y() {
  assert!(Queen::new(1,2).y() == 2)
}

#[test]
fn test_diagonal_collides() {
  //   0 1 2 3
  // 0 x . x .
  // 1 . x . .
  // 2 . . . x
  // 3 . . . .
  let q1 = Queen::new(0,0);
  let q2 = Queen::new(1,1);
  let q3 = Queen::new(2,0);

  // positive cases
  assert!(q1._collides_diagonally(&q2));
  assert!(q2._collides_diagonally(&q3));

  // negative case
  let q4 = Queen::new(3,2);
  assert!(!q1._collides_diagonally(&q4));
}

#[test]
fn test_horizontal_collides() {
  //   0 1 2 3
  // 0 x . . x
  // 1 . . . .
  // 2 . . . .
  // 3 . x . .
  let q1 = Queen::new(0,0);
  let q2 = Queen::new(3,0);
  // positive case
  assert!(q1._collides_horizontally(&q2));

  let q3 = Queen::new(1,3);
  // negative case
  assert!(!q1._collides_horizontally(&q3));
}

#[test]
fn test_vertical_collides() {
  //   0 1 2 3
  // 0 x . . .
  // 1 . . . x
  // 2 . . . .
  // 3 x . . .
  let q1 = Queen::new(0,0);
  let q2 = Queen::new(0,3);
  // positive case
  assert!(q1._collides_vertically(&q2));

  let q3 = Queen::new(3,1);
  // negative case
  assert!(!q1._collides_vertically(&q3));
}

#[test]
fn test_collides() {
  //   0 1 2 3 4
  // 0 x x . . .
  // 1 x . . . .
  // 2 . . . . x
  // 3 . . . . .
  // 4 . . . . .
  let q1 = Queen::new(0,0);
  let q2 = Queen::new(0,1);
  let q3 = Queen::new(1,0);
  let q4 = Queen::new(1,1);
  let q5 = Queen::new(4,2);

  assert!(q1.collides(&q2));
  assert!(q1.collides(&q3));
  assert!(q1.collides(&q4));

  assert!(!q1.collides(&q5));
}
