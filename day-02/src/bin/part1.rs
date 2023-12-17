fn main() {
    let input = include_str!("./input1.txt");
    let bag_of_cubes = GameCubes::new(12, 13, 14);
    let result = part1(input, &bag_of_cubes);

    println!("Sum of Id's is: {}", result);
}

fn part1(input: &str, bag_of_cubes: &GameCubes) -> u32 {
    input
        .lines()
        .map(|line: &str| -> u32 {
            let mut game = GameCubes::new(0, 0, 0);

            game.add_red(1);

            if bag_of_cubes.is_valid_game(&game) {
                return 0;
            }
            0
        })
        .sum()
}

struct GameCubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameCubes {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn add_green(&mut self, green: u32) {
        self.green += green;
    }

    fn add_red(&mut self, red: u32) {
        self.red += red;
    }

    fn add_blue(&mut self, blue: u32) {
        self.blue += blue;
    }

    fn is_valid_game(&self, game: &GameCubes) -> bool {
        self.red >= game.red && self.green >= game.green && self.blue >= game.blue
    }
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

    #[test]
    fn test_add_methods() {
        let mut game = GameCubes::new(0, 0, 0);

        game.add_red(1);
        game.add_green(2);
        game.add_blue(3);

        assert_eq!(game.red, 1);
        assert_eq!(game.green, 2);
        assert_eq!(game.blue, 3);
    }

    #[test]
    fn test_is_valid_game() {
        let valid_game = GameCubes::new(10, 12, 11);
        let not_valid_game = GameCubes::new(15, 12, 16);
        let test_bag_of_cubes = GameCubes::new(12, 12, 14);

        assert_eq!(test_bag_of_cubes.is_valid_game(&valid_game), true);
        assert_eq!(test_bag_of_cubes.is_valid_game(&not_valid_game), false);
    }
}
