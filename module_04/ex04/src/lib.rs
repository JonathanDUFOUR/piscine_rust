use std::collections::BTreeSet;

const SIEVE_COUNT: usize = 42;

pub struct Prime {
	n: u32,
	primes: Vec<u32>,
	sieve: BTreeSet<u32>, // See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
}

impl Prime {
	/// Creates a new Prime iterator instance and initializes its attributes.
	/// The newly created Prime iterator instance is used to get the prime numbers
	/// starting at `n`, generating the next one at each iteration.
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
	/// let mut prime: Prime = Prime::new(0);
	/// ```
	pub fn new(n: u32) -> Prime {
		const MAX_SIEVE_COUNT: usize = u32::MAX as usize;

		match SIEVE_COUNT {
			1..=MAX_SIEVE_COUNT => (),
			_ => panic!("SIEVE_COUNT must be in [1, {MAX_SIEVE_COUNT}]."),
		}

		Prime { n, primes: vec![2, 3, 5, 7], sieve: BTreeSet::new() }
	}

	/// Fills the sieve with the next chunk of numbers.
	/// The next chunk of numbers is composed by the `SIEVE_COUNT` numbers that directly
	/// follow the last number of `self.primes`.
	///
	/// # Return
	/// * `Some(())` - The sieve has been filled with the next chunk of numbers.
	/// * `None` - There is no next chunk of numbers to fill the sieve with.
	fn fill_sieve_with_next_chunk(self: &mut Self) -> Option<()> {
		const SIEVE_COUNT_U32: u32 = SIEVE_COUNT as u32;

		let primes_last: u32 = *self.primes.last().unwrap();
		let remaining_numbers_count: u32 = u32::MAX - primes_last;
		let chunk_count: usize = match remaining_numbers_count {
			0 => return None,
			1..=SIEVE_COUNT_U32 => remaining_numbers_count as usize,
			_ => SIEVE_COUNT,
		};

		self.sieve.extend(primes_last..primes_last + chunk_count as u32);

		Some(())
	}

	/// Removes the non-prime numbers from the sieve.
	/// The non-prime numbers are found by removing the multiples of the prime numbers
	/// that are already in `self.primes`.
	fn remove_non_prime_from_sieve(self: &mut Self) {
		// TODO
	}
}

impl Iterator for Prime {
	type Item = u32;

	/// Generates the next prime number.
	///
	/// # Return
	/// * `Some(n)` - The next prime number.
	/// * `None` - There is no next prime number.
	///
	/// # Example
	/// ```
	/// use ex04::Prime;
	///
	/// let mut prime: Prime = Prime::new(0);
	///
	/// assert_eq!(prime.next(), Some(2));
	/// assert_eq!(prime.next(), Some(3));
	/// assert_eq!(prime.next(), Some(5));
	/// assert_eq!(prime.next(), Some(7));
	/// assert_eq!(prime.next(), Some(11));
	/// ```
	fn next(self: &mut Self) -> Option<Self::Item> {
		let next_prime: u32;

		if self.n < *self.primes.last().unwrap() {
			next_prime = self.primes[lower_bound(&self.primes, self.n).unwrap()];
			self.n = self.primes[upper_bound(&self.primes, self.n).unwrap()];
		} else {
			while self.n > *self.primes.last().unwrap() {
				if self.sieve.is_empty() {
					self.fill_sieve_with_next_chunk()?;
					self.remove_non_prime_from_sieve();
				}

				self.primes.push(self.sieve.pop_first().unwrap());
			}

			next_prime = *self.primes.last().unwrap();
			// TODO: Find and append the next prime number to `self.primes` only once,
			// and give `self.n` its value.
		}

		Some(next_prime)
	}
}

/// Searches in `v` for the first element that is __greater or equal__ to `n`.
/// It is assumed that `v` is sorted in ascending order.
///
/// # Parameters
/// * `v` - The vector to search in.
/// * `n` - The number to search the lower bound for.
///
/// # Return
/// * `Some(i)` - The index of the first element in `v` that is greater or equal to `n`.
/// * `None` - There is no element in `v` that is greater or equal to `n`.
fn lower_bound(v: &Vec<u32>, n: u32) -> Option<usize> {
	if v.is_empty() || *v.last().unwrap() < n {
		return None;
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

	Some(left)
}

/// Searches in `v` for the first element that is __strictly greater__ than `n`.
/// It is assumed that `v` is sorted in ascending order.
///
/// # Parameters
/// * `v` - The vector to search in.
/// * `n` - The number to search the upper bound for.
///
/// # Return
/// * `Some(i)` - The index of the first element in `v` that is strictly greater than `n`.
/// * `None` - There is no element in `v` that is strictly greater than `n`.
fn upper_bound(v: &Vec<u32>, n: u32) -> Option<usize> {
	match lower_bound(v, n) {
		Some(i) => {
			if v[i] == n {
				if i + 1 < v.len() {
					Some(i + 1)
				} else {
					None
				}
			} else {
				Some(i)
			}
		}
		None => None,
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
