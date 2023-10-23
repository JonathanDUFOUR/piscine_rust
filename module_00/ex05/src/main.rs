fn is_leap_year(year: u32) -> bool {
	assert!(year > 0, "Invalid year");
	match (year % 4 == 0, year % 100 == 0, year % 400 == 0) {
		(true, true, true) => true,
		(true, true, false) => false,
		(true, false, _) => true,
		(false, _, _) => false,
	}
}

#[test]
#[should_panic(expected = "Invalid year")]
fn is_leap_year_0() {
	is_leap_year(0);
}

#[test]
fn is_leap_year_1() {
	assert!(!is_leap_year(1));
}

#[test]
fn is_leap_year_2() {
	assert!(!is_leap_year(2));
}

#[test]
fn is_leap_year_3() {
	assert!(!is_leap_year(3));
}

#[test]
fn is_leap_year_4() {
	assert!(is_leap_year(4));
}

#[test]
fn is_leap_year_5() {
	assert!(!is_leap_year(5));
}

#[test]
fn is_leap_year_6() {
	assert!(!is_leap_year(6));
}

#[test]
fn is_leap_year_7() {
	assert!(!is_leap_year(7));
}

#[test]
fn is_leap_year_8() {
	assert!(is_leap_year(8));
}

#[test]
fn is_leap_year_100() {
	assert!(!is_leap_year(100));
}

#[test]
fn is_leap_year_200() {
	assert!(!is_leap_year(200));
}

#[test]
fn is_leap_year_300() {
	assert!(!is_leap_year(300));
}

#[test]
fn is_leap_year_400() {
	assert!(is_leap_year(400));
}

#[test]
fn is_leap_year_500() {
	assert!(!is_leap_year(500));
}

#[test]
fn is_leap_year_600() {
	assert!(!is_leap_year(600));
}

#[test]
fn is_leap_year_700() {
	assert!(!is_leap_year(700));
}

#[test]
fn is_leap_year_800() {
	assert!(is_leap_year(800));
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
	assert!(year > 0, "Invalid year");
	assert!(month > 0 && month < 13, "Invalid month");
	if month > 7 {
		if month % 2 == 0 {
			return 31;
		}
		return 30;
	}
	if month == 2 {
		if is_leap_year(year) {
			return 29;
		}
		return 28;
	}
	if month % 2 == 0 {
		return 30;
	}
	return 31;
}

#[test]
fn num_days_in_month_00() {
	assert!(num_days_in_month(1, 1) == 31);
}

#[test]
fn num_days_in_month_01() {
	assert!(num_days_in_month(1, 2) == 28);
}

#[test]
fn num_days_in_month_02() {
	assert!(num_days_in_month(1, 3) == 31);
}

#[test]
fn num_days_in_month_03() {
	assert!(num_days_in_month(1, 4) == 30);
}

#[test]
fn num_days_in_month_04() {
	assert!(num_days_in_month(1, 5) == 31);
}

#[test]
fn num_days_in_month_05() {
	assert!(num_days_in_month(1, 6) == 30);
}

#[test]
fn num_days_in_month_06() {
	assert!(num_days_in_month(1, 7) == 31);
}

#[test]
fn num_days_in_month_07() {
	assert!(num_days_in_month(1, 8) == 31);
}

#[test]
fn num_days_in_month_08() {
	assert!(num_days_in_month(1, 9) == 30);
}

#[test]
fn num_days_in_month_09() {
	assert!(num_days_in_month(1, 10) == 31);
}

#[test]
fn num_days_in_month_10() {
	assert!(num_days_in_month(1, 11) == 30);
}

#[test]
fn num_days_in_month_11() {
	assert!(num_days_in_month(1, 12) == 31);
}

#[test]
fn num_days_in_month_12() {
	assert!(num_days_in_month(4, 1) == 31);
}

#[test]
fn num_days_in_month_13() {
	assert!(num_days_in_month(4, 2) == 29);
}

#[test]
fn num_days_in_month_14() {
	assert!(num_days_in_month(4, 3) == 31);
}

#[test]
fn num_days_in_month_15() {
	assert!(num_days_in_month(4, 4) == 30);
}

#[test]
fn num_days_in_month_16() {
	assert!(num_days_in_month(4, 5) == 31);
}

#[test]
fn num_days_in_month_17() {
	assert!(num_days_in_month(4, 6) == 30);
}

#[test]
fn num_days_in_month_18() {
	assert!(num_days_in_month(4, 7) == 31);
}

#[test]
fn num_days_in_month_19() {
	assert!(num_days_in_month(4, 8) == 31);
}

#[test]
fn num_days_in_month_20() {
	assert!(num_days_in_month(4, 9) == 30);
}

#[test]
fn num_days_in_month_21() {
	assert!(num_days_in_month(4, 10) == 31);
}

#[test]
fn num_days_in_month_22() {
	assert!(num_days_in_month(4, 11) == 30);
}

#[test]
fn num_days_in_month_23() {
	assert!(num_days_in_month(4, 12) == 31);
}

#[test]
fn num_days_in_month_24() {
	assert!(num_days_in_month(100, 1) == 31);
}

#[test]
fn num_days_in_month_25() {
	assert!(num_days_in_month(100, 2) == 28);
}

#[test]
fn num_days_in_month_26() {
	assert!(num_days_in_month(100, 3) == 31);
}

#[test]
fn num_days_in_month_27() {
	assert!(num_days_in_month(100, 4) == 30);
}

#[test]
fn num_days_in_month_28() {
	assert!(num_days_in_month(100, 5) == 31);
}

#[test]
fn num_days_in_month_29() {
	assert!(num_days_in_month(100, 6) == 30);
}

#[test]
fn num_days_in_month_30() {
	assert!(num_days_in_month(100, 7) == 31);
}

#[test]
fn num_days_in_month_31() {
	assert!(num_days_in_month(100, 8) == 31);
}

#[test]
fn num_days_in_month_32() {
	assert!(num_days_in_month(100, 9) == 30);
}

#[test]
fn num_days_in_month_33() {
	assert!(num_days_in_month(100, 10) == 31);
}

#[test]
fn num_days_in_month_34() {
	assert!(num_days_in_month(100, 11) == 30);
}

#[test]
fn num_days_in_month_35() {
	assert!(num_days_in_month(100, 12) == 31);
}

#[test]
fn num_days_in_month_36() {
	assert!(num_days_in_month(400, 1) == 31);
}

#[test]
fn num_days_in_month_37() {
	assert!(num_days_in_month(400, 2) == 29);
}

#[test]
fn num_days_in_month_38() {
	assert!(num_days_in_month(400, 3) == 31);
}

#[test]
fn num_days_in_month_39() {
	assert!(num_days_in_month(400, 4) == 30);
}

#[test]
fn num_days_in_month_40() {
	assert!(num_days_in_month(400, 5) == 31);
}

#[test]
fn num_days_in_month_41() {
	assert!(num_days_in_month(400, 6) == 30);
}

#[test]
fn num_days_in_month_42() {
	assert!(num_days_in_month(400, 7) == 31);
}

#[test]
fn num_days_in_month_43() {
	assert!(num_days_in_month(400, 8) == 31);
}

#[test]
fn num_days_in_month_44() {
	assert!(num_days_in_month(400, 9) == 30);
}

#[test]
fn num_days_in_month_45() {
	assert!(num_days_in_month(400, 10) == 31);
}

#[test]
fn num_days_in_month_46() {
	assert!(num_days_in_month(400, 11) == 30);
}

#[test]
fn num_days_in_month_47() {
	assert!(num_days_in_month(400, 12) == 31);
}

#[test]
#[should_panic(expected = "Invalid year")]
fn num_days_in_month_48() {
	num_days_in_month(0, 1);
}

#[test]
#[should_panic(expected = "Invalid month")]
fn num_days_in_month_49() {
	num_days_in_month(1, 0);
}

#[test]
#[should_panic(expected = "Invalid month")]
fn num_days_in_month_50() {
	num_days_in_month(1, 13);
}

fn month_name(month: u32) -> &'static str {
	match month {
		1 => "Januaray",
		2 => "February",
		3 => "March",
		4 => "April",
		5 => "May",
		6 => "June",
		7 => "July",
		8 => "August",
		9 => "September",
		10 => "October",
		11 => "November",
		12 => "December",
		_ => panic!("Invalid month"),
	}
}

#[test]
fn month_name_00() {
	assert_eq!(month_name(1), "Januaray");
}

#[test]
fn month_name_01() {
	assert_eq!(month_name(2), "February");
}

#[test]
fn month_name_02() {
	assert_eq!(month_name(3), "March");
}

#[test]
fn month_name_03() {
	assert_eq!(month_name(4), "April");
}

#[test]
fn month_name_04() {
	assert_eq!(month_name(5), "May");
}

#[test]
fn month_name_05() {
	assert_eq!(month_name(6), "June");
}

#[test]
fn month_name_06() {
	assert_eq!(month_name(7), "July");
}

#[test]
fn month_name_07() {
	assert_eq!(month_name(8), "August");
}

#[test]
fn month_name_08() {
	assert_eq!(month_name(9), "September");
}

#[test]
fn month_name_09() {
	assert_eq!(month_name(10), "October");
}

#[test]
fn month_name_10() {
	assert_eq!(month_name(11), "November");
}

#[test]
fn month_name_11() {
	assert_eq!(month_name(12), "December");
}

#[test]
#[should_panic(expected = "Invalid month")]
fn month_name_12() {
	month_name(0);
}

#[test]
#[should_panic(expected = "Invalid month")]
fn month_name_13() {
	month_name(13);
}

#[test]
#[should_panic(expected = "Invalid month")]
fn month_name_14() {
	month_name(u32::MAX);
}

fn main() {
	let mut total: u32 = 0;

	for year in 1..=2023 {
		for month in 1..=12 {
			if (total + 13) % 7 == 5 {
				println!("Friday, {} 13, {}", month_name(month), year);
			}
			total += num_days_in_month(year, month);
		}
	}
}
