enum Command {
	Todo(String),
	Done(usize),
	Purge,
	Quit,
}

impl Command {
	/// Displays a prompt, reads standard input until a valid command is entered,
	/// and returns it's corresponding Command enum value.
	///
	/// ### Return
	/// * `Command::Todo` - The user entered a valid TODO command.
	/// * `Command::Done` - The user entered a valid DONE command.
	/// * `Command::Purge` - The user entered a valid PURGE command.
	/// * `Command::Quit` - Either the user entered a valid QUIT command
	/// or the standard input has been closed.
	fn prompt() -> Self {
		loop {
			let line: String = ftkit::read_line().trim().to_string();

			if line.is_empty() || line == "QUIT" {
				return Self::Quit;
			} else if let Some(task) = line.strip_prefix("TODO ") {
				return Self::Todo(task.trim().to_string());
			} else if let Some(index) = line.strip_prefix("DONE ") {
				if let Ok(index) = index.trim().parse() {
					return Self::Done(index);
				}
			} else if line.trim() == "PURGE" {
				return Self::Purge;
			}
		}
	}
}

struct TodoList {
	todos: Vec<String>,
	dones: Vec<String>,
}

impl TodoList {
	/// Creates a new TodoList instance and initializes its attribute.
	/// The newly created TodoList instance is empty.
	///
	/// ### Return
	/// The newly created and initialized TodoList instance.
	fn new() -> Self {
		Self { todos: Vec::new(), dones: Vec::new() }
	}

	/// Displays the content of the calling TodoList instance on stdout.
	fn display(self: &Self) {
		println!("+-------------------+");
		println!("|    TASKS TO DO    |");
		println!("+-------------------+");
		for (index, todo) in self.todos.iter().enumerate() {
			println!("[{index}] {todo}");
		}

		println!("+-------------------+");
		println!("|    TASKS  DONE    |");
		println!("+-------------------+");
		for (index, done) in self.dones.iter().enumerate() {
			println!("[{index}] {done}");
		}
	}

	/// Adds a new TODO task to the calling TodoList instance.
	///
	/// ### Parameters
	/// * `todo` - The TODO task to add.
	fn add(self: &mut Self, todo: String) {
		self.todos.push(todo);
	}

	/// Set an existing TODO task as done in the calling TodoList instance.
	///
	/// ### Parameters
	/// * `index` - The task index to be set as done.
	fn done(self: &mut Self, index: usize) {
		if index < self.todos.len() {
			self.dones.push(self.todos.remove(index));
		}
	}

	/// Remove all the done tasks from the calling TodoList instance.
	fn purge(self: &mut Self) {
		self.dones.clear();
	}
}

fn main() {
	let mut todo_list: TodoList = TodoList::new();

	loop {
		println!();
		todo_list.display();
		match Command::prompt() {
			Command::Todo(task) => todo_list.add(task),
			Command::Done(index) => todo_list.done(index),
			Command::Purge => todo_list.purge(),
			Command::Quit => break,
		}
	}
}
