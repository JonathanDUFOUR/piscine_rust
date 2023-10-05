#[derive(Debug, PartialEq)]
pub enum PizzaStatus {
	Ordered,
	Cooking,
	Cooked,
	Delivering,
	Delivered,
}

impl PizzaStatus {
	/// Predict the status of a pizza that was ordered days ago.
	///
	/// # Parameters
	///
	/// * `ordered_days_ago` - The days number reprensenting how long ago the pizza was ordered.
	///
	/// # Returns
	///
	/// An instance of the predicted status.
	///
	/// # Examples
	///
	/// ```
	/// use ex02::PizzaStatus;
	///
	/// let status: PizzaStatus = PizzaStatus::from_delivery_time(42);
	///
	/// assert_eq!(status, PizzaStatus::Delivered);
	/// ```
	pub fn from_delivery_time(ordered_days_ago: u32) -> Self {
		match ordered_days_ago {
			0..=1 => PizzaStatus::Ordered,
			2..=6 => PizzaStatus::Cooking,
			7..=9 => PizzaStatus::Cooked,
			10..=16 => PizzaStatus::Delivering,
			_ => PizzaStatus::Delivered,
		}
	}

	/// Return the estimated time before the pizza is delivered, in days.
	/// The worst case is always returned.
	///
	/// # Returns
	///
	/// The estimated time before the pizza is delivered, in days.
	///
	/// # Examples
	///
	/// ```
	/// use ex02::PizzaStatus;
	///
	/// let status: PizzaStatus = PizzaStatus::Ordered;
	///
	/// assert_eq!(status.get_delivery_time_in_days(), 17);
	/// ```
	pub fn get_delivery_time_in_days(self: &Self) -> u32 {
		match self {
			PizzaStatus::Ordered => 17,
			PizzaStatus::Cooking => 15,
			PizzaStatus::Cooked => 10,
			PizzaStatus::Delivering => 7,
			PizzaStatus::Delivered => 0,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_delivery_time_00() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(0);

		assert_eq!(status, PizzaStatus::Ordered);
	}

	#[test]
	fn from_delivery_time_01() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(1);

		assert_eq!(status, PizzaStatus::Ordered);
	}

	#[test]
	fn from_delivery_time_02() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(2);

		assert_eq!(status, PizzaStatus::Cooking);
	}

	#[test]
	fn from_delivery_time_03() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(3);

		assert_eq!(status, PizzaStatus::Cooking);
	}

	#[test]
	fn from_delivery_time_04() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(4);

		assert_eq!(status, PizzaStatus::Cooking);
	}

	#[test]
	fn from_delivery_time_05() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(5);

		assert_eq!(status, PizzaStatus::Cooking);
	}

	#[test]
	fn from_delivery_time_06() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(6);

		assert_eq!(status, PizzaStatus::Cooking);
	}

	#[test]
	fn from_delivery_time_07() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(7);

		assert_eq!(status, PizzaStatus::Cooked);
	}

	#[test]
	fn from_delivery_time_08() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(8);

		assert_eq!(status, PizzaStatus::Cooked);
	}

	#[test]
	fn from_delivery_time_09() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(9);

		assert_eq!(status, PizzaStatus::Cooked);
	}

	#[test]
	fn from_delivery_time_10() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(10);

		assert_eq!(status, PizzaStatus::Delivering);
	}

	#[test]
	fn from_delivery_time_11() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(11);

		assert_eq!(status, PizzaStatus::Delivering);
	}

	#[test]
	fn from_delivery_time_12() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(12);

		assert_eq!(status, PizzaStatus::Delivering);
	}

	#[test]
	fn from_delivery_time_13() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(13);

		assert_eq!(status, PizzaStatus::Delivering);
	}

	#[test]
	fn from_delivery_time_14() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(14);

		assert_eq!(status, PizzaStatus::Delivering);
	}

	#[test]
	fn from_delivery_time_15() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(15);

		assert_eq!(status, PizzaStatus::Delivering);
	}

	#[test]
	fn from_delivery_time_16() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(16);

		assert_eq!(status, PizzaStatus::Delivering);
	}

	#[test]
	fn from_delivery_time_17() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(17);

		assert_eq!(status, PizzaStatus::Delivered);
	}

	#[test]
	fn from_delivery_time_18() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(18);

		assert_eq!(status, PizzaStatus::Delivered);
	}

	#[test]
	fn from_delivery_time_19() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(19);

		assert_eq!(status, PizzaStatus::Delivered);
	}

	#[test]
	fn from_delivery_time_20() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(20);

		assert_eq!(status, PizzaStatus::Delivered);
	}

	#[test]
	fn from_delivery_time_21() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(42);

		assert_eq!(status, PizzaStatus::Delivered);
	}

	#[test]
	fn from_delivery_time_22() {
		let status: PizzaStatus = PizzaStatus::from_delivery_time(4294967295);

		assert_eq!(status, PizzaStatus::Delivered);
	}

	#[test]
	fn get_delivery_time_in_days_ordered_00() {
		let status: PizzaStatus = PizzaStatus::Ordered;

		assert_eq!(status.get_delivery_time_in_days(), 17);
	}

	#[test]
	fn get_delivery_time_in_days_ordered_01() {
		let status: PizzaStatus = PizzaStatus::Cooking;

		assert_eq!(status.get_delivery_time_in_days(), 15);
	}

	#[test]
	fn get_delivery_time_in_days_ordered_02() {
		let status: PizzaStatus = PizzaStatus::Cooked;

		assert_eq!(status.get_delivery_time_in_days(), 10);
	}

	#[test]
	fn get_delivery_time_in_days_ordered_03() {
		let status: PizzaStatus = PizzaStatus::Delivering;

		assert_eq!(status.get_delivery_time_in_days(), 7);
	}

	#[test]
	fn get_delivery_time_in_days_ordered_04() {
		let status: PizzaStatus = PizzaStatus::Delivered;

		assert_eq!(status.get_delivery_time_in_days(), 0);
	}
}
