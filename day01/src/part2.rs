const DIAL_START: isize = 50;
const DIAL_MAX: isize = 100;

pub fn part2() {
    let input = include_str!("../input.txt");
    let mut zeroes = 0;
    let mut start = DIAL_START;

    for rotation in input.lines() {
        let (dir, cnt) = rotation.split_at(1);
        let cnt = cnt.parse::<isize>().unwrap();

        // Edge case where rotation left begins with dial initially on zero
        let offset = if start != 0 { 1 } else { 0 };

        match dir {
            "R" => start += cnt,
            "L" => start -= cnt,
            _ => unreachable!(),
        }

        // There's probably a sexier mathematical approach here :\
        if start == 0 {
            zeroes += 1;
        } else if start < 0 {
            zeroes += (start / DIAL_MAX).abs() + offset;
        } else {
            zeroes += start / DIAL_MAX;
        }

        start = start.rem_euclid(DIAL_MAX);
    }

    println!("Password: {zeroes}");
}
