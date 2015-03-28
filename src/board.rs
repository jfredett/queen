use piece::*;

pub struct Board {
  size: i32,
  queens: Vec<Queen>,
}

impl Board {
  pub fn new(size: i32) -> Board {
    Board {
      size: size,
      queens: vec![],
    }
  }

  pub fn consistent(&self) -> bool {
    for q in self.queens.iter() {
      for p in self.queens.iter() {
        if (q != p) && (q.collides(p)) {
          return false;
        }
      }
    }
    return true;
  }

  pub fn add_queen(&mut self, x: i32, y: i32) {
    if self.has_queen(x,y) {
      println!("Queen already present");
      return;
    } else {
      self.queens.push(Queen::new(x,y));
    }
  }

  pub fn has_queen(&self, x: i32, y: i32) -> bool {
    let proposed = Queen::new(x,y);
    for q in self.queens.iter() {
      if &proposed == q {
        return true;
      }
    }
    return false;
  }

  pub fn add_noncolliding_queen(&mut self) {
  }

  pub fn print(&self) {
    print!("  ");
    for col in 0..self.size {
      print!("{} ", col);
    }
    println!("");
    for i in 0..self.size {
      print!("{} ", i);
      for j in 0..self.size {
        if self.has_queen(i,j) {
          print!("Q");
        } else {
          print!(".");
        }
        print!(" ");
      }
      println!("");
    }
  }
}
