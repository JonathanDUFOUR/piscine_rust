#![deny(warnings)]

fn print_bytes(s: &str) {
	for c in s.bytes() {
		println!("{}", c);
	}
}

fn main() {
	let tutu: &str = "Déjà Vu\n";
	print_bytes(tutu);
}
