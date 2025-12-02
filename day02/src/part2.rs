use itertools::Itertools;

pub fn part2() {
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
            let n = n.as_bytes();

            for x in 1..n.len() {
                if n.chunks(x).all_equal() {
                    sum += i;
                    break;
                }
            }
        }
    }

    println!("Sum: {sum}");
}
