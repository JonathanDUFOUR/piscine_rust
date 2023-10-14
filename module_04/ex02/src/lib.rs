/// Parses a string as a collection of positive integers
/// and compute the sum of all odd numbers in it.
///
/// # Parameters
/// - `s`: the string to parse the positive integers from.
///
/// # Returns
/// The sum of all odd numbers in the string.
///
/// # Examples
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
	let mut sum: u32 = 0;

	for word in s.split_whitespace() {
		if let Ok(n) = word.parse::<u32>() {
			if n % 2 == 1 {
				sum += n;
			}
		}
	}

	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	// region: sum_of_odds_00
	#[test]
	fn sum_of_odds_00() {
		assert_eq!(sum_of_odds(""), 0);
	}
	// endregion

	// region: sum_of_odds_01
	#[test]
	fn sum_of_odds_01() {
		assert_eq!(sum_of_odds("0"), 0);
	}
	// endregion

	// region: sum_of_odds_02
	#[test]
	fn sum_of_odds_02() {
		assert_eq!(sum_of_odds("1"), 1);
	}
	// endregion

	// region: sum_of_odds_03
	#[test]
	fn sum_of_odds_03() {
		assert_eq!(sum_of_odds(" 1 2 3 "), 4);
	}
	// endregion

	// region: sum_of_odds_04
	#[test]
	fn sum_of_odds_04() {
		assert_eq!(sum_of_odds("12345"), 12345);
	}
	// endregion

	// region: sum_of_odds_05
	#[test]
	fn sum_of_odds_05() {
		assert_eq!(sum_of_odds("-1 +2 +3 -4 -5 +6 +7"), 10);
	}
	// endregion

	// region: sum_of_odds_06
	#[test]
	fn sum_of_odds_06() {
		assert_eq!(sum_of_odds("2147483645     2147483649  1"), 4294967295);
	}
	// endregion

	// region: sum_of_odds_07
	#[test]
	fn sum_of_odds_07() {
		assert_eq!(sum_of_odds("\x0900\x0a11\x0b22\x0c33\x0d44\x2055"), 99);
	}
	// endregion

	// region: sum_of_odds_08
	#[test]
	fn sum_of_odds_08() {
		assert_eq!(
			sum_of_odds(
				"\
21 errors occured at line 1
Don't panic, and retry in about 5 to 15 minutes"
			),
			42
		);
	}
	// endregion
}
