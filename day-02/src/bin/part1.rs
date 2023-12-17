struct GameCubes {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("./input1.txt");
    let part1_game = GameCubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    part1(input, &part1_game);
}

fn part1(input: &str, game: &GameCubes) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    println!("{:?}", lines);
    0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let game = GameCubes {
            red: 12,
            green: 13,
            blue: 14,
        };

        let result = part1(input, &game);
        assert_eq!(result, 8);
    }
}
