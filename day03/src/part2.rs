const JOLTAGE_WIDTH: usize = 12;

pub fn part2() {
    let input = include_str!("../test.txt");

    let mut sum = 0;
    for bank in input.lines() {
        let mut window = (bank.len() - JOLTAGE_WIDTH) + 1;
        let mut joltage = String::new();

        // Do a sliding and shrinking window
        // Basically the window is initially the difference between bank length and joltage width in digits
        // Find the max digit in the window, then add that to output
        // Reduce the window size by the offset of max digit from window start point
        // Then slide window to 1 after the max digit we just found
        // Then repeat
        let mut i = 0;
        while joltage.len() < JOLTAGE_WIDTH {
            let slice = &bank[i..i + window];

            let mut d = '0';
            let mut d_j = 0;
            for (j, c) in slice.chars().enumerate() {
                if c > d {
                    d = c;
                    d_j = j;
                }
            }

            joltage.push(d);
            i += d_j + 1;

            if window > 1 {
                window -= d_j;
            }
        }

        let joltage = joltage.parse::<usize>().unwrap();
        sum += joltage;
    }

    println!("Total joltage: {sum}");
}
