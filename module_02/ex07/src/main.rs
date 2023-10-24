enum ParseError {
	NotEnoughArguments,
	TooManyArguments,
	InvalidWidth { arg: &'static str },
	InvalidHeight { arg: &'static str },
	InvalidPercentage { arg: &'static str },
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
	Dead,
	Alive,
}

impl Cell {
	/// Checks if the cell is alive.
	///
	/// # Return
	/// * `true` - The cell is alive.
	/// * `false` - The cell is not alive.
	#[inline(always)]
	fn is_alive(self: &Self) -> bool {
		*self == Cell::Alive
	}

	/// Checks if the cell is dead.
	///
	/// # Return
	/// * `true` - The cell is dead.
	/// * `false` - The cell is not dead.
	#[allow(dead_code)]
	#[inline(always)]
	fn is_dead(self: &Self) -> bool {
		*self == Cell::Dead
	}
}

struct Board {
	width: usize,
	height: usize,
	cells: Vec<Cell>,
}

impl Board {
	/// Creates a new Board instance and initializes its attributes.
	/// The generated board will contains a certain percentage of alive cells,
	/// and their posistions will be random.
	///
	/// # Parameters
	/// * `width` - The width of the board.
	/// * `height` - The height of the board.
	/// * `percentage` - The percentage of alive cells.
	///
	/// # Return
	/// The newly created Board instance.
	#[inline(always)]
	fn new(width: usize, height: usize, percentage: u8) -> Self {
		#[inline(always)]
		fn random_index(len: usize) -> usize {
			use ftkit::random_number;

			(random_number(i32::MIN..i32::MAX) as u32 as usize
				* random_number(i32::MIN..i32::MAX) as u32 as usize)
				% len
		}

		return Self {
			width,
			height,
			cells: {
				let vec_len: usize = width * height;
				let alive_cell_count: usize = percentage as usize * vec_len / 100;
				let mut cells: Vec<Cell> = vec![Cell::Dead; vec_len];

				for _ in 0..alive_cell_count {
					loop {
						let i: usize = random_index(vec_len);

						if cells[i] == Cell::Dead {
							cells[i] = Cell::Alive;
							break;
						}
					}
				}
				cells
			},
		};
	}

	/// Parses the command-line arguments passed to the application
	/// and use them to create a Board instance.
	///
	/// # Return
	/// * `Ok(Self)` - The generated board.
	/// * `Err(ParseError)` - The command-line arguments are invalid.
	fn from_args() -> Result<Self, ParseError> {
		use ftkit::ARGS;

		if ARGS.len() < 4 {
			return Err(ParseError::NotEnoughArguments);
		}
		if ARGS.len() > 4 {
			return Err(ParseError::TooManyArguments);
		}

		let width: usize = match ARGS[1].parse::<usize>() {
			Ok(width) => width,
			Err(_) => return Err(ParseError::InvalidWidth { arg: &ARGS[1] }),
		};
		let height: usize = match ARGS[2].parse::<usize>() {
			Ok(height) => height,
			Err(_) => return Err(ParseError::InvalidHeight { arg: &ARGS[2] }),
		};
		let percentage: u8 = match ARGS[3].parse::<u8>() {
			Ok(percentage) => percentage,
			Err(_) => return Err(ParseError::InvalidPercentage { arg: &ARGS[3] }),
		};
		if percentage > 100 {
			return Err(ParseError::InvalidPercentage { arg: &ARGS[3] });
		}
		return Ok(Self::new(width, height, percentage));
	}

	/// Simulates the next step of the game.
	/// It is assumed that the board is a torus:
	/// - the left and right edges are connected
	/// - the top and bottom edges are connected
	fn step(&mut self) {
		#[inline(always)]
		fn alive_neighbor_count(neighbors: &[Cell]) -> u8 {
			let mut count: u8 = 0;

			for i in 0..neighbors.len() {
				if neighbors[i].is_alive() {
					count += 1;
				}
			}

			return count;
		}

		// region: Easy edge cases
		if self.width == 0 || self.height == 0 {
			return;
		}
		if self.width == 1 && self.height == 1 {
			self.cells[0] = Cell::Dead;
			return;
		}
		// endregion

		let mut new_cells: Vec<Cell> = vec![Cell::Dead; self.cells.len()];
		let mut neighbors: [Cell; 8];

		// region: More complex edge cases
		if self.width == 1 || self.height == 1 {
			// region: Extremity cells
			let last: usize = self.cells.len() - 1;
			let penultimate: usize = last - 1;

			// region: First cell
			neighbors = [
				self.cells[last],
				self.cells[last],
				self.cells[last],
				self.cells[0],
				self.cells[0],
				self.cells[1],
				self.cells[1],
				self.cells[1],
			];
			if self.cells[0].is_alive() {
				match alive_neighbor_count(&neighbors) {
					2 | 3 => new_cells[0] = Cell::Alive,
					_ => (),
				}
			} else if alive_neighbor_count(&neighbors) == 3 {
				new_cells[0] = Cell::Alive;
			}
			// endregion

			// region: Last cell
			neighbors = [
				self.cells[penultimate],
				self.cells[penultimate],
				self.cells[penultimate],
				self.cells[last],
				self.cells[last],
				self.cells[0],
				self.cells[0],
				self.cells[0],
			];
			if self.cells[last].is_alive() {
				match alive_neighbor_count(&neighbors) {
					2 | 3 => new_cells[last] = Cell::Alive,
					_ => (),
				}
			} else if alive_neighbor_count(&neighbors) == 3 {
				new_cells[last] = Cell::Alive;
			}
			// endregion
			// endregion

			// region: Intermediate cells
			for i in 1..self.cells.len() - 1 {
				neighbors = [
					self.cells[i - 1],
					self.cells[i - 1],
					self.cells[i - 1],
					self.cells[i],
					self.cells[i],
					self.cells[i + 1],
					self.cells[i + 1],
					self.cells[i + 1],
				];

				if self.cells[i].is_alive() {
					match alive_neighbor_count(&neighbors) {
						2 | 3 => new_cells[i] = Cell::Alive,
						_ => (),
					}
				} else if alive_neighbor_count(&neighbors) == 3 {
					new_cells[i] = Cell::Alive;
				}
			}
			// endregion

			self.cells = new_cells;
			return;
		}
		// endregion

		// region: Common cases
		// region: Corners
		const TOP_LEFT: usize = 0;
		const TOP_RIGHT: usize = 1;
		const BOTTOM_LEFT: usize = 2;
		const BOTTOM_RIGHT: usize = 3;

		let areas: [[Cell; 4]; 4] = [
			[
				self.cells[0],
				self.cells[1],
				self.cells[self.width],
				self.cells[self.width + 1],
			],
			[
				self.cells[self.width - 2],
				self.cells[self.width - 1],
				self.cells[self.width * 2 - 2],
				self.cells[self.width * 2 - 1],
			],
			[
				self.cells[self.cells.len() - (self.width * 2)],
				self.cells[self.cells.len() - (self.width * 2) + 1],
				self.cells[self.cells.len() - self.width],
				self.cells[self.cells.len() - self.width + 1],
			],
			[
				self.cells[self.cells.len() - self.width - 2],
				self.cells[self.cells.len() - self.width - 1],
				self.cells[self.cells.len() - 2],
				self.cells[self.cells.len() - 1],
			],
		];

		// region: Top-left corner
		neighbors = [
			areas[BOTTOM_RIGHT][BOTTOM_RIGHT],
			areas[BOTTOM_LEFT][BOTTOM_LEFT],
			areas[BOTTOM_LEFT][BOTTOM_RIGHT],
			areas[TOP_RIGHT][TOP_RIGHT],
			areas[TOP_LEFT][TOP_RIGHT],
			areas[TOP_RIGHT][BOTTOM_RIGHT],
			areas[TOP_LEFT][BOTTOM_LEFT],
			areas[TOP_LEFT][BOTTOM_RIGHT],
		];
		if areas[TOP_LEFT][TOP_LEFT].is_alive() {
			match alive_neighbor_count(&neighbors) {
				2 | 3 => new_cells[0] = Cell::Alive,
				_ => (),
			}
		} else if alive_neighbor_count(&neighbors) == 3 {
			new_cells[0] = Cell::Alive;
		}
		// endregion

		// region: Top-right corner
		neighbors = [
			areas[BOTTOM_RIGHT][BOTTOM_LEFT],
			areas[BOTTOM_RIGHT][BOTTOM_RIGHT],
			areas[BOTTOM_LEFT][BOTTOM_LEFT],
			areas[TOP_RIGHT][TOP_LEFT],
			areas[TOP_LEFT][TOP_LEFT],
			areas[TOP_RIGHT][BOTTOM_LEFT],
			areas[TOP_RIGHT][BOTTOM_RIGHT],
			areas[TOP_LEFT][BOTTOM_LEFT],
		];
		if areas[TOP_RIGHT][TOP_RIGHT].is_alive() {
			match alive_neighbor_count(&neighbors) {
				2 | 3 => new_cells[self.width - 1] = Cell::Alive,
				_ => (),
			}
		} else if alive_neighbor_count(&neighbors) == 3 {
			new_cells[self.width - 1] = Cell::Alive;
		}
		// endregion

		// region: Bottom-left corner
		neighbors = [
			areas[BOTTOM_RIGHT][TOP_RIGHT],
			areas[BOTTOM_LEFT][TOP_LEFT],
			areas[BOTTOM_LEFT][TOP_RIGHT],
			areas[BOTTOM_RIGHT][BOTTOM_RIGHT],
			areas[BOTTOM_LEFT][BOTTOM_RIGHT],
			areas[TOP_RIGHT][TOP_RIGHT],
			areas[TOP_LEFT][TOP_LEFT],
			areas[TOP_LEFT][TOP_RIGHT],
		];
		if areas[BOTTOM_LEFT][BOTTOM_LEFT].is_alive() {
			match alive_neighbor_count(&neighbors) {
				2 | 3 => new_cells[self.width * (self.height - 1)] = Cell::Alive,
				_ => (),
			}
		} else if alive_neighbor_count(&neighbors) == 3 {
			new_cells[self.width * (self.height - 1)] = Cell::Alive;
		}
		// endregion

		// region: Bottom-right corner
		neighbors = [
			areas[BOTTOM_RIGHT][TOP_LEFT],
			areas[BOTTOM_RIGHT][TOP_RIGHT],
			areas[BOTTOM_LEFT][TOP_LEFT],
			areas[BOTTOM_RIGHT][BOTTOM_LEFT],
			areas[BOTTOM_LEFT][BOTTOM_LEFT],
			areas[TOP_RIGHT][TOP_LEFT],
			areas[TOP_RIGHT][TOP_RIGHT],
			areas[TOP_LEFT][TOP_LEFT],
		];
		if areas[BOTTOM_RIGHT][BOTTOM_RIGHT].is_alive() {
			match alive_neighbor_count(&neighbors) {
				2 | 3 => new_cells[self.width * self.height - 1] = Cell::Alive,
				_ => (),
			}
		} else if alive_neighbor_count(&neighbors) == 3 {
			new_cells[self.width * self.height - 1] = Cell::Alive;
		}
		// endregion
		// endregion

		// region: Edges
		// region: Left & Right edges
		for y in 1..self.height - 1 {
			// region: Left edge
			neighbors = [
				self.cells[self.width * y - 1],
				self.cells[self.width * (y - 1)],
				self.cells[self.width * (y - 1) + 1],
				self.cells[self.width * (y + 1) - 1],
				self.cells[self.width * y + 1],
				self.cells[self.width * (y + 2) - 1],
				self.cells[self.width * (y + 1)],
				self.cells[self.width * (y + 1) + 1],
			];
			if self.cells[self.width * y].is_alive() {
				match alive_neighbor_count(&neighbors) {
					2 | 3 => new_cells[self.width * y] = Cell::Alive,
					_ => (),
				}
			} else if alive_neighbor_count(&neighbors) == 3 {
				new_cells[self.width * y] = Cell::Alive;
			}
			// endregion

			// region: Right edge
			neighbors = [
				self.cells[self.width * y - 2],
				self.cells[self.width * y - 1],
				self.cells[self.width * (y - 1)],
				self.cells[self.width * (y + 1) - 2],
				self.cells[self.width * y],
				self.cells[self.width * (y + 2) - 2],
				self.cells[self.width * (y + 2) - 1],
				self.cells[self.width * (y + 1)],
			];
			if self.cells[self.width * (y + 1) - 1].is_alive() {
				match alive_neighbor_count(&neighbors) {
					2 | 3 => new_cells[self.width * (y + 1) - 1] = Cell::Alive,
					_ => (),
				}
			} else if alive_neighbor_count(&neighbors) == 3 {
				new_cells[self.width * (y + 1) - 1] = Cell::Alive;
			}
			// endregion
		}
		// endregion

		// region: Top & Bottom edges
		for x in 1..self.width - 1 {
			// region: Top edge
			neighbors = [
				self.cells[x + self.cells.len() - self.width - 1],
				self.cells[x + self.cells.len() - self.width],
				self.cells[x + self.cells.len() - self.width + 1],
				self.cells[x - 1],
				self.cells[x + 1],
				self.cells[x + self.width - 1],
				self.cells[x + self.width],
				self.cells[x + self.width + 1],
			];
			if self.cells[x].is_alive() {
				match alive_neighbor_count(&neighbors) {
					2 | 3 => new_cells[x] = Cell::Alive,
					_ => (),
				}
			} else if alive_neighbor_count(&neighbors) == 3 {
				new_cells[x] = Cell::Alive;
			}
			// endregion

			// region: Bottom edge
			neighbors = [
				self.cells[x + self.cells.len() - self.width * 2 - 1],
				self.cells[x + self.cells.len() - self.width * 2],
				self.cells[x + self.cells.len() - self.width * 2 + 1],
				self.cells[x + self.cells.len() - self.width - 1],
				self.cells[x + self.cells.len() - self.width + 1],
				self.cells[x - 1],
				self.cells[x],
				self.cells[x + 1],
			];
			if self.cells[x + self.cells.len() - self.width].is_alive() {
				match alive_neighbor_count(&neighbors) {
					2 | 3 => new_cells[x + self.cells.len() - self.width] = Cell::Alive,
					_ => (),
				}
			} else if alive_neighbor_count(&neighbors) == 3 {
				new_cells[x + self.cells.len() - self.width] = Cell::Alive;
			}
			// endregion
		}
		// endregion
		// endregion

		// region: Center area
		for y in 1..self.height - 1 {
			for x in 1..self.width - 1 {
				neighbors = [
					self.cells[self.width * (y - 1) + x - 1],
					self.cells[self.width * (y - 1) + x],
					self.cells[self.width * (y - 1) + x + 1],
					self.cells[self.width * y + x - 1],
					self.cells[self.width * y + x + 1],
					self.cells[self.width * (y + 1) + x - 1],
					self.cells[self.width * (y + 1) + x],
					self.cells[self.width * (y + 1) + x + 1],
				];
				if self.cells[self.width * y + x].is_alive() {
					match alive_neighbor_count(&neighbors) {
						2 | 3 => new_cells[self.width * y + x] = Cell::Alive,
						_ => (),
					}
				} else if alive_neighbor_count(&neighbors) == 3 {
					new_cells[self.width * y + x] = Cell::Alive;
				}
			}
		}
		// endregion
		// endregion

		self.cells = new_cells;
	}

	/// Displays the board on stdout.
	///
	/// # Parameters
	/// * `clear` - If `true`, clear a previously displayed board before displaying the new one.
	///
	/// # Example
	/// ```
	/// let board: Board = Board::new(42, 42, 42);
	/// board.print(false);
	/// board.print(true);
	/// ```
	fn print(self: &Self, clear: bool) {
		const BORDER_COLOR: &str = "\x1b[48;2;175;175;175m";
		const ALIVE_COLOR: &str = "\x1b[48;2;255;153;0m";
		const DEAD_COLOR: &str = "\x1b[48;2;0;0;0m";
		const RESET: &str = "\x1b[0m";

		fn print_horizontal_border(width: usize) {
			print!("{BORDER_COLOR}");
			for _ in 0..width + 2 {
				print!("  ");
			}
			println!("{RESET}");
		}

		if clear {
			print!("\x1b[{}A", self.height + 2);
		}

		print_horizontal_border(self.width);
		for y in 0..self.height {
			print!("{BORDER_COLOR}  ");
			for x in 0..self.width {
				if self.cells[self.width * y + x].is_alive() {
					print!("{ALIVE_COLOR}  ");
				} else {
					print!("{DEAD_COLOR}  ");
				}
			}
			println!("{BORDER_COLOR}  {RESET}");
		}
		print_horizontal_border(self.width);
	}
}

fn main() {
	let mut board: Board = match Board::from_args() {
		Ok(board) => board,
		Err(error) => {
			match error {
				ParseError::NotEnoughArguments => eprintln!("error: not enough arguments"),
				ParseError::TooManyArguments => eprintln!("error: too many arguments"),
				ParseError::InvalidWidth { arg } => eprintln!("error: invalid width ({arg})"),
				ParseError::InvalidHeight { arg } => eprintln!("error: invalid height ({arg})"),
				ParseError::InvalidPercentage { arg } => {
					eprintln!("error: invalid percentage ({arg})")
				}
			}
			return;
		}
	};

	board.print(false);
	loop {
		std::thread::sleep(std::time::Duration::from_millis(42));
		board.step();
		board.print(true);
	}
}
