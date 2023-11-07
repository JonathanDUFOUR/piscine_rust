/// Computes and prints the Collatz sequence (https://en.wikipedia.org/wiki/Collatz_conjecture)
/// starting at a given number.
///
/// ### Parameters
/// * `start`: The starting number of the sequence.
///
/// ### Example
/// ```
/// use ex02::collatz;
///
/// collatz(42);
/// ```
fn collatz(mut start: u32) {
	while start != 1 {
		println!("{}", start);
		if start % 2 == 0 {
			start /= 2;
		} else {
			start = 3 * start + 1;
		}
	}
	println!("{}", start);
}

fn main() {
	collatz(123);
}
