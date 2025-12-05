use itertools::Itertools;

pub fn part1() {
    let input = include_str!("../test.txt");
    let (ranges, ids) = input.split("\n\n").collect_tuple().unwrap();

    let ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|l| {
            l.split('-')
                .map(|r| r.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let ids: Vec<usize> = ids.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    let mut fresh = 0;
    for id in ids {
        for range in ranges.iter() {
            if (range.0..=range.1).contains(&id) {
                fresh += 1;
                break;
            }
        }
    }

    println!("Fresh: {fresh}");
}
