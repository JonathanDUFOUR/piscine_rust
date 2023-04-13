/// Search for a substring (needle) in a string (haystack).
/// 
/// # Arguments
/// 
/// * `haystack` - The string to search in.
/// * `needle` - The string to search for.
/// 
/// # Returns
/// 
/// * `Some(index)` - The index of the first character of the needle in the haystack.
/// * `None` - The needle was not found in the haystack.
/// 
/// # Examples
/// 
/// ```
/// use ex07::strstr;
/// const i: usize = strstr(b"Hello World!", b"World");
/// ```
fn strstr(haystack: &[u8], needle: &[u8]) -> Option<usize> {
	if needle.is_empty() {
		return Some(0);
	}

	let haystack_len: usize = haystack.len();
	let needle_len: usize = needle.len();

	if haystack_len < needle_len {
		return None;
	}

	let mut arr: [usize; 256] = [needle_len; 256];

	let needle_last_index: usize = needle_len - 1;
	for i in 0..needle_last_index {
		arr[needle[i] as usize] = needle_last_index - i;
	}
	let mut i: usize = 0;
	let diff: usize = haystack_len - needle_len;
	while i <= diff {
		if haystack[i + needle_last_index] == needle[needle_last_index]
		&& haystack[i..i + needle_last_index] == needle[..needle_last_index] {
			return Some(i);
		}
		i += arr[haystack[i + needle_last_index] as usize];
	}
	return None;
}

#[test]
fn strstr_00() {
	assert_eq!(strstr(b"", b""), Some(0));
}

#[test]
fn strstr_01() {
	assert_eq!(strstr(b"", b"This is a basic needle"), None);
}

#[test]
fn strstr_02() {
	assert_eq!(strstr(b"This is a simple haystack", b""), Some(0));
}

#[test]
fn strstr_03() {
	assert_eq!(strstr(b"What about this one ?", b"o"), Some(7));
}

#[test]
fn strstr_04() {
	assert_eq!(strstr(b"Is it still working now?", b"working"), Some(12));
}

#[test]
fn strstr_05() {
	assert_eq!(strstr(b"Are you sure?...", b"sure?...."), None);
}

#[test]
fn strstr_06() {
	assert_eq!(strstr(b"(o)< cococorico", b"cocorico"), Some(7));
}

/// Check whether a string matches a pattern.
/// 
/// # Arguments
/// 
/// * `query` - The string to check.
/// * `pattern` - The pattern to check against.
/// 
/// # Returns
/// 
/// * `true` - The string matches the pattern.
/// * `false` - The string does not match the pattern.
/// 
/// # Examples
/// 
/// ```
/// use ex07::strpcmp;
/// const b: bool = strpcmp(b"Hello World!", b"He*o*rld");
/// ```
pub fn strpcmp(_query: &[u8], pattern: &[u8]) -> bool {
	let mut i: usize = 0;
	let mut j: usize = 0;

	while i < pattern.len() {
		if pattern[i] == b'*' {
			while pattern[i] == b'*' {
				i += 1;
			}
		}
		i += 1;
	}
	return true;
}
