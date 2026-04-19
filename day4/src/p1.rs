
const AJACENT: [(isize, isize); 8] = [
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0),           ( 1,  0),
    (-1,  1), ( 0,  1), ( 1,  1)
];


struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize
}


impl Grid {

    fn new(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };
        Grid { grid, width, height }
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        let ux = usize::try_from(x).ok()?;
        let uy = usize::try_from(y).ok()?;
        self.grid.get(uy)?.get(ux).copied()
    }

    fn neighbors(&self, x: isize, y: isize) -> Vec<char> {
        AJACENT.iter()
            .filter_map(|(dx, dy)| self.get(x + dx, y + dy))
            .collect()
    }

}


pub fn solve(input: &Vec<&str>) {
    let grid_vec: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let grid = Grid::new(grid_vec);

    let width = grid.width;
    let height = grid.height;
    let mut sum: u32 = 0;
    for y in 0..height {
        for x in 0..width {
            let ajacent_chars = grid.neighbors(x as isize, y as isize);
            let rolls_count = ajacent_chars.iter().fold(0, |acc, c| acc + (if *c == '@' { 1 } else { 0 }));
            if rolls_count < 4 && grid.get(x as isize, y as isize).unwrap() == '@'{
                sum += 1;
            };
        }
    }
    println!("{}", sum);
}