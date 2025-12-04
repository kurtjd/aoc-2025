fn reduce(grid: Vec<String>) -> (usize, Vec<String>) {
    let mut sum = 0;
    let mut reduced = grid.clone();

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
                    // SAFETY: LOL
                    unsafe { reduced[y].as_bytes_mut()[x] = b'.' };
                }
            }
        }
    }

    (sum, reduced)
}

pub fn part2() {
    let input = include_str!("../test.txt");
    let grid: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    let mut global_sum = 0;
    let (mut sum, mut grid) = reduce(grid);
    global_sum += sum;
    while sum != 0 {
        (sum, grid) = reduce(grid);
        global_sum += sum;
    }

    println!("Rolls: {global_sum}");
}
