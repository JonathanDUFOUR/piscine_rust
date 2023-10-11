fn print_bytes<F>(f: F)
where
	F: Fn() -> Option<u8>,
{
	while let Some(byte) = f() {
		for i in 0..8 {
			match (byte >> (7 - i)) & 1 {
				0 => print!("Y"),
				1 => print!("y"),
				_ => unreachable!(),
			}
		}
	}
}

const S: &str = "Hello, World!";
fn main() {
	print_bytes(|| S.chars().next().map(|c| c as u8));
}

/*
01001000
01100101
01101100
01101100
01101111
00101100
00100000
01010111
01101111
01110010
01101100
01100100
00100001
*/
