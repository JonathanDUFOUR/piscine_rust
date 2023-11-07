/// Gets the name of a color from its RGB value.
///
/// ### Parameters
/// * `color` - A slice of RGB values.
///
/// ### Example
/// ```
/// use ex02::color_name;
///
/// assert_eq!(color_name(&[255, 0, 0]), "pure red");
/// ```
pub fn color_name(color: &[u8; 3]) -> &'static str {
	match color {
		[0, 0, 0] => "pure black",
		[255, 255, 255] => "pure white",
		[255, 0, 0] => "pure red",
		[0, 255, 0] => "pure green",
		[0, 0, 255] => "pure blue",
		[128, 128, 128] => "perfect grey",
		[0..=30, 0..=30, 0..=30] => "almost black",
		[129..=255, 0..=127, 0..=127] => "redish",
		[0..=127, 129..=255, 0..=127] => "greenish",
		[0..=127, 0..=127, 129..=255] => "blueish",
		_ => "unknown",
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pure_black() {
		let name: &str = {
			let color: [u8; 3] = [0, 0, 0];

			color_name(&color)
		};

		assert_eq!(name, "pure black");
	}

	#[test]
	fn pure_white() {
		let name: &str = {
			let color: [u8; 3] = [255, 255, 255];

			color_name(&color)
		};

		assert_eq!(name, "pure white");
	}

	#[test]
	fn pure_red() {
		let name: &str = {
			let color: [u8; 3] = [255, 0, 0];

			color_name(&color)
		};

		assert_eq!(name, "pure red");
	}

	#[test]
	fn pure_green() {
		let name: &str = {
			let color: [u8; 3] = [0, 255, 0];

			color_name(&color)
		};

		assert_eq!(name, "pure green");
	}

	#[test]
	fn pure_blue() {
		let name: &str = {
			let color: [u8; 3] = [0, 0, 255];

			color_name(&color)
		};

		assert_eq!(name, "pure blue");
	}

	#[test]
	fn perfect_grey() {
		let name: &str = {
			let color: [u8; 3] = [128, 128, 128];

			color_name(&color)
		};

		assert_eq!(name, "perfect grey");
	}

	#[test]
	fn almost_black() {
		for blue in 1..31 {
			let name: &str = {
				let color: [u8; 3] = [0, 0, blue];

				color_name(&color)
			};

			assert_eq!(name, "almost black");
		}
		for green in 1..31 {
			for blue in 0..31 {
				let name: &str = {
					let color: [u8; 3] = [0, green, blue];

					color_name(&color)
				};

				assert_eq!(name, "almost black");
			}
		}
		for red in 1..31 {
			for green in 0..31 {
				for blue in 0..31 {
					let name: &str = {
						let color: [u8; 3] = [red, green, blue];

						color_name(&color)
					};

					assert_eq!(name, "almost black");
				}
			}
		}
	}

	#[test]
	fn redish() {
		for blue in 1..128 {
			let name: &str = {
				let color: [u8; 3] = [u8::MAX, 0, blue];

				color_name(&color)
			};

			assert_eq!(name, "redish");
		}
		for green in 1..128 {
			for blue in 0..128 {
				let name: &str = {
					let color: [u8; 3] = [u8::MAX, green, blue];

					color_name(&color)
				};

				assert_eq!(name, "redish");
			}
		}
		for red in 129..u8::MAX {
			for green in 0..128 {
				for blue in 0..128 {
					let name: &str = {
						let color: [u8; 3] = [red, green, blue];

						color_name(&color)
					};

					assert_eq!(name, "redish");
				}
			}
		}
	}

	#[test]
	fn greenish() {
		for blue in 1..128 {
			let name: &str = {
				let color: [u8; 3] = [0, u8::MAX, blue];

				color_name(&color)
			};

			assert_eq!(name, "greenish");
		}
		for red in 1..128 {
			for blue in 0..128 {
				let name: &str = {
					let color: [u8; 3] = [red, u8::MAX, blue];

					color_name(&color)
				};

				assert_eq!(name, "greenish");
			}
		}
		for red in 0..128 {
			for green in 129..u8::MAX {
				for blue in 0..128 {
					let name: &str = {
						let color: [u8; 3] = [red, green, blue];

						color_name(&color)
					};

					assert_eq!(name, "greenish");
				}
			}
		}
	}

	#[test]
	fn blueish() {
		for green in 1..128 {
			let name: &str = {
				let color: [u8; 3] = [0, green, u8::MAX];

				color_name(&color)
			};

			assert_eq!(name, "blueish");
		}
		for red in 1..128 {
			for green in 0..128 {
				let name: &str = {
					let color: [u8; 3] = [red, green, u8::MAX];

					color_name(&color)
				};

				assert_eq!(name, "blueish");
			}
		}
		for red in 0..128 {
			for green in 0..128 {
				for blue in 129..u8::MAX {
					let name: &str = {
						let color: [u8; 3] = [red, green, blue];

						color_name(&color)
					};

					assert_eq!(name, "blueish");
				}
			}
		}
	}

	#[test]
	fn unknown() {
		for a in 0..129 {
			for b in 0..129 {
				for c in 31..128 {
					let mut name: &str;

					name = {
						let color: [u8; 3] = [a, b, c];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [a, c, b];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [b, a, c];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [b, c, a];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [c, a, b];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [c, b, a];

						color_name(&color)
					};
					assert_eq!(name, "unknown");
				}
			}
		}
		for a in 0..u8::MAX {
			for b in 128..=u8::MAX {
				for c in 129..=u8::MAX {
					let mut name: &str;

					name = {
						let color: [u8; 3] = [a, b, c];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [a, c, b];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [b, a, c];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [b, c, a];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [c, a, b];

						color_name(&color)
					};
					assert_eq!(name, "unknown");

					name = {
						let color: [u8; 3] = [c, b, a];

						color_name(&color)
					};
					assert_eq!(name, "unknown");
				}
			}
		}
	}
}
