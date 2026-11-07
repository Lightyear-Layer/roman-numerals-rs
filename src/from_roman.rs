/// Converts a roman numeral string to a number.
///
/// # Examples
///
/// ```
/// use roman_numerals::FromRoman;
/// assert_eq!(u64::from_roman("ABC", true), None);
/// assert_eq!(u64::from_roman("nulla", true), Some(0));
/// assert_eq!(u64::from_roman("I", true), Some(1));
/// ```
pub trait FromRoman {
    fn from_roman(number: &str, strict: bool) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl FromRoman for u64 {
    fn from_roman(number: &str, strict: bool) -> Option<u64> {
        let number = number.to_uppercase();
        if number == "NULLA" {
            return Some(0);
        }
        if !number
            .chars()
            .all(|c| matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'))
        {
            return None;
        }
        let mut accumulator: u64 = 0;
        let mut digits = number
            .chars()
            .map(|c| match c {
                'I' => RomanDigit::I,
                'V' => RomanDigit::V,
                'X' => RomanDigit::X,
                'L' => RomanDigit::L,
                'C' => RomanDigit::C,
                'D' => RomanDigit::D,
                'M' => RomanDigit::M,
                _ => unreachable!(),
            })
            .rev()
            .peekable();
        let mut run_len = 0;
        while let Some(digit) = digits.next() {
            accumulator += match digit {
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1_000,
            };
            if let Some(&next_digit) = digits.peek() {
                if next_digit < digit {
                    accumulator -= match next_digit {
                        RomanDigit::I => 1,
                        RomanDigit::V => 5,
                        RomanDigit::X => 10,
                        RomanDigit::L => 50,
                        RomanDigit::C => 100,
                        RomanDigit::D => 500,
                        RomanDigit::M => 1_000,
                    };
                    digits.next();
                    run_len = 0;

                    if strict && digits.peek().is_some_and(|next_digit| *next_digit < digit) {
                        return None;
                    }
                } else if next_digit == digit {
                    if strict && matches!(digit, RomanDigit::V | RomanDigit::L | RomanDigit::D) {
                        return None;
                    }
                    run_len += 1;
                    if run_len >= 3 && strict {
                        return None;
                    }
                } else {
                    run_len = 0;
                }
            }
        }
        Some(accumulator)
    }
}
