fn main() {
	for n in 1u8..=100u8 {
		println!(
			"{}",
			match (n % 3, n % 5, n % 11) {
				(0, 0, _) => "fizzbuzz",
				(0, _, _) => "fizz",
				(_, 0, _) => "buzz",
				(_, _, 3) => "FIZZ",
				(_, _, 5) => "BUZZ",
				(_, _, _) => {
					println!("{}", n);
					continue;
				}
			}
		)
	}
}
