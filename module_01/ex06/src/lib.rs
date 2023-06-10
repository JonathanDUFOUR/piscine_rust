fn __are_digits_only(n: &[u8]) -> bool {
	for &c in n {
		if !c.is_ascii_digit() {
			return false;
		}
	}
	return true;
}

fn __most_significant_digit(n: &[u8]) -> usize {
	for i in 0..n.len() {
		if n[i] != b'0' {
			return i;
		}
	}
	return n.len() - 1;
}

/// Add two big numbers.
///
/// # Arguments
///
/// * `a` - The first big number to add.
/// * `b` - The second big number to add.
///
/// # Panics
///
/// This function panics if the input contains anything else than digits.
///
/// # Examples
///
/// ```
/// use ex06::big_add;
///
/// assert_eq!(big_add(b"000111222333444555666777888999101010111111", b"031"), b"111222333444555666777888999101010111142");
/// ```
pub fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
	assert!(!a.is_empty() && !b.is_empty(), "Empty input");
	assert!(
		__are_digits_only(a) && __are_digits_only(b),
		"Input contains non-digits"
	);

	let mut sum: u8;
	let mut carry: u8 = 0;
	let mut result: Vec<u8> = Vec::new();
	let mut lsd_a: usize = a.len() - 1; // least significant digit in a
	let mut lsd_b: usize = b.len() - 1; // least significant digit in b
	let msd_a: usize = __most_significant_digit(a); // most significant digit in a
	let msd_b: usize = __most_significant_digit(b); // most significant digit in b

	if lsd_a - msd_a < lsd_b - msd_b {
		while msd_a < lsd_a {
			sum = a[lsd_a] + b[lsd_b] + carry - 2 * b'0';
			if sum > 9 {
				carry = 1;
				sum -= 10;
			} else {
				carry = 0;
			}
			result.push(sum + b'0');
			lsd_a -= 1;
			lsd_b -= 1;
		}
		sum = a[lsd_a] + b[lsd_b] + carry - 2 * b'0';
		if sum > 9 {
			carry = 1;
			sum -= 10;
		} else {
			carry = 0;
		}
		result.push(sum + b'0');
		lsd_b -= 1;
		while msd_b < lsd_b {
			sum = b[lsd_b] + carry - b'0';
			if sum > 9 {
				carry = 1;
				sum -= 10;
			} else {
				carry = 0;
			}
			result.push(sum + b'0');
			lsd_b -= 1;
		}
		sum = b[lsd_b] + carry - b'0';
		if sum > 9 {
			carry = 1;
			sum -= 10;
		} else {
			carry = 0;
		}
		result.push(sum + b'0');
		if carry == 1 {
			result.push(b'1');
		}
	} else {
		while msd_b < lsd_b {
			sum = a[lsd_a] + b[lsd_b] + carry - 2 * b'0';
			if sum > 9 {
				carry = 1;
				sum -= 10;
			} else {
				carry = 0;
			}
			result.push(sum + b'0');
			lsd_a -= 1;
			lsd_b -= 1;
		}
		sum = a[lsd_a] + b[lsd_b] + carry - 2 * b'0';
		if sum > 9 {
			carry = 1;
			sum -= 10;
		} else {
			carry = 0;
		}
		result.push(sum + b'0');
		if msd_a < lsd_a {
			lsd_a -= 1;
			while msd_a < lsd_a {
				sum = a[lsd_a] + carry - b'0';
				if sum > 9 {
					carry = 1;
					sum -= 10;
				} else {
					carry = 0;
				}
				result.push(sum + b'0');
				lsd_a -= 1;
			}
			sum = a[lsd_a] + carry - b'0';
			if sum > 9 {
				carry = 1;
				sum -= 10;
			} else {
				carry = 0;
			}
			result.push(sum + b'0');
		}
		if carry == 1 {
			result.push(b'1');
		}
	}
	result.reverse();
	return result;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic(expected = "Empty input")]
	fn big_add_01() {
		big_add(b"", b"0");
	}

	#[test]
	#[should_panic(expected = "Empty input")]
	fn big_add_02() {
		big_add(b"0", b"");
	}

	#[test]
	#[should_panic(expected = "Input contains non-digits")]
	fn big_add_03() {
		big_add(b"0", b"0x2a");
	}

	#[test]
	#[should_panic(expected = "Input contains non-digits")]
	fn big_add_04() {
		big_add(b"0x2a", b"0");
	}

	#[test]
	fn big_add_05() {
		assert_eq!(big_add(b"0", b"0"), b"0");
	}

	#[test]
	fn big_add_06() {
		assert_eq!(big_add(b"0", b"1"), b"1");
	}

	#[test]
	fn big_add_07() {
		assert_eq!(big_add(b"1", b"0"), b"1");
	}

	#[test]
	fn big_add_08() {
		assert_eq!(big_add(b"1", b"1"), b"2");
	}

	#[test]
	fn big_add_09() {
		assert_eq!(big_add(b"1", b"2"), b"3");
	}

	#[test]
	fn big_add_10() {
		assert_eq!(big_add(b"2", b"1"), b"3");
	}

	#[test]
	fn big_add_11() {
		assert_eq!(big_add(b"1", b"9"), b"10");
	}

	#[test]
	fn big_add_12() {
		assert_eq!(big_add(b"9", b"1"), b"10");
	}

	#[test]
	fn big_add_13() {
		assert_eq!(big_add(b"9", b"9"), b"18");
	}

	#[test]
	fn big_add_14() {
		assert_eq!(
			big_add(
				b"99999999999999999999999999999999999999999999999999999999999999999999999999999999",
				b"1"
			),
			b"100000000000000000000000000000000000000000000000000000000000000000000000000000000"
		);
	}

	#[test]
	fn big_add_15() {
		assert_eq!(big_add(b"00000001234", b"00005678"), b"6912");
	}
}
