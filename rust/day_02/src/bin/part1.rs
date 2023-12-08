fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

/*
Requirements:
- Read TXT file
- Split Lines into String Array/Slice
- Iterate through String
- Check for numeric value in String
- Sum the numeric values together from each string
*/

fn part1(input: &str) -> i32 {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // input
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}