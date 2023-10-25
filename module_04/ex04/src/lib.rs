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
	pub const fn new(n: u32) -> Prime {
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
	}
/*

	n: 0
	unit: 7
	primes: [2, 3, 5, 7]

	Si n < 7 Alors
	|
	|	Si n < 5 Alors
	|	|
	|	|	Si n < 3 Alors
	|	|	|
	|	|	|	n = 3
	|	|	|	retourner 2
	|	|	|
	|	|	FinSi
	|	|
	|	|	Si n == 3 Alors
	|	|	|
	|	|	|	n = 5
	|	|	|	retourner 3
	|	|	|
	|	|	FinSi
	|	|
	|	|	n = 7
	|	|	retourner 5
	|	|
	|	Sinon
	|	|
	|	|	Si n == 5 Alors
	|	|	|
	|	|	|	n = 7
	|	|	|	retourner 5
	|	|	|
	|	|	FinSi
	|	|
	|	FinSi
	Sinon
	|
	FinSi
*/

#[cfg(test)]
mod tests {
	use super::*;
}
