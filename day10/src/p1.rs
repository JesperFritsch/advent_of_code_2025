

use std::fmt;
use itertools::Itertools;

struct Machine {
    light_mask: u32,
    btn_masks: Vec<u32>
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut widest: usize = self.btn_masks.iter().map(|n| 32 - n.leading_zeros()).max().unwrap() as usize;
        widest = widest.max(32 - self.light_mask.leading_zeros() as usize);
        write!(f, "({:0widest$b}, [{}])", self.light_mask, self.btn_masks.iter().map(|n| format!("{:0widest$b}", n)).collect::<Vec<String>>().join(", "))
    }
}


pub fn solve(input: &Vec<&str>) {
    
    let machines: Vec<Machine> = input.iter()
        .map( |line| {
            let mut parts: Vec<&str> = line.split(' ').collect();
            let light_mask: u32 = parts.remove(0)
                .trim_matches(|c| c == '[' || c == ']')
                .match_indices('#')
                .fold(0u32, |acc, (i, _)| acc | 1 << i);
            let btn_masks: Vec<u32> = parts.iter()
                .filter(|p| p.chars().nth(0).unwrap() == '(')
                .map(|s| s.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|n|{ 
                        n.parse::<u32>().unwrap()
                    })
                    .fold(0u32, |acc, c| acc | (1 << c))
                ).collect();
            Machine {light_mask, btn_masks}
        }).collect();

    let result: u32 = machines.iter()
        .map(|m| (1..=m.btn_masks.len())
            .find(|&i| m.btn_masks.iter()
                .permutations(i)
                .any(|masks| masks.iter().fold(0u32, |acc, m| acc ^ **m) == m.light_mask)
            ).unwrap() as u32
        ).sum();
    println!("p1: {}", result);

}