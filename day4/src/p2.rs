
const ADJACENT: [(isize, isize); 8] = [
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0),           ( 1,  0),
    (-1,  1), ( 0,  1), ( 1,  1)
];


struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize
}


struct GridIter<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}


impl<'a> Iterator for GridIter<'a> {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height {
            return None;
        }

        let result = (
            self.x as usize, 
            self.y as usize,
            self.grid.get(self.x, self.y).unwrap(),
        );
        self.x += 1;
        if self.x >= self.grid.width {
            self.y += 1;
            self.x = 0;
        }
        Some(result)
    }
}


impl Grid {

    fn new(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };
        Grid { grid, width, height }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.grid.get(y)?.get(x).copied()
    }

    fn set(& mut self, x: usize, y: usize, val: char) -> bool {
        if !self.get(x, y).is_some() {return false};
        self.grid[y as usize][x as usize] = val;
        true
    }

    fn count_neighbors(&self, x: usize, y: usize, target: char) -> usize {
        ADJACENT.iter()
            .filter(|(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                usize::try_from(nx).ok()
                    .zip(usize::try_from(ny).ok())
                    .and_then(|(ux, uy)| self.get(ux, uy))
                    == Some(target)
            })
            .count()
    }

    fn iter(&self) -> GridIter<'_>{
        GridIter { grid: self, x: 0, y: 0 }
    }

}


pub fn solve(input: &Vec<&str>) {
    let grid_vec: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let mut grid = Grid::new(grid_vec);

    let mut sum: u32 = 0;
    let mut to_remove: Vec<(usize, usize)> = vec!();
    loop {

        let roll_coords = grid.iter()
            .filter(|(_, _, c)| *c == '@')
            .map(|(x, y, _)| (x, y));
            
        for (x, y) in roll_coords {
            if grid.count_neighbors(x, y, '@') < 4 {
                to_remove.push((x, y));
            }
        }
        if !to_remove.is_empty() {
            sum += to_remove.len() as u32;
            for (x, y) in to_remove.iter() {
                grid.set(*x, *y, '.');
            }
            to_remove.clear();
        } else {
            break;
        }
    }
    println!("{}", sum);
}