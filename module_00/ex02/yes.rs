/// Print "y" forever.
///
/// # Example
/// ```
/// use ex02::yes;
///
/// yes();
/// ```
fn yes() -> ! {
	loop {
		println("y");
	}
}

fn main() {
	yes();
}
