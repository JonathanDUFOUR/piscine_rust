fn print_all_things<Collection>(collection: Collection)
where
	Collection: IntoIterator,
	Collection::Item: std::fmt::Debug,
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
