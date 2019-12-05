//It is a six-digit number.
//The value is within the range given in your puzzle input.
//Two adjacent digits are the same (like 22 in 122345).
//Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

type Unit = i32;

fn main() {
    let from: Unit = 264360;
    let to: Unit = 746325;

    let count = (from..to).filter(|n|is_match(*n)).count();
    println!("{} possible matches", count)
}

fn is_match(n: Unit) -> bool {
    is_six_digit_number(n) && has_adjacent_digits(n) && has_only_increasing_digits(n)
}

fn is_six_digit_number(n: Unit) -> bool {
    n >= 100000 && n < 1000000
}

fn has_adjacent_digits(n: Unit) -> bool {
    let (mut last_digit, mut number) = split_last_digit(n);

    // Count the number of numbers
    let steps = 1.0 +  (n as f64).log10();

    for _ in 0..steps as usize {
        let (new_last_digit, new_number) = split_last_digit(number);
        if last_digit == new_last_digit {
            return true;
        }

        last_digit = new_last_digit;
        number = new_number;
    }

    return false
}

fn split_last_digit(n: Unit) -> (Unit, Unit) {
    (n % 10, n / 10)
}

fn has_only_increasing_digits(n: Unit) -> bool {
    let (mut last_digit, mut number) = split_last_digit(n);

    // Count the number of numbers
    let steps = 1.0 +  (n as f64).log10();
    for _ in 0..steps as usize {
        let (new_last_digit, new_number) = split_last_digit(number);
        if last_digit < new_last_digit {
            return false;
        }

        last_digit = new_last_digit;
        number = new_number;
    }

    return true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_adjacent_digits() {
        assert!(!has_adjacent_digits(12345));
        assert!(!has_adjacent_digits(12345));
        assert!(!has_adjacent_digits(123450));
        assert!(!has_adjacent_digits(121));
        assert!(!has_adjacent_digits(1));
        assert!(has_adjacent_digits(11));
        assert!(has_adjacent_digits(0));
        assert!(has_adjacent_digits(11111));
        assert!(has_adjacent_digits(2112));
        assert!(has_adjacent_digits(112));
        assert!(has_adjacent_digits(211));
    }

    #[test]
    fn test_has_only_increasing_digits() {
        assert!(has_only_increasing_digits(1234567));
        assert!(!has_only_increasing_digits(76543210));
        assert!(has_only_increasing_digits(0112));
        assert!(has_only_increasing_digits(11233));
        assert!(has_only_increasing_digits(1233));
        assert!(has_only_increasing_digits(111));
        assert!(has_only_increasing_digits(1));
    }
}
