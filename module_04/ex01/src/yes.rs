fn yes<F>(f: F) -> !
where
	F: FnOnce() -> String,
{
	// TODO: Implement this function
	loop {
		println!("{}", f());
	}
}
