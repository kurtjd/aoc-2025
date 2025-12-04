pub fn part1() {
    let input = include_str!("../test.txt");
    let grid: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '@' {
                let mut count = 0;
                for j in y.saturating_sub(1)..=y + 1 {
                    for i in x.saturating_sub(1)..=x + 1 {
                        if (j != y || i != x)
                            && let Some(c) = grid.get(j)
                            && let Some(n) = c.chars().nth(i)
                            && n == '@'
                        {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    sum += 1;
                }
            }
        }
    }

    println!("Rolls: {sum}");
}
