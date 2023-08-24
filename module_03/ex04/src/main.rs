use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

enum TimeParseError {
	MissingColon,
	InvalidLength,
	InvalidNumber,
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

struct Time {
	hours: u32,
	minutes: u32,
}

impl std::str::FromStr for Time {
	type Err = TimeParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s_bytes: &[u8] = s.as_bytes();

		if s_bytes.len() != 5 {
			return Err(Self::Err::InvalidLength);
		}
		// TODO: Split on colon, checking if it is missing at the same time.
		// TODO: Check if there is only 2 resulting strings.
		// TODO: Check if the resulting strings have both a length of 2.
		// TODO: Parse both resulting strings as u32, and check the numbers are valid as hours and minutes.
	}
}

impl Display for Time {
	fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(formatter, "{} hours, {} minutes", self.hours, self.minutes)
	}
}

fn main() {
	let a: Time = "12:20".parse().unwrap();
	let b: Time = "15:14".parse().unwrap();

	println!("{a}");
	println!("{b}");

	let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
	let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
	let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
	println!("error: {err1}");
	println!("error: {err2}");
	println!("error: {err3}");
}
