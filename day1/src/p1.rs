
pub fn solve(input_lines: &Vec<&str>){
    let mut position = 50;
    let mut zero_count: i32 = 0;
    for turn in input_lines.iter(){
        let split_idx = turn.find(|c: char| c.is_ascii_digit()).unwrap();
        let direction = &turn[..split_idx];
        let mut value: i32 = (&turn[split_idx..]).parse().unwrap();
        if direction == "L" {
            value = -value;
        }
        position = (position + value).rem_euclid(100);
        if position == 0 {
            zero_count += 1;
        }
    }
    println!("{}", zero_count);
}