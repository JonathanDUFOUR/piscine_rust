/// Compares two numbers and returns the lowest one.
///
/// ### Parameters
/// * `a`: The first number to compare.
/// * `b`: The second number to compare.
///
/// ### Return
/// The lowest number between `a` and `b`.
///
/// ### Example
/// ```
/// use ex01::min;
///
/// assert_eq!(min(0, 0), 0);
/// assert_eq!(min(1, 2), 1);
/// assert_eq!(min(3, -4), -4);
/// assert_eq!(min(-5, 6), -5);
/// assert_eq!(min(-7, -8), -8);
/// ```
fn min(a: i32, b: i32) -> i32 {
	if a < b {
		a
	} else {
		b
	}
}

fn main() {
	println!(
		" first call: {}\nsecond call: {}\n third call: {}\n",
		min(-0x80000000, 0),
		min(-1, -1),
		min(-2147483648, 2147483647),
	);
}
