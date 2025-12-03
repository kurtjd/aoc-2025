pub fn part1() {
    let input = include_str!("../test.txt");

    let mut sum = 0;
    for bank in input.lines() {
        let mut max1 = 0;
        let mut max1_i = 0;
        let mut max2 = 0;

        for (i, battery) in bank.chars().take(bank.len() - 1).enumerate() {
            let val = battery.to_digit(10).unwrap();
            if val > max1 {
                max1 = val;
                max1_i = i;
            }
        }

        for battery in bank[max1_i + 1..].chars() {
            let val = battery.to_digit(10).unwrap();
            max2 = std::cmp::max(max2, val);
        }

        let joltage = format!("{max1}{max2}").parse::<usize>().unwrap();
        sum += joltage;
    }

    println!("Total joltage: {sum}");
}
