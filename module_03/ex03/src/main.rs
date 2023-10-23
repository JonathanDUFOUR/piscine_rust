trait FortyTwo {
	fn forty_two() -> Self;
}

macro_rules! impl_forty_two_fot_numeric_types {
	($($type:ty)*) => {
		$(
			impl FortyTwo for $type {
				fn forty_two() -> Self {
					42 as $type
				}
			}
		)*
	}
}

impl_forty_two_fot_numeric_types!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

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
	print_forty_two::<u16>();
	print_forty_two::<u32>();
	print_forty_two::<u64>();
	print_forty_two::<u128>();
	print_forty_two::<usize>();
	print_forty_two::<i8>();
	print_forty_two::<i16>();
	print_forty_two::<i32>();
	print_forty_two::<i64>();
	print_forty_two::<i128>();
	print_forty_two::<isize>();
	print_forty_two::<f32>();
	print_forty_two::<f64>();
	print_forty_two::<String>();
}
