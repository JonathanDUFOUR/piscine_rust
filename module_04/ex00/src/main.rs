/// Prints every element of a collection.
///
/// ### Type parameters
/// * `C` - The type of the collection to print.
///
/// ### Parameters
/// * `collection` - The collection to print.
fn print_all_things<C>(collection: C)
where
	C: IntoIterator,
	C::Item: std::fmt::Debug,
{
	print!("[ ");
	for item in collection.into_iter() {
		print!("{:?} ", item);
	}
	println!("]");
}

fn main() {
	print_all_things(0..=5);
	print_all_things("Hello".chars());
	print_all_things(vec![1, 3, 4, 2]);
	print_all_things([1, 2, 5, 4]);
}
