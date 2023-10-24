/// Parses a string as a collection of positive integers, and compute the sum of all odd numbers in.
///
/// The positive integers contained in the string are separated by whitespaces.
/// (['\t', '\n', '\v', '\f', '\r', ' '])
///
/// Whenever a word is not a valid positive integer, it is ignored.
///
/// # Parameters
/// * `s` - The string to parse the positive integers from.
///
/// # Return
/// The sum of all odd positive integers in the string.
///
/// # Example
/// ```
/// use ex02::sum_of_odds;
///
/// assert_eq!(sum_of_odds("1 3 5"), 9);
/// assert_eq!(sum_of_odds("\t1\t0\t1\t0\t1\t0\t"), 3);
/// assert_eq!(sum_of_odds("
///  0  1  1
///  2  3  5
///  8 13 21
/// 34 55 89"), 188);
/// ```
pub fn sum_of_odds(s: &str) -> u32 {
	s.split_whitespace().filter_map(|word| word.parse::<u32>().ok()).filter(|&n| n % 2 == 1).sum()
}

use std::str::FromStr;
/// Parses a string as a collection of pairs.
///
/// Each line of the string is a pair, where the first element is the key,
/// and the second element is the value.
/// The two elements are separated by a single colon (':').
/// Both fields are trimmed before being parsed.
///
/// Whenever a line is not a valid pair, it is ignored.
///
/// # Parameters
/// * `s` - The string to parse the pairs from.
///
/// # Return
/// A `Vec` of pairs.
///
/// # Example
/// ```
/// use ex02::create_pairs;
///
/// assert_eq!(create_pairs::<u32>("  foo:0  "), vec![("foo", 0)]);
/// assert_eq!(
/// 	create_pairs::<u32>(
/// 		" foo : 0 \n bar : 1 \n muf : 2 \n"
/// 	),
/// 	vec![("foo", 0), ("bar", 1), ("muf", 2)]
/// );
/// ```
pub fn create_pairs<T: FromStr>(s: &str) -> Vec<(&str, T)> {
	s.lines()
		.filter_map(|line| match line.split_once(':') {
			Some((key, value)) => Some((key.trim(), value.trim())),
			None => None,
		})
		.filter_map(|(key, value)| match value.parse::<T>() {
			Ok(parsed) => Some((key, parsed)),
			Err(_) => None,
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	// region: Struct A
	#[derive(Debug, Eq, PartialEq)]
	struct A {}

	struct ParseAError;

	impl FromStr for A {
		type Err = ParseAError;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s.is_empty() {
				true => Ok(A {}),
				false => Err(ParseAError),
			}
		}
	}
	// endregion

	// region: Struct B
	#[derive(Debug, Eq, PartialEq)]
	struct B {
		a: String,
	}

	impl FromStr for B {
		type Err = ();

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			Ok(B { a: s.to_string() })
		}
	}
	// endregion

	// region: Struct C
	#[derive(Debug, Eq, PartialEq)]
	struct C {
		a: u8,
		b: i8,
	}

	struct ParseBError;

	impl FromStr for C {
		type Err = ParseBError;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s.split_once(',') {
				Some((sa, sb)) => match (sa.trim().parse::<u8>(), sb.trim().parse::<i8>()) {
					(Ok(a), Ok(b)) => Ok(C { a, b }),
					_ => Err(ParseBError),
				},
				None => Err(ParseBError),
			}
		}
	}
	// endregion

	// region: sum_of_odds_00
	#[test]
	fn sum_of_odds_00() {
		const S: &str = "";
		const EXPECTED: u32 = 0;

		assert_eq!(sum_of_odds(S), EXPECTED);
	}
	// endregion

	// region: sum_of_odds_01
	#[test]
	fn sum_of_odds_01() {
		const S: &str = "0";
		const EXPECTED: u32 = 0;

		assert_eq!(sum_of_odds(S), EXPECTED);
	}
	// endregion

	// region: sum_of_odds_02
	#[test]
	fn sum_of_odds_02() {
		const S: &str = "1";
		const EXPECTED: u32 = 1;

		assert_eq!(sum_of_odds(S), EXPECTED);
	}
	// endregion

	// region: sum_of_odds_03
	#[test]
	fn sum_of_odds_03() {
		const S: &str = " 1 2 3 ";
		const EXPECTED: u32 = 4;

		assert_eq!(sum_of_odds(S), EXPECTED);
	}
	// endregion

	// region: sum_of_odds_04
	#[test]
	fn sum_of_odds_04() {
		const S: &str = "12345";
		const EXPECTED: u32 = 12345;

		assert_eq!(sum_of_odds(S), EXPECTED);
	}
	// endregion

	// region: sum_of_odds_05
	#[test]
	fn sum_of_odds_05() {
		const S: &str = "-1 +2 +3 -4 -5 +6 +7";
		const EXPECTED: u32 = 10;

		assert_eq!(sum_of_odds(S), EXPECTED);
	}
	// endregion

	// region: sum_of_odds_06
	#[test]
	fn sum_of_odds_06() {
		const S: &str = "2147483645     2147483649  1";
		const EXPECTED: u32 = 4294967295;

		assert_eq!(sum_of_odds(S), EXPECTED);
	}
	// endregion

	// region: sum_of_odds_07
	#[test]
	fn sum_of_odds_07() {
		const S: &str = "\x0900\x0a11\x0b22\x0c33\x0d44\x2055";
		const EXPECTED: u32 = 99;

		assert_eq!(sum_of_odds(S), EXPECTED);
	}
	// endregion

	// region: sum_of_odds_08
	#[test]
	fn sum_of_odds_08() {
		const S: &str =
			"21 errors occured at line 1\nDon't panic, and retry in about 5 to 15 minutes";
		const EXPECTED: u32 = 42;

		assert_eq!(sum_of_odds(S), EXPECTED);
	}
	// endregion

	// region: create_pairs_00
	#[test]
	fn create_pairs_00() {
		const S: &str = "";
		let expected: Vec<(&str, A)> = vec![];

		assert_eq!(create_pairs::<A>(S), expected);
	}
	// endregion

	// region: create_pairs_01
	#[test]
	fn create_pairs_01() {
		const S: &str = "
foo:\n\
:bar\n\
muf:diz";
		let expected: Vec<(&str, B)> = vec![
			("foo", B { a: "".to_string() }),
			("", B { a: "bar".to_string() }),
			("muf", B { a: "diz".to_string() }),
		];

		assert_eq!(create_pairs::<B>(S), expected);
	}
	// endregion

	// region: create_pairs_02
	#[test]
	fn create_pairs_02() {
		const S: &str = "\
\t\nWelcome \r :\x0b42 , -12  \n\
to  \t  \n\
summoners   :3 ,5\x0b  \r \n\
  \x0crift:  \t+218, 118\n\
Good luck!:-1,+128\n\
Have fun!:255 -0\n";
		let expected: Vec<(&str, C)> = vec![
			("Welcome", C { a: 42, b: -12 }),
			("summoners", C { a: 3, b: 5 }),
			("rift", C { a: 218, b: 118 }),
		];

		assert_eq!(create_pairs::<C>(S), expected);
	}
	// endregion
}
