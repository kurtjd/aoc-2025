pub fn part1() {
    let input = include_str!("../test.txt");
    let input: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect())
        .collect();

    let (ops, args) = input.split_last().unwrap();

    let mut grand_total = 0;
    for i in 0..ops.len() {
        let op = ops[i];
        let mut res = match op {
            "+" => 0,
            "*" => 1,
            _ => unreachable!(),
        };
        for row in args {
            let n = row[i].parse::<usize>().unwrap();
            match op {
                "+" => res += n,
                "*" => res *= n,
                _ => unreachable!(),
            }
        }

        grand_total += res;
    }

    println!("Grand total: {grand_total}");
}
