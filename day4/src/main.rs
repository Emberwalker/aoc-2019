use std::env;
use std::io;
use std::io::Write;

fn main() {
    // Call with --part-two to calculate part 2 answer
    let part_two = env::args()
        .nth(1)
        .map_or(false, |arg| "--part-two".eq(&arg));

    let mut raw = String::new();
    print!("Enter range string: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut raw)
        .expect("failed to get line from terminal");

    let raw_parts: Vec<u32> = raw.split('-').map(|s| s.trim().parse().unwrap()).collect();
    let possible_passwords_iter = possible_passwords(raw_parts[0], raw_parts[1]);

    let possible_passwords_count = if part_two {
        part_two_filter(possible_passwords_iter).count()
    } else {
        possible_passwords_iter.count()
    };

    println!("Possible passwords: {}", possible_passwords_count);
}

fn to_digits(i: u32) -> Vec<u32> {
    // Pad to 6 digits
    format!("{:06}", i)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn possible_passwords(start: u32, end_inclusive: u32) -> impl Iterator<Item = u32> {
    (start..=end_inclusive)
        .map(|i| (i, to_digits(i)))
        .filter(|(_, digits)| digits.windows(2).all(|window| window[0] <= window[1]))
        .filter(|(_, digits)| digits.windows(2).any(|window| window[0] == window[1]))
        .map(|(password, _)| password)
}

fn part_two_filter(iter: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    iter.map(|i| {
        let mut windows = to_digits(i)
            .windows(2)
            .map(|window| window[0] == window[1])
            .collect::<Vec<bool>>();
        windows.push(false); // to make sure the scan completes a streak
        (i, windows)
    })
    .filter(|(_, windows)| {
        let min_streak = windows
            .iter()
            .scan(0u8, |streak, v| {
                if *v {
                    *streak += 1;
                    Some(u8::max_value())
                } else if *streak == 0 {
                    // Ignore falsey streaks
                    Some(u8::max_value())
                } else {
                    let res = *streak;
                    *streak = 0;
                    Some(res)
                }
            })
            .min()
            .unwrap_or(u8::max_value());
        min_streak == 1
    })
    .map(|(i, _)| i)
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn test_possible_password() {
        assert_eq!(
            possible_passwords(111_111, 111_111).collect::<Vec<u32>>(),
            vec!(111_111u32)
        );
    }

    #[test]
    fn test_not_possible_password_decreasing() {
        assert_eq!(
            possible_passwords(223_450, 223_450).collect::<Vec<u32>>(),
            vec!()
        );
    }

    #[test]
    fn test_not_possible_password_no_double() {
        assert_eq!(
            possible_passwords(123_789, 123_789).collect::<Vec<u32>>(),
            vec!()
        );
    }

    #[test]
    fn part_two_all_doubles() {
        assert_eq!(
            part_two_filter(vec!(112_233u32).into_iter()).collect::<Vec<u32>>(),
            vec!(112_233u32)
        );
    }

    #[test]
    fn part_two_invalid_streak() {
        assert_eq!(
            part_two_filter(vec!(123_444u32).into_iter()).collect::<Vec<u32>>(),
            vec!()
        );
    }

    #[test]
    fn part_two_invalid_and_valid_streak() {
        assert_eq!(
            part_two_filter(vec!(111_122u32).into_iter()).collect::<Vec<u32>>(),
            vec!(111_122u32)
        );
    }

    #[test]
    fn part_two_blah() {
        let v = 111_223u32;
        assert_eq!(
            part_two_filter(vec!(v).into_iter()).collect::<Vec<u32>>(),
            vec!(v)
        );
    }
}
