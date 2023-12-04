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
    let mut nums = Vec::new();
    let mut result_nums = Vec::new();
    let lines: std::str::Lines<'_>= input.lines();
    for line in lines {
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