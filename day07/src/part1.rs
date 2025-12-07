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

fn split(grid: &mut [Vec<char>], pos: Pos) -> usize {
    if let Some(row) = grid.get(pos.y)
        && let Some(c) = row.get(pos.x)
    {
        let mut did_split = false;
        // We need to split here
        if *c == '^' {
            // We in bounds?
            let left = if let Some(c) = grid.get_mut(pos.y).unwrap().get_mut(pos.x - 1) {
                // We have room to split?
                if *c == '.' {
                    // Then split and mark this side taken
                    *c = '|';
                    did_split = true;
                    split(grid, pos.left())
                } else {
                    0
                }
            } else {
                0
            };
            // Same logic as above but for right side
            let right = if let Some(c) = grid.get_mut(pos.y).unwrap().get_mut(pos.x + 1) {
                if *c == '.' {
                    *c = '|';
                    did_split = true;
                    split(grid, pos.right())
                } else {
                    0
                }
            } else {
                0
            };

            // Edge case where we couldn't split because we had two neigbors split before us
            let s = if did_split { 1 } else { 0 };
            s + left + right
        // Don't need to split, so just recurse downward
        } else {
            split(grid, pos.down())
        }
    // We've reached the bottom
    } else {
        0
    }
}

pub fn part1() {
    let input = include_str!("../test.txt");
    let grid = input.lines().collect::<Vec<&str>>();
    let mut grid = grid
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Find the first ^
    let start = find_start(&grid);
    // Then recurse
    let res = split(&mut grid, start);

    println!("Res: {res}");
}
