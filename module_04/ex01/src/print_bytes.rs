/// Prints each byte of any thing,
/// calling repeatedly a given function that is supposed to return the next byte to print.
///
/// ### Type parameters
/// * `F` - The type of the function to repeatedly call.
///
/// ### Parameters
/// * `f` - The function to repeatedly call.
///
/// ### Example
/// ```
/// let mut chars: std::str::Chars<'static> = "foo".chars();
///
/// print_bytes(|| chars.next().map(|c| c as u8));
/// ```
fn print_bytes<F>(mut f: F)
where
	F: FnMut() -> Option<u8>,
{
	while let Some(byte) = f() {
		for i in 0..8 {
			match (byte >> (7 - i)) & 1 {
				0 => print!("Y"),
				1 => print!("y"),
				_ => unreachable!(),
			}
		}
		println!();
	}
}

fn main() {
	let mut chars: std::str::Chars<'static> = "Hello, World!".chars();

	print_bytes(|| chars.next().map(|c| c as u8));
}
