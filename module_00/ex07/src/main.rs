use ftkit::ARGS;
use piscine_rust_module_00_ex07::strpcmp;

fn main() {
	if ARGS.len() == 3 {
		println!(
			"strpcmp(b\"{}\", b\"{}\"): {}",
			&ARGS[1],
			&ARGS[2],
			strpcmp(ARGS[1].as_bytes(), ARGS[2].as_bytes())
		);
	}
}
