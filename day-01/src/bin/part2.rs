use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("Sum of calibration values should be: {}", result);
}

fn part2(input: &str) -> u16 {
    let string_digits = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    input
        .lines()
        .map(|row: &str| -> u16 {
            let row_digits = get_digits_from_row(&row, &string_digits);

            match !row_digits.is_empty() {
                true => {
                    let first = row_digits.first().expect("First should be a number");
                    let last = row_digits.last().expect("Last should be a number");

                    let combined = format!("{}{}", first, last);
                    return combined.parse::<u16>().expect("Failed to parse number");
                }
                false => 0,
            }
        })
        .sum()
}

///  Takes a row and and a map of digits represted as word an number as char. Searches written
///  numbers in the row and replaces them with the number representation. Other characters are
///  returned as is. Then only ascii digits are filtered from the result and returned as a vector.
/// * `row`:  The row being processed.
/// * `string_digits`: Vector containing the digits as chars.
fn get_digits_from_row(row: &str, string_digits: &HashMap<&str, char>) -> Vec<char> {
    let mut index = 0;
    std::iter::from_fn(move || {
        if !row.is_char_boundary(index) {
            return None;
        }

        let line = &row[index..];

        let mut value = string_digits.iter().find_map(|(key, value)| {
            if line.starts_with(key) {
                return Some(value.to_owned());
            }
            return None;
        });

        if value.is_none() {
            value = line.chars().next();
        }

        index += 1;
        value
    })
    .filter(|c| c.is_ascii_digit())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen§";
        let result = part2(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn test_part2_case2() {
        let input = "fiveone8threethreezsfpzsrbb9fourfive\n56snzkgsone2cxtpvvh\nc5\nfivezg8twoneg";
        let result = part2(input);
        assert_eq!(result, 213);
    }

    #[test]
    fn test_get_row_digits() {
        let string_digits = HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);
        let input = "two1nine";
        let result = get_digits_from_row(&input, &string_digits);

        assert_eq!(result, vec!['2', '1', '9']);
    }
}
