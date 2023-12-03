use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_arg = 1;
    let file = &args[file_arg];
    let part = &args[2];

    println!("You're reading file: {}", file);
    println!("Showing part: {} solution", part);

    match part.as_str() {
        "a" => part_a(&file),
        "b" => part_b(&file),
        _ => println!("Invalid part"),
    }
}

fn part_a(file: &str) {
    let mut result = 0;

    for line in read_lines(file) {
        let num = collect_nums(&line);
        result += num;
    }

    println!("Result: {}", result);
}

fn part_b(file: &str) {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut result = 0;

    for mut line in read_lines(file) {
        let digits = digits(&line);

        while !nums.iter().any(|x| line.starts_with(x))
            && !line.chars().next().unwrap().is_numeric()
        {
            line = line[1..].to_string();
        }

        while !nums.iter().any(|x| line.ends_with(x)) && !line.chars().last().unwrap().is_numeric()
        {
            line = line[..line.len() - 1].to_string();
        }

        let mut first = 0;
        let mut last = 0;
        for (i, num) in nums.iter().enumerate() {
            if line.starts_with(num) {
                first = i + 1;
            }
            if line.ends_with(num) {
                last = i + 1;
            }
        }

        if line.chars().next().unwrap().is_numeric() {
            first = digits[0];
        }

        if line.chars().last().unwrap().is_numeric() {
            last = digits[digits.len() - 1];
        }

        let sum = first * 10 + last;
        result += sum;
    }

    println!("Result: {}", result);
}

fn digits(line: &str) -> Vec<usize> {
    return line
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as usize)
        .collect();
}

fn read_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn collect_nums(line: &str) -> i32 {
    let (first, last) = first_and_last_digits(line);
    let value = format!("{}{}", first, last);

    return value.parse::<i32>().unwrap();
}

fn first_and_last_digits(line: &str) -> (u32, u32) {
    let (mut first, mut last) = (0, 0);

    for char in line.chars() {
        if char.is_numeric() {
            if first == 0 {
                first = char.to_digit(10).unwrap();
            }
            last = char.to_digit(10).unwrap();
        }
    }

    return (first, last);
}
