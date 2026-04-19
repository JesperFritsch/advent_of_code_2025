
use log::{
    debug,
    info,
};

fn first_max_val_idx(bytes: &[u8]) -> usize {
    let max_val = bytes.iter().max().unwrap();
    bytes.iter().position(|v| v == max_val).unwrap()
}


pub fn solve(input: &Vec<&str>) {
    let num_cells = 12;
    
    let mut digits: Vec<u8> = vec![0; num_cells];
    let mut idxs: Vec<usize> = vec![0; num_cells];
    let mut sum = 0u64;

    for b_bank_s in input.iter() {
        let b_bank: Vec<u8> = b_bank_s.bytes().map(|b| b - b'0').collect();
        let bank_len = b_bank.len();
        let mut next_start_idx = 0;
        for d_place in 0..num_cells {
            let max_rel_idx = first_max_val_idx(&b_bank[next_start_idx..=(bank_len - (num_cells - d_place))]);
            let current_idx = next_start_idx + max_rel_idx;

            idxs[d_place] = current_idx;
            digits[d_place] = b_bank[current_idx];
            next_start_idx = current_idx + 1;

        }
        let num = digits.iter().fold(0u64, |acc, &d| (acc * 10) + d as u64);
        sum += num;
        let indicators: String = (0..bank_len).map(|i| if idxs.contains(&i) {'^'} else {' '}).collect();
        debug!("{} - {}", b_bank_s, num);
        debug!("{}", indicators);
    }
    info!("{}", sum);
}