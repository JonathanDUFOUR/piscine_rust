struct John;

use std::fmt;
impl std::fmt::Display for John {
	fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.precision() == Some(0) {
			write!(f, "Don't try to silence me!")
		} else {
			f.pad("Hey! I'm John.")
		}
	}
}

impl std::fmt::Debug for John {
	fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"John, the man himself.{}",
			if f.alternate() {
				" He's awesome AND formidable."
			} else {
				""
			}
		)
	}
}

impl std::fmt::Binary for John {
	fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Bip Boop?")
	}
}

fn main() {
	let john = John;

	println!("1. {john}");
	println!("2. |{john:<30}|");
	println!("3. |{john:>30}|");
	println!("4. {john:.6}");
	println!("5. {john:.0}");
	println!("6. {john:?}");
	println!("7. {john:#?}");
	println!("8. {john:b}");
}
