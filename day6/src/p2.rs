

pub fn solve(input: &Vec<&str>) {

    let rev_lines: Vec<Vec<char>> = input.iter().map(|l| l.chars().rev().collect()).collect();
    let mut ops: Vec<char> = vec!();
    let mut numbers: Vec<Vec<u64>> = vec!();

    let mut num_str = String::new();
    let mut nums: Vec<u64> = vec!();
    for i in 0..rev_lines[0].len() {
        num_str.clear();
        for j in 0..rev_lines.len() - 1 {
            num_str.push(rev_lines[j][i]);
        }
        if num_str.trim().is_empty() {
            continue;
        }
        nums.push(num_str.trim().parse::<u64>().unwrap());
        let op = rev_lines.last().unwrap()[i];
        if op != ' ' {
            ops.push(op);
            numbers.push(nums.clone());
            nums.clear();
        }   
    }

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

    println!("{}", sum);

}