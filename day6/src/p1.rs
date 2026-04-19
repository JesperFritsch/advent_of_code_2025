

pub fn solve(input: &Vec<&str>) {
    let numbers: Vec<Vec<u64>> = input[..input.len() - 1].iter()
        .map(|l| l.split_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect())
        .collect();

    let numbers: Vec<Vec<u64>> = (0..numbers[0].len())
        .map(|col| numbers.iter().map(|row| row[col]).collect())
        .collect();

    let ops: Vec<char> = input.iter()
        .last()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();

    fn op(a: u64, b: u64, op: char) -> u64 {
        match op {
            '*' => a * b,
            '+' => a + b,
            _ => panic!("No operand {}", op)
        }
    }

    let sum: u64 = numbers.into_iter()
        .enumerate()
        .map(|(i, nums)| nums.into_iter().reduce(|acc, n| op(acc, n, ops[i])).unwrap())
        .sum();

    println!("{:?}", sum);
}