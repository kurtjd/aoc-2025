use itertools::Itertools;

pub fn part1() {
    let input = include_str!("../test.txt");
    let ranges = input.split(',').map(|s| {
        s.split('-')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple::<(usize, usize)>()
            .unwrap()
    });

    let mut sum = 0;
    for (start, end) in ranges {
        for i in start..=end {
            let n = i.to_string();
            let (l, r) = n.split_at(n.len() / 2);
            if l == r {
                sum += i;
            }
        }
    }

    println!("Sum: {sum}");
}
