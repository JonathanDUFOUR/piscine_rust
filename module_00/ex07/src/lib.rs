/// Searches for the first occurence of a character in a string.
///
/// # Parameters
/// * `haystack` - The string to search in.
/// * `needle` - The character to search for.
/// * `i` - The index of the first occurence of the character in the string.
///
/// # Return
/// * `true` - `needle` was found in `haystack`.
/// * `false` - `needle` was not found in `haystack`.
fn strchr(haystack: &[u8], needle: u8, i: &mut usize) -> bool {
	let mut j: usize = 0;

	while j < haystack.len() {
		if haystack[j] == needle {
			*i = j;
			return true;
		}
		j += 1;
	}
	return false;
}

/// Searches for the last occurence of a character in a string.
///
/// # Parameters
/// * `haystack` - The string to search in.
/// * `needle` - The character to search for.
/// * `i` - The index of the last occurence of the character in the string.
///
/// # Returns
/// * `true` - `needle` was found in `haystack`.
/// * `false` - `needle` was not found in `haystack`.
fn strrchr(haystack: &[u8], needle: u8, i: &mut usize) -> bool {
	let mut j: usize = haystack.len();

	while j != 0 {
		j -= 1;
		if haystack[j] == needle {
			*i = j;
			return true;
		}
	}
	return false;
}

/// Searches for the first occurence of a substring in a string.
///
/// # Parameters
/// * `haystack` - The string to search in.
/// * `needle` - The string to search for.
/// * `i` - The index of the first character of the first occurence of the needle in the haystack.
///
/// # Returns
/// * `true` - `needle` was found in `haystack`.
/// * `false` - `needle` was not found in `haystack`.
fn strstr(haystack: &[u8], needle: &[u8], i: &mut usize) -> bool {
	if needle.is_empty() {
		*i = 0;
		return true;
	}

	if needle.len() > haystack.len() {
		return false;
	}

	let mut j: usize = 0;
	let jumps: [usize; 256] = {
		let mut arr: [usize; 256] = [needle.len(); 256];

		for j in 0..needle.len() - 1 {
			arr[needle[j] as usize] = needle.len() - j - 1;
		}

		arr
	};

	while j <= (haystack.len() - needle.len()) {
		if haystack[j + needle.len() - 1] == needle[needle.len() - 1]
			&& haystack[j..j + needle.len() - 1] == needle[..needle.len() - 1]
		{
			*i = j;
			return true;
		}
		j += jumps[haystack[j + needle.len() - 1] as usize];
	}
	return false;
}

/// Checks whether a string matches a pattern.
///
/// # Parameters
/// * `query` - The string to check.
/// * `pattern` - The pattern to check against.
///
/// # Returns
/// * `true` - The string matches the pattern.
/// * `false` - The string does not match the pattern.
///
/// # Example
/// ```
/// use ex07::strpcmp;
///
/// assert_eq!(strpcmp(b"Hello World!", b"He*o*rld*"), true);
/// ```
pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
	let mut i0: usize = 0;

	if !strrchr(pattern, b'*', &mut i0) && query[..] != pattern[..] {
		return false;
	}
	i0 += 1;
	if i0 < pattern.len() && query[query.len() - (pattern.len() - i0)..] != pattern[i0..] {
		return false;
	}
	i0 = 0;
	strchr(pattern, b'*', &mut i0);
	if query[..i0] != pattern[..i0] {
		return false;
	}

	let mut i1: usize = 0;
	let mut i2: usize;
	let mut i3: usize;

	while i0 < pattern.len() {
		while i0 < pattern.len() && pattern[i0] == b'*' {
			i0 += 1;
		}
		i2 = i0;
		while i2 < pattern.len() && pattern[i2] != b'*' {
			i2 += 1;
		}
		i3 = i1;
		if !strstr(&query[i1..], &pattern[i0..i2], &mut i1) {
			return false;
		}
		i1 += i3;
		i1 += i2 - i0;
		i0 = i2;
	}

	true
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn strchr_00() {
		let mut i: usize = 42;

		assert_eq!(strchr(b"", 0, &mut i), false);
		assert_eq!(i, 42);
	}

	#[test]
	fn strchr_01() {
		let mut i: usize = 42;

		assert_eq!(strchr(b"", b'a', &mut i), false);
		assert_eq!(i, 42);
	}

	#[test]
	fn strchr_02() {
		let mut i: usize = 42;

		assert_eq!(strchr(b"a", b'a', &mut i), true);
		assert_eq!(i, 0);
	}

	#[test]
	fn strchr_03() {
		let mut i: usize = 42;

		assert_eq!(strchr(b"Hello World!", b'o', &mut i), true);
		assert_eq!(i, 4);
	}

	#[test]
	fn strchr_04() {
		let mut i: usize = 42;

		assert_eq!(strchr(b"How are you?", b'?', &mut i), true);
		assert_eq!(i, 11);
	}

	#[test]
	fn strchr_05() {
		let mut i: usize = 42;

		assert_eq!(strchr(b"Oups, I did not find it...", b'0', &mut i), false);
		assert_eq!(i, 42);
	}

	#[test]
	fn strrchr_00() {
		let mut i: usize = 42;

		assert_eq!(strrchr(b"", 0, &mut i), false);
		assert_eq!(i, 42);
	}

	#[test]
	fn strrchr_01() {
		let mut i: usize = 42;

		assert_eq!(strrchr(b"", b'a', &mut i), false);
		assert_eq!(i, 42);
	}

	#[test]
	fn strrchr_02() {
		let mut i: usize = 42;

		assert_eq!(strrchr(b"a", b'a', &mut i), true);
		assert_eq!(i, 0);
	}

	#[test]
	fn strrchr_03() {
		let mut i: usize = 42;

		assert_eq!(strrchr(b"Hello World!", b'o', &mut i), true);
		assert_eq!(i, 7);
	}

	#[test]
	fn strrchr_04() {
		let mut i: usize = 42;

		assert_eq!(strrchr(b"How are you?", b'H', &mut i), true);
		assert_eq!(i, 0);
	}

	#[test]
	fn strrchr_05() {
		let mut i: usize = 42;

		assert_eq!(strrchr(b"Oups, I did not find it...", b'0', &mut i), false);
		assert_eq!(i, 42);
	}

	#[test]
	fn strstr_00() {
		let mut i: usize = 42;

		assert_eq!(strstr(b"", b"", &mut i), true);
		assert_eq!(i, 0);
	}

	#[test]
	fn strstr_01() {
		let mut i: usize = 42;

		assert_eq!(strstr(b"", b"This is a basic needle", &mut i), false);
		assert_eq!(i, 42);
	}

	#[test]
	fn strstr_02() {
		let mut i: usize = 42;

		assert_eq!(strstr(b"This is a simple haystack", b"", &mut i), true);
		assert_eq!(i, 0);
	}

	#[test]
	fn strstr_03() {
		let mut i: usize = 42;

		assert_eq!(strstr(b"What about this one ?", b"o", &mut i), true);
		assert_eq!(i, 7)
	}

	#[test]
	fn strstr_04() {
		let mut i: usize = 42;

		assert_eq!(strstr(b"Is it still working now?", b"working", &mut i), true);
		assert_eq!(i, 12)
	}

	#[test]
	fn strstr_05() {
		let mut i: usize = 42;

		assert_eq!(strstr(b"Are you sure?...", b"sure?....", &mut i), false);
		assert_eq!(i, 42);
	}

	#[test]
	fn strstr_06() {
		let mut i: usize = 42;

		assert_eq!(strstr(b"(o)< cococorico", b"cocorico", &mut i), true);
		assert_eq!(i, 7);
	}

	#[test]
	fn strstr_07() {
		let mut i: usize = 42;

		assert_eq!(
			strstr(b"What if we look for the beginning of the string?", b"What if", &mut i),
			true
		);
		assert_eq!(i, 0);
	}

	#[test]
	fn strstr_08() {
		let mut i: usize = 42;

		assert_eq!(strstr(b"And what about the end?", b" end?", &mut i), true);
		assert_eq!(i, 18);
	}

	#[test]
	fn strpcmp_00() {
		assert_eq!(strpcmp(b"", b""), true);
	}

	#[test]
	fn strpcmp_01() {
		assert_eq!(strpcmp(b"", b"a"), false);
	}

	#[test]
	fn strpcmp_02() {
		assert_eq!(strpcmp(b"a", b""), false);
	}

	#[test]
	fn strpcmp_03() {
		assert_eq!(strpcmp(b"a", b"a"), true);
	}

	#[test]
	fn strpcmp_04() {
		assert_eq!(strpcmp(b"a", b"ba"), false);
	}

	#[test]
	fn strpcmp_05() {
		assert_eq!(strpcmp(b"a", b"ab"), false);
	}

	#[test]
	fn strpcmp_06() {
		assert_eq!(strpcmp(b"ba", b"a"), false);
	}

	#[test]
	fn strpcmp_07() {
		assert_eq!(strpcmp(b"ab", b"a"), false);
	}

	#[test]
	fn strpcmp_08() {
		assert_eq!(strpcmp(b"*", b""), false);
	}

	#[test]
	fn strpcmp_09() {
		assert_eq!(strpcmp(b"", b"*"), true);
	}

	#[test]
	fn strpcmp_10() {
		assert_eq!(strpcmp(b"*", b"*"), true);
	}

	#[test]
	fn strpcmp_11() {
		assert_eq!(strpcmp(b"a", b"*"), true);
	}

	#[test]
	fn strpcmp_12() {
		assert_eq!(strpcmp(b"*", b"a"), false);
	}

	#[test]
	fn strpcmp_13() {
		assert_eq!(strpcmp(b"a", b"*a"), true);
	}

	#[test]
	fn strpcmp_14() {
		assert_eq!(strpcmp(b"a", b"a*"), true);
	}

	#[test]
	fn strpcmp_15() {
		assert_eq!(strpcmp(b"*a", b"a"), false);
	}

	#[test]
	fn strpcmp_16() {
		assert_eq!(strpcmp(b"a*", b"a"), false);
	}

	#[test]
	fn strpcmp_17() {
		assert_eq!(strpcmp(b"ab", b"*"), true);
	}

	#[test]
	fn strpcmp_18() {
		assert_eq!(strpcmp(b"ab", b"*a"), false);
	}

	#[test]
	fn strpcmp_19() {
		assert_eq!(strpcmp(b"ab", b"a*"), true);
	}

	#[test]
	fn strpcmp_20() {
		assert_eq!(strpcmp(b"ab", b"*b"), true);
	}

	#[test]
	fn strpcmp_21() {
		assert_eq!(strpcmp(b"ab", b"b*"), false);
	}

	#[test]
	fn strpcmp_22() {
		assert_eq!(strpcmp(b"abc", b"*a"), false);
	}

	#[test]
	fn strpcmp_23() {
		assert_eq!(strpcmp(b"abc", b"a*"), true);
	}

	#[test]
	fn strpcmp_24() {
		assert_eq!(strpcmp(b"abc", b"*a*"), true);
	}

	#[test]
	fn strpcmp_25() {
		assert_eq!(strpcmp(b"abc", b"*b"), false);
	}

	#[test]
	fn strpcmp_26() {
		assert_eq!(strpcmp(b"abc", b"b*"), false);
	}

	#[test]
	fn strpcmp_27() {
		assert_eq!(strpcmp(b"abc", b"*b*"), true);
	}

	#[test]
	fn strpcmp_28() {
		assert_eq!(strpcmp(b"abc", b"*c"), true);
	}

	#[test]
	fn strpcmp_29() {
		assert_eq!(strpcmp(b"abc", b"c*"), false);
	}

	#[test]
	fn strpcmp_30() {
		assert_eq!(strpcmp(b"abc", b"*c*"), true);
	}

	#[test]
	fn strpcmp_31() {
		assert_eq!(strpcmp(b"abc", b"*ab"), false);
	}

	#[test]
	fn strpcmp_32() {
		assert_eq!(strpcmp(b"abc", b"a*b"), false);
	}

	#[test]
	fn strpcmp_33() {
		assert_eq!(strpcmp(b"abc", b"ab*"), true);
	}

	#[test]
	fn strpcmp_34() {
		assert_eq!(strpcmp(b"abc", b"*a*b"), false);
	}

	#[test]
	fn strpcmp_35() {
		assert_eq!(strpcmp(b"abc", b"*ab*"), true);
	}

	#[test]
	fn strpcmp_36() {
		assert_eq!(strpcmp(b"abc", b"a*b*"), true);
	}

	#[test]
	fn strpcmp_37() {
		assert_eq!(strpcmp(b"abc", b"*a*b*"), true);
	}

	#[test]
	fn strpcmp_38() {
		assert_eq!(strpcmp(b"abc", b"*a*c"), true);
	}

	#[test]
	fn strpcmp_39() {
		assert_eq!(strpcmp(b"abc", b"*ac*"), false);
	}

	#[test]
	fn strpcmp_40() {
		assert_eq!(strpcmp(b"abc", b"a*c*"), true);
	}

	#[test]
	fn strpcmp_41() {
		assert_eq!(strpcmp(b"abc", b"*a*c*"), true);
	}

	#[test]
	fn strpcmp_42() {
		assert_eq!(strpcmp(b"abcdefh", b"*adh"), false);
	}

	#[test]
	fn strpcmp_43() {
		assert_eq!(strpcmp(b"abcdefh", b"a*dh"), false);
	}

	#[test]
	fn strpcmp_44() {
		assert_eq!(strpcmp(b"abcdefh", b"ad*h"), false);
	}

	#[test]
	fn strpcmp_45() {
		assert_eq!(strpcmp(b"abcdefh", b"adh*"), false);
	}

	#[test]
	fn strpcmp_46() {
		assert_eq!(strpcmp(b"abcdefh", b"*a*dh"), false);
	}

	#[test]
	fn strpcmp_47() {
		assert_eq!(strpcmp(b"abcdefh", b"*ad*h"), false);
	}

	#[test]
	fn strpcmp_48() {
		assert_eq!(strpcmp(b"abcdefh", b"*adh*"), false);
	}

	#[test]
	fn strpcmp_49() {
		assert_eq!(strpcmp(b"abcdefh", b"a*d*h"), true);
	}

	#[test]
	fn strpcmp_50() {
		assert_eq!(strpcmp(b"abcdefh", b"a*dh*"), false);
	}

	#[test]
	fn strpcmp_51() {
		assert_eq!(strpcmp(b"abcdefh", b"ad*h*"), false);
	}

	#[test]
	fn strpcmp_52() {
		assert_eq!(strpcmp(b"abcdefh", b"*a*d*h"), true);
	}

	#[test]
	fn strpcmp_53() {
		assert_eq!(strpcmp(b"abcdefh", b"*a*dh*"), false);
	}

	#[test]
	fn strpcmp_54() {
		assert_eq!(strpcmp(b"abcdefh", b"*ad*h*"), false);
	}

	#[test]
	fn strpcmp_55() {
		assert_eq!(strpcmp(b"abcdefh", b"a*d*h*"), true);
	}

	#[test]
	fn strpcmp_56() {
		assert_eq!(strpcmp(b"abcdefh", b"*a*d*h*"), true);
	}

	#[test]
	fn strpcmp_57() {
		assert_eq!(strpcmp(b"Gotta catch them all", b"Go**atch** them *"), true);
	}

	#[test]
	fn strpcmp_58() {
		assert_eq!(strpcmp(b"abcabcdabc", b"abc*abc*abcd"), false);
	}

	#[test]
	fn strpcmp_59() {
		assert_eq!(strpcmp(b"abcabcdabc", b"*abcd*abcd*"), false);
	}
}
