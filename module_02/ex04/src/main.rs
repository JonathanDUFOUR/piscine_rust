enum Command {
	Todo(String),
	Done(usize),
	Purge,
	Quit,
}

impl Command {
	/// Display a prompt, read standard input until a valid command is entered,
	/// and return it's corresponding Command enum value.
	///
	/// # Returns
	///
	/// - `Command::Todo` if the user entered a valid TODO command.
	/// - `Command::Done` if the user entered a valid DONE command.
	/// - `Command::Purge` if the user entered a valid PURGE command.
	/// - `Command::Quit` if the user entered a valid QUIT command
	/// or if the standard input has been closed.
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
	/// Create a new TodoList instance, initialize it to be empty, and return it.
	///
	/// # Returns
	///
	/// The newly created and initialized TodoList instance.
	fn new() -> Self {
		Self {
			todos: Vec::new(),
			dones: Vec::new(),
		}
	}

	/// Display the TodoList instance's content on stdout.
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

	/// Add a new TODO task to the TodoList instance.
	///
	/// # Arguments
	///
	/// - `todo`: The TODO task to add.
	///
	/// # Examples
	/// ```
	/// let mut todo_list: TodoList = TodoList::new();
	///
	/// todo_list.add("Buy milk".to_string());
	/// todo_list.add("Buy eggs".to_string());
	/// todo_list.add("Buy coffee".to_string());
	/// todo_list.add("Find a girlfriend".to_string());
	/// ```
	fn add(self: &mut Self, todo: String) {
		self.todos.push(todo);
	}

	/// Set an existing TODO task as done.
	///
	/// # Arguments
	///
	/// - `index`: The task index to be set as done.
	///
	/// # Examples
	/// ```
	/// let mut todo_list: TodoList = TodoList::new();
	///
	/// todo_list.add("Finish".to_string());
	/// todo_list.add("this".to_string());
	/// todo_list.add("examples".to_string());
	/// todo_list.add("section".to_string());
	///
	/// todo_list.done(1);
	/// todo_list.done(0);
	/// todo_list.done(1);
	/// todo_list.done(0);
	fn done(self: &mut Self, index: usize) {
		if index < self.todos.len() {
			self.dones.push(self.todos.remove(index));
		}
	}

	/// Remove all the done tasks from the TodoList instance.
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
