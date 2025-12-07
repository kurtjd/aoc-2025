use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
}

fn find_start(grid: &[Vec<char>]) -> Pos {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                return Pos { x, y };
            }
        }
    }

    Pos { x: 0, y: 0 }
}

fn split(cache: &mut HashMap<Pos, usize>, grid: &mut [Vec<char>], pos: Pos) -> usize {
    // If we've already traveled this path, get result from cache
    if let Some(r) = cache.get(&pos) {
        *r
    // Otherwise its a new path...
    } else if let Some(row) = grid.get(pos.y)
        && let Some(c) = row.get(pos.x)
    {
        // We splitting?
        if *c == '^' {
            // And we in bounds?
            let left = if let Some(_c) = grid.get(pos.y).unwrap().get(pos.x - 1) {
                // Then just split, no questions asked
                split(cache, grid, pos.left())
            } else {
                0
            };
            // Same logic as above
            let right = if let Some(_c) = grid.get(pos.y).unwrap().get(pos.x + 1) {
                split(cache, grid, pos.right())
            } else {
                0
            };
            let res = left + right;
            cache.insert(pos, res);
            res
        // If not splitting, then recurse down
        } else {
            split(cache, grid, pos.down())
        }
    // We've reached bottom of grid
    } else {
        cache.insert(pos, 1);
        // Serves as our base case, bubble up as this path taken
        1
    }
}

pub fn part2() {
    let input = include_str!("../test.txt");
    let grid = input.lines().collect::<Vec<&str>>();
    let mut grid = grid
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Find the first ^
    let start = find_start(&grid);

    // Keep a cache of paths we've already taken to speed things up immensely
    let mut cache = HashMap::<Pos, usize>::new();
    // Rewcurse with cache
    let res = split(&mut cache, &mut grid, start);

    println!("Res: {res}");
}
