/// Removes every duplicate element from a vector.
///
/// # Parameters
/// * `v` - The vector of integers to remove the duplicate elements from.
///
/// # Example
/// ```
/// use ex05::deduplicate;
///
/// let mut v = vec![1, 2, 2, 3, 2, 4, 3];
/// deduplicate(&mut v);
/// assert_eq!(v, [1, 2, 3, 4]);
/// ```
pub fn deduplicate(v: &mut Vec<i32>) {
	let mut i: usize;
	let mut j: usize;

	i = 0;
	while i < v.len() {
		j = i + 1;
		while j < v.len() {
			if v[i] == v[j] {
				v.remove(j);
			} else {
				j += 1;
			}
		}
		i += 1;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn deduplicate_00() {
		let mut v: Vec<i32> = vec![];

		deduplicate(&mut v);
		assert_eq!(v, []);
	}

	#[test]
	fn deduplicate_01() {
		let mut v: Vec<i32> = vec![1];

		deduplicate(&mut v);
		assert_eq!(v, [1]);
	}

	#[test]
	fn deduplicate_02() {
		let mut v: Vec<i32> = vec![1, 2, 3, 4];

		deduplicate(&mut v);
		assert_eq!(v, [1, 2, 3, 4]);
	}

	#[test]
	fn deduplicate_03() {
		let mut v: Vec<i32> = vec![4, 3, 4, 2, 3, 2, 1];

		deduplicate(&mut v);
		assert_eq!(v, [4, 3, 2, 1]);
	}

	#[test]
	fn deduplicate_04() {
		let mut v: Vec<i32> = vec![
			42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
			42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
		];

		deduplicate(&mut v);
		assert_eq!(v, [42]);
	}

	#[test]
	fn deduplicate_05() {
		let mut v: Vec<i32> = vec![i32::MIN, i32::MAX, 0, 0, 0, 7, i32::MAX, i32::MIN];

		deduplicate(&mut v);
		assert_eq!(v, [i32::MIN, i32::MAX, 0, 7]);
	}
}
