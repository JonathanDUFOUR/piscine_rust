trait FortyTwo {
	fn forty_two() -> Self;
}

impl FortyTwo for u8 {
	fn forty_two() -> Self {
		42u8
	}
}

impl FortyTwo for i8 {
	fn forty_two() -> Self {
		42i8
	}
}

impl FortyTwo for u16 {
	fn forty_two() -> Self {
		42u16
	}
}

impl FortyTwo for i16 {
	fn forty_two() -> Self {
		42i16
	}
}

impl FortyTwo for u32 {
	fn forty_two() -> Self {
		42u32
	}
}

impl FortyTwo for i32 {
	fn forty_two() -> Self {
		42i32
	}
}

impl FortyTwo for u64 {
	fn forty_two() -> Self {
		42u64
	}
}

impl FortyTwo for i64 {
	fn forty_two() -> Self {
		42i64
	}
}

impl FortyTwo for u128 {
	fn forty_two() -> Self {
		42u128
	}
}

impl FortyTwo for i128 {
	fn forty_two() -> Self {
		42i128
	}
}

impl FortyTwo for usize {
	fn forty_two() -> Self {
		42usize
	}
}

impl FortyTwo for isize {
	fn forty_two() -> Self {
		42isize
	}
}

impl FortyTwo for f32 {
	fn forty_two() -> Self {
		42f32
	}
}

impl FortyTwo for f64 {
	fn forty_two() -> Self {
		42f64
	}
}

impl FortyTwo for String {
	fn forty_two() -> Self {
		"42".to_string()
	}
}

fn print_forty_two<T: std::fmt::Debug + FortyTwo>() {
	println!("{:?}", T::forty_two())
}

fn main() {
	print_forty_two::<u8>();
	print_forty_two::<i8>();
	print_forty_two::<u16>();
	print_forty_two::<i16>();
	print_forty_two::<u32>();
	print_forty_two::<i32>();
	print_forty_two::<u64>();
	print_forty_two::<i64>();
	print_forty_two::<u128>();
	print_forty_two::<i128>();
	print_forty_two::<usize>();
	print_forty_two::<isize>();
	print_forty_two::<f32>();
	print_forty_two::<f64>();
	print_forty_two::<String>();
}
