enum ParseError {
	InvalidWidth { arg: &'static str },
	InvalidHeight { arg: &'static str },
	InvalidPercentage { arg: &'static str },
	TooManyArguments,
	NotEnoughArguments,
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
			return ((random_number(0..i32::MAX) + random_number(0..i32::MAX)) as u32
				* (random_number(0..i32::MAX) + random_number(0..i32::MAX)) as u32) as usize
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
					while let i = random_index(vec_size) {
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

	// TODO
	fn from_args() -> Result<Self, ParseError> {}

	// TODO
	fn step(&mut self);

	// TODO
	fn print(&self, clear: bool);
}

fn main() {
	println!("Hello, world!");
}
