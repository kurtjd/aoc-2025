const DIAL_START: isize = 50;
const DIAL_MAX: isize = 100;

pub fn part1() {
    let input = include_str!("../input.txt");
    let mut zeroes = 0;
    let mut start = DIAL_START;

    for rotation in input.lines() {
        let (dir, cnt) = rotation.split_at(1);
        let cnt = cnt.parse::<isize>().unwrap();

        match dir {
            "R" => start += cnt,
            "L" => start -= cnt,
            _ => unreachable!(),
        }

        start = start.rem_euclid(DIAL_MAX);
        if start == 0 {
            zeroes += 1;
        }
    }

    println!("Password: {zeroes}");
}
