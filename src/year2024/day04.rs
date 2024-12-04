type Input = Vec<Vec<char>>;

const DIRS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn part1(input: &Input) -> usize {
    let ROWS = input.len();
    let COLS = input[0].len();

    let mut res = 0;

    for r in 0..ROWS {
        for c in 0..COLS {
            if input[r][c] == 'X' {
                for dir in DIRS {
                    if dfs(input, 0, (r as i32, c as i32), dir) {
                        res += 1;
                    }
                }
            }
        }
    }
    res
}

fn dfs(input: &Input, char_index: usize, (i, j): (i32, i32), dir: (i32, i32)) -> bool {
    if char_index == 4 {
        return true;
    }
    let ROWS = input.len() as i32;
    let COLS = input[0].len() as i32;
    if i < 0 || j < 0 || i >= ROWS || j >= COLS || input[i as usize][j as usize] != XMAS[char_index]
    {
        return false;
    }
    dfs(input, char_index + 1, (i + dir.0, j + dir.1), dir)
}

pub fn part2(input: &Input) -> usize {
    let ROWS = input.len();
    let COLS = input[0].len();
    let mut res = 0;

    for r in 1..ROWS - 1 {
        for c in 1..COLS - 1 {
            if input[r][c] == 'A' && check_x(input, r, c) {
                res += 1;
            }
        }
    }
    res
}

const DIAGS: [((i32, i32), (i32, i32)); 2] = [((1, 1), (-1, -1)), ((1, -1), (-1, 1))];
fn check_x(input: &Input, i: usize, j: usize) -> bool {
    for ((i1, j1), (i2, j2)) in DIAGS {
        let c1 = input[(i as i32 + i1) as usize][(j as i32 + j1) as usize];
        let c2 = input[(i as i32 + i2) as usize][(j as i32 + j2) as usize];
        match (c1, c2) {
            ('M', 'S') | ('S', 'M') => {}
            _ => return false,
        }
        // check that M on one side and A on the other
    }
    true
}
