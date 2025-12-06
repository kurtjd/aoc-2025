pub fn part2() {
    let input = include_str!("../test.txt");
    let input = input.lines().collect::<Vec<&str>>();

    let (ops, args) = input.split_last().unwrap();
    let ops = ops.split_ascii_whitespace().collect::<Vec<&str>>();

    let arg_len = args[0].len();

    // Holds the parsed argument after de-cephalopodizing
    let mut new_args: Vec<Vec<usize>> = Vec::new();

    // Start from end of rows
    let mut cur = Vec::new();
    for i in (0..arg_len).rev() {
        let mut n = String::new();

        // If not whitespace, append to string of numebr we are forming
        for row in args {
            let c = row.chars().nth(i).unwrap();
            if c.is_ascii_digit() {
                n.push(c);
            }
        }

        // Does our string repr a number? If so its a arg
        if let Ok(n) = n.parse::<usize>() {
            cur.push(n);
        // If not, we hit all whitespaces, so start new arg list
        } else {
            new_args.push(cur);
            cur = Vec::new();
        }
    }
    // Catch the last one
    new_args.push(cur);

    // Now we have de-cephalopodized numbers, so just simply perform the corresponding op
    let mut grand_total = 0;
    for (i, args) in new_args.iter().rev().enumerate() {
        let op = ops[i];
        let mut res = match op {
            "+" => 0,
            "*" => 1,
            _ => unreachable!(),
        };

        for arg in args.iter() {
            match op {
                "+" => res += arg,
                "*" => res *= arg,
                _ => unreachable!(),
            }
        }

        grand_total += res;
    }

    println!("Grand total: {grand_total}");
}
