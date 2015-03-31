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
        (self.x <= bound as u16) && (self.y <= bound as u16)
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

struct ColumnIterator {
    current_position: Position,
    size: usize,
}

struct RankIterator {
    current_position: Position,
    size: usize,
}

struct FrontslashIterator {
    current_position: Position,
    size: usize,
}

struct BackslashIterator {
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
        if (self.current_position).bounded_by(self.size) {
            let (x,y) = ColumnIterator::iterator_vector();
            let ret = Some(self.current_position.clone());

            self.current_position = self.current_position.adjust(x,y);
            return ret
        } else {
            None
        }
    }
}

#[test]
fn column_creation() {
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
    assert!(col_iter.next().unwrap() == Position::new(1,3));
}


/*impl PositionIterator {*/

// A
//  012345
//0 .\.|./
//1 ..\|/.
//2 ---x--
//3 ../|\.
//4 ./.|.\
//5 /..|..
//
// B
//  012345
//0 .|.../
//1 .|../.
//2 .|./..
//3 \|/...
//4 -x----
//5 /|\...
//
// C
//  012345
//0 .\..|.
//1 ..\.|.
//2 ...\|/
//3 ----x-
//4 .../|\
//5 ../.|.

// A:
//  pos     : (3,2)
//  dia-min : (1,0)
//  dia-max : (5,0)
// B:
//  pos     : (1,4)
//  dia-min : (0,3)
//  dia-max : (5,0)
// C:
//  pos     : (4,3)
//  dia-min : (1,0)
//  dia-max : (5,2)

//
// (x,y) - A + r(1,1) = 0  | max(A) <= s
// (x,y) - A + r(1,-1) = 0 | max(A) <= s
//
// Position has an diagonal-minimal origin. i.e., the closest point to the actual
// origin which falls along a diagonal that aims toward the origin (the
// 'backslash' diagonal), and a diagonal-maximal origin position. i.e., the
// closest point to the origin which falls along a diagonal that aims *away*
// from the origin and *toward* the (0,s) square, where `s` is the size of the
// board.
// The diagonal-minimal origin for a position (x,y) is equal to 
//     (x - min(x,y), y - min(x,y))
// The diagonal-maximal origin for a position (x,y)
//

// The dia-min maintains that, for P = (x,y), dia-min (x',y'), |x' - y'| == |x - y|
// The dia-max maintains that, for P = (x,y), dia-max (x',y'), |x' + y'| == |x + y|


