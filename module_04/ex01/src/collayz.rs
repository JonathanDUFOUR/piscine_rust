/// Computes the Collatz sequence (https://en.wikipedia.org/wiki/Collatz_conjecture)
/// for a given number, calling a given function each time the number gets an odd value.
///
/// ### Type parameters
/// * `F` - The type of the function to call.
///
/// ### Parameters
/// * `start` - The number to start the Collatz conjecture with.
/// * `f` - The function to call each time the number gets an odd value.
///
/// ### Example
/// ```
/// collayz(11, |n| println!("{}", n));
/// ```
fn collayz<F>(start: u32, f: F)
where
	F: Fn(u32),
{
	if start == 0 {
		return;
	}

	let mut n: u32 = start;

	while n != 1 {
		if n % 2 == 0 {
			n /= 2;
		} else {
			f(n);
			n = n * 3 + 1;
		}
	}
	f(n);
}

fn main() {
	collayz(11, |n| println!("{}", "Y".repeat(n as usize)));
}
