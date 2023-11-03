type Inner = u32;
type BitField = u16;

/// The maximum number of bits that can be used to represent the numbers in the sieve.
const SIEVE_BITS: Inner = 42;

const SIEVE_LEN: usize = {
	const LEN: Inner = SIEVE_BITS / BitField::BITS;

	match LEN * BitField::BITS {
		SIEVE_BITS => LEN as usize,
		_ => LEN as usize + 1,
	}
};

trait IsFullOfOnes {
	/// Checks if every bit is set to 1 in the binary representation of `self`.
	///
	/// # Return
	/// * `true` - Every bit is set to 1.
	/// * `false` - There is at least 1 bit that is set to 0.
	fn is_full_of_ones(self: &Self) -> bool;
}

impl IsFullOfOnes for [BitField; SIEVE_LEN] {
	fn is_full_of_ones(self: &Self) -> bool {
		self.iter().all(|bit_field| bit_field.count_zeros() == 0)
	}
}

/// An iterator that generates prime numbers.
pub struct Prime {
	/// The number to find the next prime from.
	n: Inner,

	/// A vector that contains the prime numbers that have already been found,
	/// sorted in ascending order.
	primes: Vec<Inner>,

	/// A bit set where each bit represents a number, starting at `self.offset`.
	/// First, each bit is set to 0, then the bits representing non-prime numbers are set to 1.
	/// These non-prime numbers are found by removing the multiples
	/// of the already known prime numbers, and then removing the multiples
	/// of the remaining numbers in the sieve from the sieve.
	/// See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes for more information.
	sieve: [BitField; SIEVE_LEN],

	/// The number represented by the first bit of `self.sieve`.
	sieve_first: Inner,

	/// The effective number of bits that `self.sieve` uses.
	sieve_bits: Inner,
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
	pub fn new(n: Inner) -> Self {
		let primes: Vec<Inner> = Vec::new();
		let sieve: [BitField; SIEVE_LEN] = [0; SIEVE_LEN];
		let sieve_first: Inner = 2;
		let remaining_numbers: Inner = Inner::MAX - sieve_first + 1;
		let sieve_bits: Inner = Inner::min(SIEVE_BITS, remaining_numbers);

		Self { n, primes, sieve, sieve_first, sieve_bits }
	}

	/// Sets the bits of `self.sieve` to 0,
	/// and update `self.sieve_first` to represent the first number of the next chunk of numbers.
	///
	/// # Return
	/// * `Some(())` - The bits of the `self.sieve` have been set
	/// and `self.offset` has been updated.
	/// * `None` - The last chunk of numbers has already been computed.
	fn fill_sieve_with_next_chunk(self: &mut Self) -> Option<()> {
		if self.sieve_bits == 0 {
			return None;
		}

		self.sieve = [0; SIEVE_LEN];
		self.sieve_first += self.sieve_bits;

		let remaining_numbers: Inner = Inner::MAX - self.sieve_first + 1;

		self.sieve_bits = Inner::min(SIEVE_BITS, remaining_numbers);

		Some(())
	}

	/// Sets the bits of the non-prime numbers in the sieve to 1.
	/// The non-prime numbers are found by the multiplying the prime numbers
	/// that are already in `self.primes`, and then multiplying the remaining
	/// numbers in the sieve from the sieve. (Yes, it sounds like an Inception)
	fn remove_non_prime_from_sieve(self: &mut Self) {
		#[inline(always)]
		fn set_bit_to_one(multiple: Inner, sieve: &mut [BitField; SIEVE_LEN], sieve_first: Inner) {
			let absolute_bit_index: usize = (multiple - sieve_first) as usize;
			let relative_bit_index: usize = absolute_bit_index % BitField::BITS as usize;
			let sieve_index: usize = absolute_bit_index / BitField::BITS as usize;

			sieve[sieve_index] |= 1 << relative_bit_index;
		}

		for prime in &self.primes {
			let mut multiple: Inner = match self.sieve_first.checked_next_multiple_of(*prime) {
				Some(multiple) => multiple,
				None => continue,
			};

			while multiple - self.sieve_first < self.sieve_bits {
				set_bit_to_one(multiple, &mut self.sieve, self.sieve_first);
				multiple += *prime;
			}
		}

		for i in 0..self.sieve.len() {
			for bit in 0..BitField::BITS {
				if self.sieve[i] & (1 << bit) == 0 {
					let prime: Inner =
						self.sieve_first + i as Inner * BitField::BITS as Inner + bit as Inner;

					let mut multiple: Inner = match prime.checked_mul(prime) {
						Some(square) => square,
						None => continue,
					};

					while multiple - self.sieve_first < self.sieve_bits {
						set_bit_to_one(multiple, &mut self.sieve, self.sieve_first);
						multiple += prime;
					}
				}
			}
		}
	}

	/// Transfers the first prime number from `self.sieve` to `self.primes`.
	/// The prime number is found by searching for the first bit that is set to 0
	/// in the binary representation of `self.sieve`.
	/// It is assumed that `self.sieve` contains at least 1 of it.
	/// Once the prime number has been found, it is removed from `self.sieve`,
	/// and appended to `self.primes`.
	fn transfer_first_from_sieve_to_primes(self: &mut Self) {
		for i in 0..self.sieve.len() {
			for bit in 0..BitField::BITS {
				let mask: BitField = 1 << bit;

				if self.sieve[i] & mask == 0 {
					let prime: Inner =
						self.sieve_first + i as Inner * BitField::BITS as Inner + bit as Inner;
					self.sieve[i] |= mask;
					self.primes.push(prime);
					return;
				}
			}
		}
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
		let next_prime: Inner;

		while self.primes.is_empty() || self.n > *self.primes.last().unwrap() {
			while self.sieve.is_full_of_ones() {
				self.fill_sieve_with_next_chunk()?;
				self.remove_non_prime_from_sieve();
			}
			self.transfer_first_from_sieve_to_primes();
		}
		next_prime = *self.primes.last().unwrap();

		if self.sieve.is_full_of_ones() {
			self.fill_sieve_with_next_chunk();
			self.remove_non_prime_from_sieve();
		}
		if !self.sieve.is_full_of_ones() {
			self.transfer_first_from_sieve_to_primes();
			self.n = *self.primes.last().unwrap();
		}

		Some(next_prime)
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
