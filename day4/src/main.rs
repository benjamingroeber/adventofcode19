//It is a six-digit number.
//The value is within the range given in your puzzle input.
//Two adjacent digits are the same (like 22 in 122345).
//Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

// Part 2 requires the number must contain one set of exactly two adjacent identical digits
type Unit = i32;

fn main() {
    let from: Unit = 264360;
    let to: Unit = 746325;

    let count = (from..to).filter(|n| is_match_first_challenge(*n)).count();
    println!("Part 1: {} possible matches", count);
    let count = (from..to).filter(|n| is_match_second_challenge(*n)).count();
    println!("Part 2: {} possible matches", count);
}

fn is_match_first_challenge(n: Unit) -> bool {
    is_six_digit_number(n) && has_only_increasing_digits(n) && has_adjacent_digits(n)
}

fn is_match_second_challenge(n: Unit) -> bool {
    is_six_digit_number(n) && has_only_increasing_digits(n) && has_set_of_two_adjacent_digits(n)
}

fn is_six_digit_number(n: Unit) -> bool {
    n >= 100000 && n < 1000000
}

/// This does only check for the presence of any adjacent identical digits
fn has_adjacent_digits(n: Unit) -> bool {
    let (mut last_digit, mut number) = split_last_digit(n);

    // Count the number of numbers
    let steps = 1.0 + (n as f64).log10();

    for _ in 0..steps as usize {
        let (new_last_digit, new_number) = split_last_digit(number);
        if last_digit == new_last_digit {
            return true;
        }

        last_digit = new_last_digit;
        number = new_number;
    }

    return false;
}

/// This does account for one set of two adjacent identical digits
fn has_set_of_two_adjacent_digits(n: Unit) -> bool {
    let (mut last_digit, mut number) = split_last_digit(n);

    // Count the number of numbers
    let steps = 1.0 + (n as f64).log10();

    let mut repetitions = 0;
    for _ in 0..steps as usize {
        let (new_last_digit, new_number) = split_last_digit(number);
        let is_new_number = last_digit != new_last_digit;

        if is_new_number {
            // found a set of two identical digits in the middle
            if repetitions == 1 {
                return true;
            }
            repetitions = 0
        } else {
            repetitions += 1
        }

        last_digit = new_last_digit;
        number = new_number;
    }

    // true if last set of numbers is matching criteria
    return repetitions == 1;
}

fn split_last_digit(n: Unit) -> (Unit, Unit) {
    (n % 10, n / 10)
}

fn has_only_increasing_digits(n: Unit) -> bool {
    let (mut last_digit, mut number) = split_last_digit(n);

    // Count the number of numbers
    let steps = 1.0 + (n as f64).log10();
    for _ in 0..steps as usize {
        let (new_last_digit, new_number) = split_last_digit(number);
        if last_digit < new_last_digit {
            return false;
        }

        last_digit = new_last_digit;
        number = new_number;
    }

    return true;
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

    #[test]
    fn test_has_set_of_two_adjacent_digits() {
        assert!(has_set_of_two_adjacent_digits(112233));
        assert!(!has_set_of_two_adjacent_digits(123444));
        assert!(has_set_of_two_adjacent_digits(111122));
        assert!(has_set_of_two_adjacent_digits(110022));
        assert!(has_set_of_two_adjacent_digits(1110022));
        assert!(!has_set_of_two_adjacent_digits(111));
        assert!(has_set_of_two_adjacent_digits(11));
        assert!(!has_set_of_two_adjacent_digits(1));
    }
}
