use std::fmt;
use std::fmt::{Binary, Debug, Display, Formatter};

struct John;

impl Display for John {
	fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> fmt::Result {
		if formatter.precision() == Some(0) {
			write!(formatter, "Don't try to silence me!")
		} else {
			formatter.pad("Hey! I'm John.")
		}
	}
}

impl Debug for John {
	fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(
			formatter,
			"John, the man himself.{}",
			if formatter.alternate() {
				" He's awesome AND formidable."
			} else {
				""
			}
		)
	}
}

impl Binary for John {
	fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result {
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
