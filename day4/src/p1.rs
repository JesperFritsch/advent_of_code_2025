use common::Grid;


pub fn solve(input: &Vec<&str>) {
    let grid_vec: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let grid = Grid::new(grid_vec);

    let width = grid.width;
    let height = grid.height;
    let mut sum: u32 = 0;
    for y in 0..height {
        for x in 0..width {
            let ajacent_chars: Vec<char> = grid.neighbors(x, y).into_iter().flatten().collect();
            let rolls_count = ajacent_chars.iter().fold(0, |acc, c| acc + (if *c == '@' { 1 } else { 0 }));
            if rolls_count < 4 && grid.get(x, y).unwrap() == '@'{
                sum += 1;
            };
        }
    }
    println!("{}", sum);
}