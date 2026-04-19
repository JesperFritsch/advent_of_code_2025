


pub fn solve(input: &Vec<&str>) {
    let ranges: Vec<Vec<u64>> = input.iter()
        .next()
        .unwrap()
        .split(',')
        .map( | r | r.split('-')
            .map( | s | s.parse::<u64>()
                .unwrap()
            )
            .collect()
        )
        .collect();

    let mut invalid_ids: Vec<u64> = vec!();

    for range in ranges.iter() {
        let (r_first, r_last) = (range[0], range[1]);
        for id_n in r_first..=r_last {
            let id_s = id_n.to_string();
            let id_len = id_s.len();
            let max_p_len = id_len / 2;
            for p_len in 1..=max_p_len {
                if !(id_len % p_len == 0) { continue; }
                let first = &id_s[..p_len];
                let all_same = id_s.as_bytes().chunks(p_len).all(|c| c == first.as_bytes());
                if all_same {
                    invalid_ids.push(id_n);
                    break;
                }
            }
        } 
    }
    println!("{}", invalid_ids.into_iter().sum::<u64>());
}