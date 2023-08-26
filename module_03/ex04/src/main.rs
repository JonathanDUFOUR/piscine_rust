use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

enum TimeParseError {
	MissingColon,
	InvalidLength,
	InvalidNumber,
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

struct Time {
	hours: u32,
	minutes: u32,
}

impl std::str::FromStr for Time {
	type Err = TimeParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s_bytes: &[u8] = s.as_bytes();
		let colon_index = {
			let mut i: usize = 0;
			while i < s_bytes.len() {
				if s_bytes[i] == b':' {
					break;
				}
				i += 1;
			}
			i
		};

		if colon_index == s_bytes.len() {
			return Err(Self::Err::MissingColon);
		}
		if s_bytes[..colon_index].len() > 2 || s_bytes[colon_index + 1..].len() != 2 {
			return Err(Self::Err::InvalidLength);
		}
		for i in 0..colon_index {
			if !s_bytes[i].is_ascii_digit() {
				return Err(Self::Err::InvalidNumber);
			}
		}
		for i in colon_index + 1..s.len() {
			if !s_bytes[i].is_ascii_digit() {
				return Err(Self::Err::InvalidNumber);
			}
		}

		let hours: u32 = match s[..colon_index].parse() {
			Ok(ok) => ok,
			_ => return Err(Self::Err::InvalidNumber),
		};
		let minutes: u32 = match s[colon_index + 1..].parse() {
			Ok(ok) => ok,
			_ => return Err(Self::Err::InvalidNumber),
		};

		if hours > 23 || minutes > 59 {
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
	let a: Time = "12:20".parse().unwrap();
	let b: Time = "15:14".parse().unwrap();

	println!("{a}");
	println!("{b}");

	for c in "Hello World".as_bytes() {
		println!("{}", c);
	}

	let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
	let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
	let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
	println!("error: {err1}");
	println!("error: {err2}");
	println!("error: {err3}");
}
