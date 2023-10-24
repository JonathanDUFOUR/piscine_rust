/// Calls a given function only once to get a string,
/// and then print that string infinitely.
///
/// # Type parameters
/// * `F` - The type of the function to call.
///
/// # Parameters
/// * `f` - The function to call only once to get the string to print.
///
/// # Example
/// ```
/// yes(|| "foo".to_string());
/// ```
fn yes<F>(f: F) -> !
where
	F: FnOnce() -> String,
{
	let s: String = f();

	loop {
		println!("{}", s);
	}
}

fn main() {
	yes(|| "YyY".to_string());
}
