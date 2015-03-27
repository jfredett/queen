extern crate queen;

/*use queen::piece::*;*/
use queen::board::*;

fn main() {
  let mut b = Board::new(8);

  b.add_queen(0,0);
  b.add_queen(1,2);
  b.add_queen(2,1);

  println!("consistent? -- {}", b.consistent());

  b.print();
}

/*


  let size = 8;
  let mut queens : Vec<Queen> = vec![];

  let mut add_queen = true;

  for i in 0..size {
    for j in 0..size {
      let queen = Queen::new(i,j);

      for e in queens.iter() {
        if e.collides(&queen) {
          add_queen = false;
          break;
        }
      }

      if add_queen { queens.push(queen); }

      add_queen = true;
    }
  }

  let mut sq = ".";

  for i in 0..size {
    for j in 0..size {
      for e in queens.iter() {
        if e.x() == i && e.y() == j {
          sq = "Q";
          break;
        }
      }

      print!("{} ", sq);
      sq = ".";
    }
    println!("");
  }
*/
