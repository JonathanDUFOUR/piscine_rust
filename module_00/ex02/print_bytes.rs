/// Prints the bytes of a string.
///
/// # Parameters
/// * `s`: The string to print the bytes from.
///
/// # Example
/// ```
/// use ex02::print_bytes;
///
/// print_bytes("Hello Rust!\n");
/// ```
fn print_bytes(s: &str) {
	for c in s.bytes() {
		println!("{}", c);
	}
}

fn main() {
	let tutu: &str = "Déjà Vu\n";
	print_bytes(tutu);
}
