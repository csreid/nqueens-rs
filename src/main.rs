extern crate stopwatch;

use std::collections::BinaryHeap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::cmp::Ord;
use std::cmp::Ordering;

use stopwatch::{Stopwatch};

const N:i32 = 14;

#[derive(Eq,Clone,Hash,Debug)]
struct Board {
	queen_locs: Vec<(i32, i32)>,
	depth: i32,
	col_filled: [bool; N as usize],
	row_filled: [bool; N as usize],
	diag_filled: [bool; (N * 2) as usize],
	neg_diag_filled: [bool; (N * 2) as usize]
}

#[derive(Eq)]
struct BoardEval {
	n_queens: i32,
	n_open: i32
}

impl Ord for Board {
	fn cmp(&self, other:&Self) -> Ordering {
		self.eval().cmp(&other.eval())
	}
}

impl Ord for BoardEval {
	fn cmp(&self, other:&Self) -> Ordering {
		if self.n_queens > other.n_queens {
			Ordering::Greater
		} else if self.n_queens < other.n_queens {
			Ordering::Less
		} else if self.n_open > other.n_open {
			Ordering::Greater
		} else if self.n_open < other.n_open{
			Ordering::Less
		} else {
			Ordering::Equal
		}
	}
}

impl PartialOrd for Board {
	fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
		self.eval().partial_cmp(&other.eval())
	}
}

impl PartialOrd for BoardEval {
	fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Board {
	fn eq(&self, other:&Self) -> bool {
		self.eval() == other.eval()
	}
}
impl PartialEq for BoardEval {
	fn eq(&self, other:&Self) -> bool {
		self.n_queens == other.n_queens && self.n_open == other.n_open
	}
}

impl Display for Board {
	fn fmt(&self, f: &mut Formatter) -> Result {
		for _ in 0 .. N {
			write!(f, "+---");
		}

		write!(f, "+\n");

		for x in 0..N {
			for y in 0..N {
				if self.queen_locs.iter().any(|(x1, y1)| *x1 == x && *y1 == y) {
					write!(f, "| Q ");
				} else {
					write!(f, "|   ");
				}
			}

			write!(f, "|\n");

			for _ in 0 .. N {
				write!(f, "+---");
			}
			write!(f, "+\n");
		}

		Ok(())
	}
}

impl Board {
	fn new() -> Board {
		Board {
			queen_locs: vec![],
			depth: 0,
			col_filled: [false; N as usize],
			row_filled: [false; N as usize],
			diag_filled: [false; (N * 2) as usize],
			neg_diag_filled: [false; (N * 2) as usize]
		}
	}

	fn eval(&self) -> BoardEval {
		let mut total = 0;
		for i in (0..=N) {
			for j in (0..=N) {
				if j == 2 {
					total += 1;
				}
			}
		}

		BoardEval {
			n_queens: self.queens(),
			n_open: total
		}
	}

	fn queens(&self) -> i32 {
		self.queen_locs.len() as i32
	}

	fn can_add_queen(&self, x:&i32, y:&i32) -> bool {
		if self.col_filled[*x as usize] {
			return false
		}

		if self.row_filled[*y as usize] {
			return false
		}

		if self.diag_filled[((*y - *x) + N - 1) as usize] {
			return false
		}

		if self.neg_diag_filled[((*y + *x)) as usize] {
			return false
		}

		true
	}

	fn add_queen(&self, x:usize, y:usize) -> Board {
		let mut newlocs = self.queen_locs.clone();
		let mut newcolfilled = self.col_filled.clone();
		let mut newrowfilled = self.row_filled.clone();
		let mut newdiagfilled = self.diag_filled.clone();
		let mut newnegdiagfilled = self.neg_diag_filled.clone();

		newlocs.push((x as i32, y as i32));
		newcolfilled[x] = true;
		newrowfilled[y] = true;
		newdiagfilled[y + ((N - 1) as usize) - x] = true;
		newnegdiagfilled[y + x] = true;

		Board {
			queen_locs: newlocs,
			depth: self.depth + 1,
			col_filled: newcolfilled,
			row_filled: newrowfilled,
			diag_filled: newdiagfilled,
			neg_diag_filled: newnegdiagfilled
		}
	}

	fn children(&self) -> Vec<Board> {
		let mut retval = vec![];

		for i in (0..N) {
			for j in (0..N) {
				if self.can_add_queen(&(i as i32), &(j as i32)) {
					retval.push(self.add_queen(i as usize, j as usize))
				}
			}
		}

		retval
	}

	fn n_queens_bestfs(&self, n:i32) -> (Board, i32) {
		let mut heap = BinaryHeap::with_capacity(1000);
		let mut checks = 0;

		heap.push(self.clone());

		loop {
			let next = heap.pop().unwrap();

			if next.queens() == n {
				return (next, checks)
			}
			checks += 1;

			for c in next.children() {
				heap.push(c)
			}
		}
	}

	fn n_queens_dfs(&self, n:i32) -> (Board, i32) {
		let mut list = Vec::new();
		let mut checks = 0;

		list.push(self.clone());

		let children = self.children();
		let startTime = Stopwatch::start_new();

		loop {
			let next = list.pop().unwrap();

			checks += 1;
			if next.queens() == n {
				return (next, checks)
			}

			if checks % 500000 == 0 {
				println!("{} checks; Rate: {}/ms", checks, (checks as f64) / (startTime.elapsed_ms() as f64));
			}

			for c in next.children() {
				list.push(c);
			}
		}
	}
}

fn main() {
	let b = Board::new();
	let sw = Stopwatch::start_new();
	let (board2, total2) = b.n_queens_dfs(N);
	let time = (sw.elapsed_ms() as f64);

	println!("{}", board2);
	println!("DFS Iterations in {}ms: {}\nRate: {}/s", time, total2, ((total2 as f64) / time) * 1000 as f64);
}
