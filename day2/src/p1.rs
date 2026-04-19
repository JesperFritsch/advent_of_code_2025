


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
            let hl = id_s.len() / 2;
            if id_s[..hl] == id_s[hl..] {
                invalid_ids.push(id_n);
            }
        } 
    }
    println!("{}", invalid_ids.into_iter().sum::<u64>());
}