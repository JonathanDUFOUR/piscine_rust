enum ParseError {
	NotEnoughArguments,
	TooManyArguments,
	InvalidWidth { arg: &'static str },
	InvalidHeight { arg: &'static str },
	InvalidPercentage { arg: &'static str },
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
	Dead,
	Alive,
}

impl Cell {
	/// Check if the cell is alive.
	///
	/// # Returns
	///
	/// * `true` if the cell is alive.
	/// * `false` otherwise.
	///
	/// # Examples
	/// ```
	/// let cell_a: Cell = Cell::Alive;
	/// let cell_b: Cell = Cell::Dead;
	///
	/// assert_eq!(cell_a.is_alive(), true);
	/// assert_eq!(cell_b.is_alive(), false);
	/// ```
	#[inline(always)]
	fn is_alive(self: &Self) -> bool {
		return self == &Cell::Alive;
	}

	/// Check if the cell is dead.
	///
	/// # Returns
	///
	/// * `true` if the cell is dead.
	/// * `false` otherwise.
	///
	/// # Examples
	/// ```
	/// let cell_a: Cell = Cell::Alive;
	/// let cell_b: Cell = Cell::Dead;
	///
	/// assert_eq!(cell_a.is_dead(), false);
	/// assert_eq!(cell_b.is_dead(), true);
	/// ```
	#[allow(dead_code)]
	#[inline(always)]
	fn is_dead(self: &Self) -> bool {
		return self == &Cell::Dead;
	}
}

struct Board {
	width: usize,
	height: usize,
	cells: Vec<Cell>,
}

impl Board {
	/// Create a new Board instance and initialize its attributes.
	/// The generated board will contains a certain percentage of alive cells,
	/// and their posistions will be random.
	///
	/// # Arguments
	///
	/// * `width` - The width of the board.
	/// * `height` - The height of the board.
	/// * `percentage` - The percentage of alive cells.
	///
	/// # Returns
	///
	/// The newly created Board instance.
	///
	/// # Examples
	/// ```
	/// let board: Board = Board::new(10, 10);
	///
	/// assert_eq!(board, Board { width});
	/// ```
	#[inline(always)]
	fn new(width: usize, height: usize, percentage: u8) -> Self {
		#[inline(always)]
		fn random_index(vec_size: usize) -> usize {
			use ftkit::random_number;
			return ((random_number(0..i32::MAX) as u32 + random_number(0..i32::MAX) as u32)
				as usize * (random_number(0..i32::MAX) as u32
				+ random_number(0..i32::MAX) as u32) as usize)
				% vec_size;
		}

		return Self {
			width,
			height,
			cells: {
				let vec_size: usize = width * height;
				let alive_cell_count: usize = percentage as usize * vec_size / 100;
				let mut cells: Vec<Cell> = vec![Cell::Dead; vec_size];

				for _ in 0..alive_cell_count {
					loop {
						let i: usize = random_index(vec_size);

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

	/// Parse the command-line arguments passed to the application
	/// and use them to create a Board instance.
	///
	/// # Returns
	///
	/// * `Ok(board)` if the arguments are valid.
	/// * `Err(error)` otherwise.
	///
	/// # Examples
	/// ```
	/// let board: Board = Board::from_args();
	/// ```
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

	/// Simulate the next step of the game.
	/// It is assumed that the board is a torus:
	/// - the left and right edges are connected
	/// - the top and bottom edges are connected
	///
	/// # Examples
	/// ```
	/// let board: Board = Board::new(42, 42, 42);
	/// board.step();
	/// ```
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

		/* Handling easy edge cases */
		{
			if self.width == 0 || self.height == 0 {
				return;
			}
			if self.width == 1 && self.height == 1 {
				self.cells[0] = Cell::Dead;
				return;
			}
		}

		let mut new_cells: Vec<Cell> = vec![Cell::Dead; self.cells.len()];
		let mut neighbors: [Cell; 8];

		/* Handling more complex edge cases */
		{
			if self.width == 1 || self.height == 1 {
				/* Extremity cells */
				{
					let last: usize = self.cells.len() - 1;
					let penultimate: usize = last - 1;

					/* First cell */
					{
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
					}

					/* Last cell */
					{
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
					}
				}

				/* Intermediate cells */
				{
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
				}

				self.cells = new_cells;
				return;
			}
		}

		/* First, we compute the corners */
		{
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

			/* Top-left corner */
			{
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
			}

			/* Top-right corner */
			{
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
			}

			/* Bottom-left corner */
			{
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
			}

			/* Bottom-right corner */
			{
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
			}
		}

		/* Then, we compute the edges */
		{
			/* Left & Right edges */
			{
				for y in 1..self.height - 1 {
					/* Left edge */
					{
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
					}

					/* Right edge */
					{
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
					}
				}
			}

			/* Top & Bottom edges */
			{
				for x in 1..self.width - 1 {
					/* Top edge */
					{
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
					}

					/* Bottom edge */
					{
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
					}
				}
			}
		}

		/* Finally, we compute the center area */
		{
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
		}

		self.cells = new_cells;
	}

	/// Display the board on stdout.
	///
	/// # Arguments
	///
	/// * `clear` - If `true`, clear a previously displayed board before displaying the new one.
	///
	/// # Examples
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
