/// Print "y" forever.
///
/// # Examples
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
