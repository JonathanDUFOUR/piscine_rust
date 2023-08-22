fn min(a: i32, b: i32) -> i32 {
	if a < b {
		a
	} else {
		b
	}
}

fn main() {
	println!(
		" first call: {}\nsecond call: {}\n third call: {}\n",
		min(-0x80000000, 0),
		min(-1, -1),
		min(-2147483648, 2147483647),
	);
}
