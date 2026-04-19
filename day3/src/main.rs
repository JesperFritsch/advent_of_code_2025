mod p1;
mod p2;

fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
    let input = include_str!("../input.txt").lines().collect();
    p1::solve(&input);
    p2::solve(&input);
}
