use std::collections::BTreeSet;

const STARTING_PRIMES: [u32; 4] = [2, 3, 5, 7];
const SIEVE_COUNT: usize = 42;

pub struct Prime {
	n: u32,
	primes: Vec<u32>,
	sieve: BTreeSet<u32>, // See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
	chunk_first: u32,
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

		let primes: Vec<u32> = STARTING_PRIMES.to_vec();
		let sieve: BTreeSet<u32> = BTreeSet::new();
		let chunk_first: u32 = match STARTING_PRIMES.len() {
			0 => 2,
			len => STARTING_PRIMES[len - 1] + 1,
		};

		Prime { n, primes, sieve, chunk_first }
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

		let remaining_numbers: u32 = u32::MAX - self.chunk_first;
		let chunk_count: usize = match remaining_numbers {
			0 => return None,
			1..=SIEVE_COUNT_U32 => remaining_numbers as usize,
			_ => SIEVE_COUNT,
		};

		self.sieve.extend(self.chunk_first..self.chunk_first + chunk_count as u32);
		self.chunk_first += chunk_count as u32;

		Some(())
	}

	/// Removes the non-prime numbers from the sieve.
	/// The non-prime numbers are found by removing the multiples of the prime numbers
	/// that are already in `self.primes`, and then removing the multiples of the remaining
	/// numbers in the sieve from the sieve. (Yes, it sounds like an Inception)
	fn remove_non_prime_from_sieve(self: &mut Self) {
		for prime in self.primes.iter() {
			let mut multiple: u32 =
				*self.sieve.first().unwrap() - *self.sieve.first().unwrap() % *prime + *prime;

			while multiple <= *self.sieve.last().unwrap() {
				self.sieve.remove(&multiple);
				multiple = match multiple.checked_add(*prime) {
					Some(multiple) => multiple,
					None => break,
				}
			}
		}

		// TODO: Remove from the sieve the multiples of the remaining numbers in the sieve.
		// Hint: You can use the same logic as above. (but you may encounter some borrow checker issues)

		// for prime in self.sieve.iter() {
		// 	let mut multiple: u32 = match prime.checked_pow(2) {
		// 		Some(multiple) => multiple,
		// 		None => break,
		// 	};

		// 	while multiple <= *self.sieve.last().unwrap() {
		// 		self.sieve.remove(&multiple);
		// 		multiple = match multiple.checked_add(*prime) {
		// 			Some(multiple) => multiple,
		// 			None => break,
		// 		}
		// 	}
		// }
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
				while self.sieve.is_empty() {
					self.fill_sieve_with_next_chunk()?;
					self.remove_non_prime_from_sieve();
				}

				self.primes.push(self.sieve.pop_first().unwrap());
			}

			next_prime = *self.primes.last().unwrap();

			// TODO: Find and append the next prime number to `self.primes` only once,
			// and give `self.n` its value.
			if self.sieve.is_empty() {
				self.fill_sieve_with_next_chunk();
				self.remove_non_prime_from_sieve();
			}
			if let Some(first) = self.sieve.pop_first() {
				self.primes.push(first);
			}
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
