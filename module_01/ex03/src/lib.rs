/// Searches for the largest subslice of a slice that contains all and only specified values.
///
/// # Parameters
/// * `haystack` - The slice to search in.
/// * `needle` - The slice of values to search for.
///
/// # Return
/// The largest subslice of `haystack` that contains all and only values in `needles`.
///
/// # Example
/// ```
/// use ex03::largest_group;
///
/// assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
/// ```
pub fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32] {
	let mut best_so_far: &[u32] = &[];
	let mut candidate: &[u32];
	let mut i0: usize = 0;
	let mut i1: usize;

	while i0 < haystack.len() {
		while i0 < haystack.len() && !needle.contains(&haystack[i0]) {
			i0 += 1;
		}
		i1 = i0;
		while i1 < haystack.len() && needle.contains(&haystack[i1]) {
			i1 += 1;
		}
		candidate = &haystack[i0..i1];
		if candidate.len() > best_so_far.len() {
			i0 = 0;
			while i0 < needle.len() {
				if !candidate.contains(&needle[i0]) {
					break;
				}
				i0 += 1;
			}
			if i0 == needle.len() {
				best_so_far = candidate;
			}
		}
		i0 = i1;
	}
	return best_so_far;
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn largest_group_00() {
		let haystack: [u32; 0] = [];
		let result: &[u32] = {
			let needle: [u32; 0] = [];

			largest_group(&haystack, &needle)
		};

		assert_eq!(result, &[]);
	}

	#[test]
	fn largest_group_01() {
		let haystack: [u32; 3] = [1, 2, 3];
		let result: &[u32] = {
			let needle: [u32; 0] = [];

			largest_group(&haystack, &needle)
		};

		assert_eq!(result, &[]);
	}

	#[test]
	fn largest_group_02() {
		let haystack: [u32; 0] = [];
		let result: &[u32] = {
			let needle: [u32; 3] = [1, 2, 3];

			largest_group(&haystack, &needle)
		};

		assert_eq!(result, &[]);
	}

	#[test]
	fn largest_group_03() {
		let haystack: [u32; 3] = [1, 2, 3];
		let result: &[u32] = {
			let needle: [u32; 3] = [4, 5, 6];

			largest_group(&haystack, &needle)
		};

		assert_eq!(result, &[]);
	}

	#[test]
	fn largest_group_04() {
		let haystack: [u32; 3] = [1, 2, 3];
		let result: &[u32] = {
			let needle: [u32; 3] = [1, 2, 3];

			largest_group(&haystack, &needle)
		};

		assert_eq!(result, &[1, 2, 3]);
	}

	#[test]
	fn largest_group_05() {
		let haystack: [u32; 5] = [1, 2, 3, 2, 1];
		let result: &[u32] = {
			let needle: [u32; 2] = [1, 2];

			largest_group(&haystack, &needle)
		};

		assert_eq!(result, &[1, 2]);
	}

	#[test]
	fn largest_group_06() {
		let haystack: [u32; 6] = [1, 2, 3, 3, 2, 1];
		let result: &[u32] = {
			let needle: [u32; 2] = [2, 3];

			largest_group(&haystack, &needle)
		};

		assert_eq!(result, &[2, 3, 3, 2]);
	}

	#[test]
	fn largest_group_07() {
		let haystack: [u32; 9] = [1, 2, 3, 4, 3, 2, 1, 2, 3];
		let result: &[u32] = {
			let needle: [u32; 3] = [1, 2, 3];

			largest_group(&haystack, &needle)
		};

		assert_eq!(result, &[3, 2, 1, 2, 3]);
	}

	#[test]
	fn largest_group_08() {
		let haystack: [u32; 9] = [1, 1, 2, 3, 2, 2, 1, 2, 3];
		let result: &[u32] = {
			let needle: [u32; 3] = [1, 2, 1];

			largest_group(&haystack, &needle)
		};

		assert_eq!(result, &[2, 2, 1, 2]);
	}
}
