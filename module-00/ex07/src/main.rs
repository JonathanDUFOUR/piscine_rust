use ex07::strpcmp;

#[test]
fn strpcmp_00() {
	assert!(strpcmp(b"", b""));
}

#[test]
fn strpcmp_01() {
	assert!(!strpcmp(b"", b"a"));
}

#[test]
fn strpcmp_02() {
	assert!(!strpcmp(b"a", b""));
}

#[test]
fn strpcmp_03() {
	assert!(strpcmp(b"a", b"a"));
}

#[test]
fn strpcmp_04() {
	assert!(!strpcmp(b"a", b"ba"));
}

#[test]
fn strpcmp_05() {
	assert!(!strpcmp(b"a", b"ab"));
}

#[test]
fn strpcmp_06() {
	assert!(!strpcmp(b"ba", b"a"));
}

#[test]
fn strpcmp_07() {
	assert!(!strpcmp(b"ab", b"a"));
}

#[test]
fn strpcmp_08() {
	assert!(!strpcmp(b"*", b""));
}

#[test]
fn strpcmp_09() {
	assert!(strpcmp(b"", b"*"));
}

#[test]
fn strpcmp_10() {
	assert!(strpcmp(b"*", b"*"));
}

#[test]
fn strpcmp_11() {
	assert!(strpcmp(b"a", b"*"));
}

#[test]
fn strpcmp_12() {
	assert!(!strpcmp(b"*", b"a"));
}

#[test]
fn strpcmp_13() {
	assert!(strpcmp(b"a", b"*a"));
}

#[test]
fn strpcmp_14() {
	assert!(strpcmp(b"a", b"a*"));
}

#[test]
fn strpcmp_15() {
	assert!(!strpcmp(b"*a", b"a"));
}

#[test]
fn strpcmp_16() {
	assert!(!strpcmp(b"a*", b"a"));
}

#[test]
fn strpcmp_17() {
	assert!(strpcmp(b"ab", b"*"));
}

#[test]
fn strpcmp_18() {
	assert!(!strpcmp(b"ab", b"*a"));
}

#[test]
fn strpcmp_19() {
	assert!(strpcmp(b"ab", b"a*"));
}

#[test]
fn strpcmp_20() {
	assert!(strpcmp(b"ab", b"*b"));
}

#[test]
fn strpcmp_21() {
	assert!(!strpcmp(b"ab", b"b*"));
}

#[test]
fn strpcmp_22() {
	assert!(!strpcmp(b"abc", b"*a"));
}

#[test]
fn strpcmp_23() {
	assert!(strpcmp(b"abc", b"a*"));
}

#[test]
fn strpcmp_24() {
	assert!(strpcmp(b"abc", b"*a*"));
}

#[test]
fn strpcmp_25() {
	assert!(!strpcmp(b"abc", b"*b"));
}

#[test]
fn strpcmp_26() {
	assert!(!strpcmp(b"abc", b"b*"));
}

#[test]
fn strpcmp_27() {
	assert!(strpcmp(b"abc", b"*b*"));
}

#[test]
fn strpcmp_28() {
	assert!(strpcmp(b"abc", b"*c"));
}

#[test]
fn strpcmp_29() {
	assert!(!strpcmp(b"abc", b"c*"));
}

#[test]
fn strpcmp_30() {
	assert!(strpcmp(b"abc", b"*c*"));
}

#[test]
fn strpcmp_31() {
	assert!(!strpcmp(b"abc", b"*ab"));
}

#[test]
fn strpcmp_32() {
	assert!(!strpcmp(b"abc", b"a*b"));
}

#[test]
fn strpcmp_33() {
	assert!(strpcmp(b"abc", b"ab*"));
}

#[test]
fn strpcmp_34() {
	assert!(!strpcmp(b"abc", b"*a*b"));
}

#[test]
fn strpcmp_35() {
	assert!(strpcmp(b"abc", b"*ab*"));
}

#[test]
fn strpcmp_36() {
	assert!(strpcmp(b"abc", b"a*b*"));
}

#[test]
fn strpcmp_37() {
	assert!(strpcmp(b"abc", b"*a*b*"));
}

#[test]
fn strpcmp_38() {
	assert!(strpcmp(b"abc", b"*a*c"));
}

#[test]
fn strpcmp_39() {
	assert!(!strpcmp(b"abc", b"*ac*"));
}

#[test]
fn strpcmp_40() {
	assert!(strpcmp(b"abc", b"a*c*"));
}

#[test]
fn strpcmp_41() {
	assert!(strpcmp(b"abc", b"*a*c*"));
}

#[test]
fn strpcmp_42() {
	assert!(!strpcmp(b"abcdefh", b"*adh"));
}

#[test]
fn strpcmp_43() {
	assert!(!strpcmp(b"abcdefh", b"a*dh"));
}

#[test]
fn strpcmp_44() {
	assert!(!strpcmp(b"abcdefh", b"ad*h"));
}

#[test]
fn strpcmp_45() {
	assert!(!strpcmp(b"abcdefh", b"adh*"));
}

#[test]
fn strpcmp_46() {
	assert!(!strpcmp(b"abcdefh", b"*a*dh"));
}

#[test]
fn strpcmp_47() {
	assert!(!strpcmp(b"abcdefh", b"*ad*h"));
}

#[test]
fn strpcmp_48() {
	assert!(!strpcmp(b"abcdefh", b"*adh*"));
}

#[test]
fn strpcmp_49() {
	assert!(strpcmp(b"abcdefh", b"a*d*h"));
}

#[test]
fn strpcmp_50() {
	assert!(!strpcmp(b"abcdefh", b"a*dh*"));
}

#[test]
fn strpcmp_51() {
	assert!(!strpcmp(b"abcdefh", b"ad*h*"));
}

#[test]
fn strpcmp_52() {
	assert!(strpcmp(b"abcdefh", b"*a*d*h"));
}

#[test]
fn strpcmp_53() {
	assert!(!strpcmp(b"abcdefh", b"*a*dh*"));
}

#[test]
fn strpcmp_54() {
	assert!(!strpcmp(b"abcdefh", b"*ad*h*"));
}

#[test]
fn strpcmp_55() {
	assert!(strpcmp(b"abcdefh", b"a*d*h*"));
}

#[test]
fn strpcmp_56() {
	assert!(strpcmp(b"abcdefh", b"*a*d*h*"));
}

#[test]
fn strpcmp_57() {
	assert!(strpcmp(b"Gotta catch them all", b"Go**atch** them *"));
}

#[test]
fn strpcmp_58() {
	assert!(!strpcmp(b"abcabcdabc", b"abc*abc*abcd"));
}

#[test]
fn strpcmp_59() {
	assert!(!strpcmp(b"abcabcdabc", b"*abcd*abcd*"));
}
