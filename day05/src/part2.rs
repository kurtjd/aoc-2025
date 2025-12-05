use itertools::Itertools;

pub fn part2() {
    let input = include_str!("../test.txt");
    let (ranges, _) = input.split("\n\n").collect_tuple().unwrap();

    let mut ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|l| {
            l.split('-')
                .map(|r| r.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    // Sort the input by start of range in ascending input to prepare for finding overlaps
    ranges.sort();

    // Now we just create a new vec with overlaps fixed
    // As in, we condense any ranges when possible
    let mut fixed: Vec<(usize, usize)> = vec![ranges[0]];
    let mut fixed_i = 0;
    for range in ranges.iter().skip(1) {
        let prev = fixed[fixed_i];
        // Thsi range is completely overlapped by previous, so drop it
        if range.0 <= prev.1 && range.1 <= prev.1 {
            // NOP
            // This range partially overlaps, so update bounds of previous
        } else if range.0 <= prev.1 && range.1 > prev.1 {
            fixed[fixed_i].1 = range.1;
        // This range does not overlap so add it back to vec
        } else {
            fixed.push(*range);
            fixed_i += 1;
        }
    }

    // Now we just find the size of all ranges and add them up
    let mut fresh = 0;
    for range in fixed.iter() {
        fresh += (range.1 - range.0) + 1;
    }

    println!("Fresh: {fresh}");
}
