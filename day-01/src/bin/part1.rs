fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("Sum of calibration values should be: {}", result);
}

fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(|row: &str| -> u16 {
            let num_vec: Vec<char> = row.chars().filter(|c| c.is_ascii_digit()).collect();

            match !num_vec.is_empty() {
                true => {
                    let first = num_vec.first().expect("First should be a number");
                    let last = num_vec.last().expect("Last should be a number");

                    let combined = format!("{}{}", first, last);
                    combined.parse::<u16>().expect("Failed to parse number")
                }
                false => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part1_has_zero() {
        let input = "1a0bc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = part1(input);
        assert_eq!(result, 0);
    }
}
