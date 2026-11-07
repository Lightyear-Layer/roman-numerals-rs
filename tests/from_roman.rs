use roman_numerals::FromRoman;

#[test]
fn invalids() {
    assert_eq!(u64::from_roman("a", true), None);
    assert_eq!(u64::from_roman("B", true), None);
    assert_eq!(u64::from_roman("abc", true), None);
    assert_eq!(u64::from_roman("ABC", true), None);
}

#[test]
fn uppercase_lowercase() {
    assert_eq!(
        u64::from_roman("NULLA", true),
        u64::from_roman("nulla", true)
    );
    assert_eq!(u64::from_roman("I", true), u64::from_roman("i", true));
    assert_eq!(u64::from_roman("V", true), u64::from_roman("v", true));
    assert_eq!(u64::from_roman("X", true), u64::from_roman("x", true));
    assert_eq!(u64::from_roman("L", true), u64::from_roman("l", true));
    assert_eq!(u64::from_roman("C", true), u64::from_roman("c", true));
    assert_eq!(u64::from_roman("D", true), u64::from_roman("d", true));
    assert_eq!(u64::from_roman("M", true), u64::from_roman("m", true));
}

#[test]
fn basics() {
    assert_eq!(u64::from_roman("nulla", true), Some(0));
    assert_eq!(u64::from_roman("I", true), Some(1));
    assert_eq!(u64::from_roman("V", true), Some(5));
    assert_eq!(u64::from_roman("X", true), Some(10));
    assert_eq!(u64::from_roman("L", true), Some(50));
    assert_eq!(u64::from_roman("C", true), Some(100));
    assert_eq!(u64::from_roman("D", true), Some(500));
    assert_eq!(u64::from_roman("M", true), Some(1_000));
}

#[test]
fn zero_to_one_hundred() {
    assert_eq!(u64::from_roman("nulla", true), Some(0));
    assert_eq!(u64::from_roman("I", true), Some(1));
    assert_eq!(u64::from_roman("II", true), Some(2));
    assert_eq!(u64::from_roman("III", true), Some(3));
    assert_eq!(u64::from_roman("IV", true), Some(4));
    assert_eq!(u64::from_roman("V", true), Some(5));
    assert_eq!(u64::from_roman("VI", true), Some(6));
    assert_eq!(u64::from_roman("VII", true), Some(7));
    assert_eq!(u64::from_roman("VIII", true), Some(8));
    assert_eq!(u64::from_roman("IX", true), Some(9));
    assert_eq!(u64::from_roman("X", true), Some(10));
    assert_eq!(u64::from_roman("XI", true), Some(11));
    assert_eq!(u64::from_roman("XII", true), Some(12));
    assert_eq!(u64::from_roman("XIII", true), Some(13));
    assert_eq!(u64::from_roman("XIV", true), Some(14));
    assert_eq!(u64::from_roman("XV", true), Some(15));
    assert_eq!(u64::from_roman("XVI", true), Some(16));
    assert_eq!(u64::from_roman("XVII", true), Some(17));
    assert_eq!(u64::from_roman("XVIII", true), Some(18));
    assert_eq!(u64::from_roman("XIX", true), Some(19));
    assert_eq!(u64::from_roman("XX", true), Some(20));
    assert_eq!(u64::from_roman("XXI", true), Some(21));
    assert_eq!(u64::from_roman("XXII", true), Some(22));
    assert_eq!(u64::from_roman("XXIII", true), Some(23));
    assert_eq!(u64::from_roman("XXIV", true), Some(24));
    assert_eq!(u64::from_roman("XXV", true), Some(25));
    assert_eq!(u64::from_roman("XXVI", true), Some(26));
    assert_eq!(u64::from_roman("XXVII", true), Some(27));
    assert_eq!(u64::from_roman("XXVIII", true), Some(28));
    assert_eq!(u64::from_roman("XXIX", true), Some(29));
    assert_eq!(u64::from_roman("XXX", true), Some(30));
    assert_eq!(u64::from_roman("XXXI", true), Some(31));
    assert_eq!(u64::from_roman("XXXII", true), Some(32));
    assert_eq!(u64::from_roman("XXXIII", true), Some(33));
    assert_eq!(u64::from_roman("XXXIV", true), Some(34));
    assert_eq!(u64::from_roman("XXXV", true), Some(35));
    assert_eq!(u64::from_roman("XXXVI", true), Some(36));
    assert_eq!(u64::from_roman("XXXVII", true), Some(37));
    assert_eq!(u64::from_roman("XXXVIII", true), Some(38));
    assert_eq!(u64::from_roman("XXXIX", true), Some(39));
    assert_eq!(u64::from_roman("XL", true), Some(40));
    assert_eq!(u64::from_roman("XLI", true), Some(41));
    assert_eq!(u64::from_roman("XLII", true), Some(42));
    assert_eq!(u64::from_roman("XLIII", true), Some(43));
    assert_eq!(u64::from_roman("XLIV", true), Some(44));
    assert_eq!(u64::from_roman("XLV", true), Some(45));
    assert_eq!(u64::from_roman("XLVI", true), Some(46));
    assert_eq!(u64::from_roman("XLVII", true), Some(47));
    assert_eq!(u64::from_roman("XLVIII", true), Some(48));
    assert_eq!(u64::from_roman("XLIX", true), Some(49));
    assert_eq!(u64::from_roman("L", true), Some(50));
    assert_eq!(u64::from_roman("LI", true), Some(51));
    assert_eq!(u64::from_roman("LII", true), Some(52));
    assert_eq!(u64::from_roman("LIII", true), Some(53));
    assert_eq!(u64::from_roman("LIV", true), Some(54));
    assert_eq!(u64::from_roman("LV", true), Some(55));
    assert_eq!(u64::from_roman("LVI", true), Some(56));
    assert_eq!(u64::from_roman("LVII", true), Some(57));
    assert_eq!(u64::from_roman("LVIII", true), Some(58));
    assert_eq!(u64::from_roman("LIX", true), Some(59));
    assert_eq!(u64::from_roman("LX", true), Some(60));
    assert_eq!(u64::from_roman("LXI", true), Some(61));
    assert_eq!(u64::from_roman("LXII", true), Some(62));
    assert_eq!(u64::from_roman("LXIII", true), Some(63));
    assert_eq!(u64::from_roman("LXIV", true), Some(64));
    assert_eq!(u64::from_roman("LXV", true), Some(65));
    assert_eq!(u64::from_roman("LXVI", true), Some(66));
    assert_eq!(u64::from_roman("LXVII", true), Some(67));
    assert_eq!(u64::from_roman("LXVIII", true), Some(68));
    assert_eq!(u64::from_roman("LXIX", true), Some(69));
    assert_eq!(u64::from_roman("LXX", true), Some(70));
    assert_eq!(u64::from_roman("LXXI", true), Some(71));
    assert_eq!(u64::from_roman("LXXII", true), Some(72));
    assert_eq!(u64::from_roman("LXXIII", true), Some(73));
    assert_eq!(u64::from_roman("LXXIV", true), Some(74));
    assert_eq!(u64::from_roman("LXXV", true), Some(75));
    assert_eq!(u64::from_roman("LXXVI", true), Some(76));
    assert_eq!(u64::from_roman("LXXVII", true), Some(77));
    assert_eq!(u64::from_roman("LXXVIII", true), Some(78));
    assert_eq!(u64::from_roman("LXXIX", true), Some(79));
    assert_eq!(u64::from_roman("LXXX", true), Some(80));
    assert_eq!(u64::from_roman("LXXXI", true), Some(81));
    assert_eq!(u64::from_roman("LXXXII", true), Some(82));
    assert_eq!(u64::from_roman("LXXXIII", true), Some(83));
    assert_eq!(u64::from_roman("LXXXIV", true), Some(84));
    assert_eq!(u64::from_roman("LXXXV", true), Some(85));
    assert_eq!(u64::from_roman("LXXXVI", true), Some(86));
    assert_eq!(u64::from_roman("LXXXVII", true), Some(87));
    assert_eq!(u64::from_roman("LXXXVIII", true), Some(88));
    assert_eq!(u64::from_roman("LXXXIX", true), Some(89));
    assert_eq!(u64::from_roman("XC", true), Some(90));
    assert_eq!(u64::from_roman("XCI", true), Some(91));
    assert_eq!(u64::from_roman("XCII", true), Some(92));
    assert_eq!(u64::from_roman("XCIII", true), Some(93));
    assert_eq!(u64::from_roman("XCIV", true), Some(94));
    assert_eq!(u64::from_roman("XCV", true), Some(95));
    assert_eq!(u64::from_roman("XCVI", true), Some(96));
    assert_eq!(u64::from_roman("XCVII", true), Some(97));
    assert_eq!(u64::from_roman("XCVIII", true), Some(98));
    assert_eq!(u64::from_roman("XCIX", true), Some(99));
    assert_eq!(u64::from_roman("C", true), Some(100));
}
