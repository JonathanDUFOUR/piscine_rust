#![deny(warnings)]

fn yes() -> ! {
	loop {
		println("y");
	}
}

fn main() {
	yes();
}
