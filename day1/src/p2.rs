
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
        zero_count += if value > 0 {
            (position + value) / 100
        } else {
            let first_zero = if position > 0 { position } else { 100 };
            if value.abs() >= first_zero {
                1 + (value.abs() - first_zero) / 100
            } else {
                0
            }
        };
        position = (position + value).rem_euclid(100);
    }
    println!("{}", zero_count);
}