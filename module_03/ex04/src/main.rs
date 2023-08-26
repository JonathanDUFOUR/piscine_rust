use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(PartialEq)]
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
	const RED: &str = "\x1b[38;2;255;0;0m";
	const GREEN: &str = "\x1b[38;2;0;255;0m";
	const RESET: &str = "\x1b[0m";

	println!("Tests:");

	/* Test error cases */
	{
		println!("\tError cases:");
		println!(
			"\t\t{:>0}: {}",
			"\"\"",
			match "".parse::<Time>() {
				Err(TimeParseError::MissingColon) => format!("{GREEN}[OK]{RESET}"),
				_ => format!("{RED}[KO]{RESET}"),
			}
		);
	}
}
