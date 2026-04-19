use std::ops::RangeInclusive;

pub fn solve(input: &Vec<&str>) {
    let (ranges_lines, id_strs): (Vec<&str>, Vec<&str>) = {
        let mut parts = input.split(|line| line.is_empty());
        (
            parts.next().unwrap().to_vec(),
            parts.next().unwrap().to_vec(),
        )
    };

    let ids: Vec<u64> = id_strs.iter()
        .map(|id| id.parse::<u64>().unwrap())
        .collect();

    let ranges: Vec<RangeInclusive<u64>> = ranges_lines.iter()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
        })
        .collect();

    let count = ids.iter()
        .filter(|id| ranges.iter().find(|r| r.contains(id)).is_some())
        .count();

    println!("{}", count);
}