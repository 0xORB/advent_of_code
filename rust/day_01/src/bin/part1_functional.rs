fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let result = input
    .lines()
    .map(|line| {
        let mut it =
            line.chars().filter_map(|character| {
                character.to_digit(10)
            });
        let first = it.next().expect("should be a number");

        match it.last() {
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}"),
        }
        .parse::<i32>()
        .expect("should be a valid number")
    })
    .sum::<i32>();

    return result

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // input
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142);
    }
}