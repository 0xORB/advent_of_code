use std::collections::HashMap;

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
    let mut num_map:HashMap<String, i32> = HashMap::new();
    num_map.insert(String::from("zero"), 0);
    num_map.insert(String::from("one"), 1);
    num_map.insert(String::from("two"), 2);
    num_map.insert(String::from("three"), 3);
    num_map.insert(String::from("four"), 4);
    num_map.insert(String::from("five"), 5);
    num_map.insert(String::from("six"), 6);
    num_map.insert(String::from("seven"), 7);
    num_map.insert(String::from("eight"), 8);
    num_map.insert(String::from("nine"), 9);
    let mut nums = Vec::new();
    let mut result_nums = Vec::new();
    let mut lines: std::str::Lines<'_>= input.lines();
    for line in lines.next() {
        // found_keys vector will be backwards, so the first value found will be at the end
        // and the last value found will be at the beginning
        let found_keys: Vec<_> = num_map.keys().filter(|&key| line.contains(key)).collect();
        if !found_keys.is_empty() {
            println!("Keys found in the string: {:?}", found_keys);
        } else {
            println!("No keys found in the string.");
        }
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                nums.push(digit);
            }
        }
        // println!("{:?}", nums);
        if nums.len() >= 2 {
            let line_value = nums[0].to_string() + nums[nums.len() - 1].to_string().as_str();
            // println!("{}", line_value);
            // println!("-----\n\n");
            // let parsed_num = line_value.parse::<i32>().unwrap();
            result_nums.push(line_value.parse::<i32>().unwrap());
        } else {
            let line_value = nums[0].to_string() + nums[0].to_string().as_str();
            result_nums.push(line_value.parse::<i32>().unwrap());
            // println!("{}", line_value);
            // println!("-----\n\n");
        }
        nums.clear();
        // println!("{:?}", v)
    }
    // let result: i32 = result_nums.iter().sum();
    // println!("{:?}", result_nums);
    result_nums.iter().sum()
    // println!("{}", result);
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

/*

7jlncfksix7rjgrpglmn9

7 is a number.
j is not a number.
l is not a number.
n is not a number.
c is not a number.
f is not a number.
k is not a number.
s is not a number.
i is not a number.
x is not a number.
7 is a number.
r is not a number.
j is not a number.
g is not a number.
r is not a number.
p is not a number.
g is not a number.
l is not a number.
m is not a number.
n is not a number.
9 is a number.
*/