pub struct Prime {
	n: u32,
	unit: u8,
	primes: Vec<u32>,
}

impl Prime {
	/// Creates a new Prime iterator instance and initializes its attributes.
	/// The newly created Prime iterator instance will generate every prime number
	/// that fits a 32-bits unsigned integer starting at a given number.
	///
	/// # Paramters
	/// * `n` - The starting number.
	///
	/// # Return
	/// The newly created Prime iterator instance.
	///
	/// # Example
	/// ```
	/// use ex04::Prime;
	///
	/// let mut prime: Prime = Prime::new();
	/// ```
	pub fn new(n: u32) -> Prime {
		Prime { n, unit: 7, primes: vec![2, 3, 5, 7] }
	}
}

impl Iterator for Prime {
	type Item = u32;

	/// Generates the next prime number.
	///
	/// # Return
	/// The next prime number.
	///
	/// # Example
	/// ```
	/// use ex04::Prime;
	///
	/// let mut prime: Prime = Prime::new();
	///
	/// assert_eq!(prime.next(), Some(2));
	/// assert_eq!(prime.next(), Some(3));
	/// assert_eq!(prime.next(), Some(5));
	/// assert_eq!(prime.next(), Some(7));
	/// assert_eq!(prime.next(), Some(11));
	/// ```
	fn next(self: &mut Self) -> Option<Self::Item> {
		let next_prime: u32;
		let last_prime: u32 = *self.primes.last().unwrap();

		if self.n < last_prime {
			next_prime = lower_bound(&self.primes, self.n);
			self.n = upper_bound(&self.primes, self.n);
			Some(next_prime)
		} else if self.n == last_prime {
			// TODO: Give `next_prime` the value of the last element of `self.primes`.
			// Then, append the next prime number to `self.primes` only once,
			// and give `self.n` its value.
			None
		} else {
			// TODO: Append the next prime numbers to `self.primes`
			// until the last element of `self.primes` is greater or equal to `self.n`.
			// Once done, give `next_prime` the value of the last element of `self.primes`.
			// Then, append the next prime number to `self.primes` only once,
			// and give `self.n` its value.
			// Finally, return `Some(next_prime)`.
			None
		}
	}
}

/// Searches in `v` for the first element that is __greater or equal__ to `n`.
/// It is assumed that `v` is sorted in ascending order,
/// and that `n` is lower or equal to the last element of `v`.
///
/// # Parameters
/// * `v` - The vector to search in.
/// * `n` - The value to search the lower bound for.
///
/// # Return
/// The value of the first element in `v` that is greater or equal to `n`.
///
/// # Panic
/// * `v` is empty.
/// * `n` is strictly greater than the last element of `v`.
fn lower_bound(v: &Vec<u32>, n: u32) -> u32 {
	if v.is_empty() {
		panic!("The given vector is empty.");
	}
	if *v.last().unwrap() < n {
		panic!("{n} is greater than the last element of the given vector.");
	}

	let mut left: usize = 0;
	let mut right: usize = v.len();

	while left < right {
		let middle: usize = left + (right - left) / 2;

		if v[middle] < n {
			left = middle + 1;
		} else {
			right = middle;
		}
	}

	v[left]
}

/// Searches in `v` for the first element that is strictly greater__ than `n`.
/// It is assumed that `v` is sorted in ascending order,
/// and that `n` is strictly lower than the last element of `v`.
///
/// # Parameters
/// * `v` - The vector to search in.
/// * `n` - The value to search the lower bound for.
///
/// # Return
/// The value of the first element in `v` that is strictly greater than `n`.
///
/// # Panic
/// * `v` is empty.
/// * `n` is greater or equal to the last element of `v`.
fn upper_bound(v: &Vec<u32>, n: u32) -> u32 {
	if v.is_empty() {
		panic!("The given vector is empty.");
	}
	if *v.last().unwrap() <= n {
		panic!("{n} is greater than the last element of the given vector.");
	}

	let mut left: usize = 0;
	let mut right: usize = v.len();

	while left < right {
		let middle: usize = left + (right - left) / 2;

		if v[middle] <= n {
			left = middle + 1;
		} else {
			right = middle;
		}
	}

	v[left]
}

#[cfg(test)]
mod tests {
	use super::*;
}
