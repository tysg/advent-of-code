#[derive(Debug, Copy, Clone, PartialEq)]
enum Square {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug)]
struct Grid {
    map: Vec<Vec<Square>>,
    width: usize,
    height: usize,
}

impl From<char> for Square {
    fn from(i: char) -> Square {
        match i {
            'L' => Square::Empty,
            '.' => Square::Floor,
            '#' => Square::Occupied,
            _ => panic!("unexpected token"),
        }
    }
}

pub fn solve(input: &str) {
    let map = input
        .lines()
        .map(|l| l.chars().map(Square::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = map[0].len();
    let height = map.len();
    let mut grid = Grid { map, width, height };
    loop {
        let (new_grid, count) = next_state(grid);
        if count == 0 {
            println!("{}", count_occupied(&new_grid));
            return;
        } else {
            grid = new_grid
        }
    }
}

fn count_occupied(g: &Grid) -> usize {
    g.map
        .iter()
        .map(|r| r.iter().filter(|e| *e == &Square::Occupied).count())
        .sum()
}

fn next_state(grid: Grid) -> (Grid, u32) {
    let mut new_grid = vec![vec![Square::Empty; grid.width]; grid.height];
    let mut count = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            new_grid[y][x] = if let Some(p) = calc_cell_part2(&grid, x, y) {
                if p != grid.map[y][x] {
                    count += 1;
                }
                p
            } else {
                grid.map[y][x]
            };
        }
    }

    (
        Grid {
            map: new_grid,
            ..grid
        },
        count,
    )
}

fn _calc_cell_part1(grid: &Grid, x: usize, y: usize) -> Option<Square> {
    let directions = [-1, 0, 1];
    let mut adj = Vec::new();
    for dx in &directions {
        for dy in &directions {
            if *dx == 0 && *dy == 0 {
                continue;
            }
            let x = x as i32 + dx;
            let y = y as i32 + dy;

            if x >= 0 && x < grid.width as i32 && y >= 0 && y < grid.height as i32 {
                adj.push(grid.map[y as usize][x as usize]);
            }
        }
    }

    match grid.map[y][x] {
        Square::Empty => {
            if adj.iter().all(|s| *s != Square::Occupied) {
                return Some(Square::Occupied);
            }
        }
        Square::Occupied => {
            if adj.iter().filter(|s| *s == &Square::Occupied).count() >= 4 {
                return Some(Square::Empty);
            }
        }
        _ => (),
    }

    None
}

fn calc_cell_part2(grid: &Grid, x: usize, y: usize) -> Option<Square> {
    let directions = [-1, 0, 1];
    let mut adj = Vec::new();
    for dx in &directions {
        for dy in &directions {
            if *dx == 0 && *dy == 0 {
                continue;
            }

            let mut scale = 1;
            loop {
                let x = x as i32 + dx * scale;
                let y = y as i32 + dy * scale;

                if x < 0 || x >= grid.width as i32 || y < 0 || y >= grid.height as i32 {
                    break;
                }

                let this = grid.map[y as usize][x as usize];
                if this != Square::Floor {
                    adj.push(this);
                    break;
                }

                scale += 1;
            }
        }
    }

    match grid.map[y][x] {
        Square::Empty => {
            if adj.iter().all(|s| *s != Square::Occupied) {
                return Some(Square::Occupied);
            }
        }
        Square::Occupied => {
            if adj.iter().filter(|s| *s == &Square::Occupied).count() >= 5 {
                return Some(Square::Empty);
            }
        }
        _ => (),
    }

    None
}
