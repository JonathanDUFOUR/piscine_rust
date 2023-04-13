fn main() {
	let number_to_guess:i32 = ftkit::random_number(1..=100);

	println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
	loop {
		let guess: i32 = ftkit::read_number();

		match guess.cmp(&number_to_guess) {
			std::cmp::Ordering::Less => println!("This student might not be as smart as I was told. This answer is obviously too weak."),
			std::cmp::Ordering::Greater => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
			std::cmp::Ordering::Equal => break,
		}
	}
	println!("That is right! The secret was indeed the number {number_to_guess}, which you have brilliantly discovered!");
}
