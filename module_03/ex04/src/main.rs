use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq)]
enum TimeParseError {
	InvalidLength,
	InvalidNumber,
	MissingColon,
}

impl Debug for TimeParseError {
	fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(
			formatter,
			"{}",
			match self {
				Self::MissingColon => "missing ':'",
				Self::InvalidLength => "invalid length",
				Self::InvalidNumber => "invalid number",
			}
		)
	}
}

impl Display for TimeParseError {
	fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(
			formatter,
			"{}",
			match self {
				Self::MissingColon => "missing ':'",
				Self::InvalidLength => "invalid length",
				Self::InvalidNumber => "invalid number",
			}
		)
	}
}

#[derive(PartialEq)]
struct Time {
	hours: u32,
	minutes: u32,
}

impl std::str::FromStr for Time {
	type Err = TimeParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		#[inline(always)]
		fn find(s: &[u8], c: u8, n: usize) -> Option<usize> {
			for i in 0..n {
				if s[i] == c {
					return Some(i);
				}
			}
			None
		}

		const EXPECTED_LEN: usize = 5;
		const EXPECTED_COLON_INDEX: usize = 2;

		let bytes: &[u8] = s.as_bytes();
		let len: usize = bytes.len();
		let colon_index: usize = match find(bytes, b':', len) {
			Some(ok) => ok,
			None => return Err(Self::Err::MissingColon),
		};

		if len != EXPECTED_LEN || colon_index != EXPECTED_COLON_INDEX {
			return Err(Self::Err::InvalidLength);
		}

		for i in 0..EXPECTED_COLON_INDEX {
			if !bytes[i].is_ascii_digit() {
				return Err(Self::Err::InvalidNumber);
			}
		}
		for i in EXPECTED_COLON_INDEX + 1..EXPECTED_LEN {
			if !bytes[i].is_ascii_digit() {
				return Err(Self::Err::InvalidNumber);
			}
		}

		let hours: u32 = s[..EXPECTED_COLON_INDEX].parse().unwrap();

		if hours > 23 {
			return Err(Self::Err::InvalidNumber);
		}

		let minutes: u32 = s[EXPECTED_COLON_INDEX + 1..].parse().unwrap();

		if minutes > 59 {
			return Err(Self::Err::InvalidNumber);
		}

		Ok(Self { hours, minutes })
	}
}

impl Debug for Time {
	fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(formatter, "{} hours, {} minutes", self.hours, self.minutes)
	}
}

impl Display for Time {
	fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(formatter, "{} hours, {} minutes", self.hours, self.minutes)
	}
}

fn main() {
	const RED: &str = "\x1b[38;2;255;0;0m";
	const GREEN: &str = "\x1b[38;2;0;255;0m";
	const RESET: &str = "\x1b[0m";

	println!("Tests:");

	// region: Test error cases
	{
		let padding: usize = 8;
		let tests: [(&str, TimeParseError); 26] = [
			// region: tests
			("", TimeParseError::MissingColon),
			("12", TimeParseError::MissingColon),
			("1234", TimeParseError::MissingColon),
			("12.34", TimeParseError::MissingColon),
			("abcdef", TimeParseError::MissingColon),
			(":", TimeParseError::InvalidLength),
			("a:", TimeParseError::InvalidLength),
			(":a", TimeParseError::InvalidLength),
			("12:", TimeParseError::InvalidLength),
			(":12", TimeParseError::InvalidLength),
			("1:23", TimeParseError::InvalidLength),
			("12:3", TimeParseError::InvalidLength),
			("12:345", TimeParseError::InvalidLength),
			("123:45", TimeParseError::InvalidLength),
			("12:3a", TimeParseError::InvalidNumber),
			("12:+4", TimeParseError::InvalidNumber),
			("12:-4", TimeParseError::InvalidNumber),
			("1b:34", TimeParseError::InvalidNumber),
			("+2:34", TimeParseError::InvalidNumber),
			("-2:34", TimeParseError::InvalidNumber),
			("12:60", TimeParseError::InvalidNumber),
			("12:84", TimeParseError::InvalidNumber),
			("12:99", TimeParseError::InvalidNumber),
			("24:34", TimeParseError::InvalidNumber),
			("42:34", TimeParseError::InvalidNumber),
			("99:34", TimeParseError::InvalidNumber),
			// endregion
		];

		println!("\tError cases:");
		for test in tests {
			println!(
				"\t\t{:>padding$}: {}",
				format!("\"{}\"", test.0),
				if test.0.parse::<Time>() == Err(test.1) {
					format!("{GREEN}[OK]{RESET}")
				} else {
					format!("{RED}[KO]{RESET}")
				},
				padding = padding,
			);
		}
	}
	// endregion

	println!();

	// region: Test valid cases
	{
		let padding: usize = 8;
		let tests: [(&str, Time); 11] = [
			// region: tests
			("00:00", Time { hours: 0, minutes: 0 }),
			("00:01", Time { hours: 0, minutes: 1 }),
			("00:59", Time { hours: 0, minutes: 59 }),
			("01:00", Time { hours: 1, minutes: 0 }),
			("01:01", Time { hours: 1, minutes: 1 }),
			("01:59", Time { hours: 1, minutes: 59 }),
			("23:00", Time { hours: 23, minutes: 0 }),
			("23:01", Time { hours: 23, minutes: 1 }),
			("23:59", Time { hours: 23, minutes: 59 }),
			("12:34", Time { hours: 12, minutes: 34 }),
			("21:42", Time { hours: 21, minutes: 42 }),
			// endregion
		];

		println!("\tValid cases:");
		for test in tests {
			println!(
				"\t\t{:>padding$}: {}",
				format!("\"{}\"", test.0),
				if test.0.parse::<Time>() == Ok(test.1) {
					format!("{GREEN}[OK]{RESET}")
				} else {
					format!("{RED}[KO]{RESET}")
				},
				padding = padding,
			);
		}
	}
	// endregion
}
