use aho_corasick::AhoCorasick;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
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

fn part2(input: &str) -> i32 {
    let numbers = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5",
        "six", "6", "seven", "7", "eight", "8", "nine", "9"
    ];
    let mut total = 0;
    let ac = AhoCorasick::new(numbers).unwrap();

    for line in input.lines() {
        let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
        let first = matches.iter().nth(0).unwrap().pattern().as_usize() / 2 + 1;
        let last = matches.iter().last().unwrap().pattern().as_usize() / 2 + 1;
        total += 10 * first as i32 + last as i32;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        // input
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = part2(input);
        assert_eq!(result, 281);
    }
}