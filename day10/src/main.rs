mod p1;
mod p2;

fn main() {

    let input = include_str!("../test_input.txt").lines().collect();
    p1::solve(&input);
    p2::solve(&input);
}
