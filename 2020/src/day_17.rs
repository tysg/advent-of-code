use std::collections::HashMap;
type Loc = (i32, i32, i32, i32);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cube {
    Active,
    Inactive,
}

impl From<char> for Cube {
    fn from(i: char) -> Cube {
        match i {
            // 'L' => Square::Empty,
            '.' => Cube::Inactive,
            '#' => Cube::Active,
            _ => panic!("unexpected token"),
        }
    }
}

pub fn solve(input: &str) {
    let map = input
        .lines()
        .map(|l| l.chars().map(Cube::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let len = map.len();
    let mut space = HashMap::new();

    for i in 0..len {
        for j in 0..len {
            space.insert((i as i32, j as i32, 0, 0), map[i][j]);
        }
    }

    for _ in 0..6 {
        let mut new_space = HashMap::new();
        // add immeidiate neighbors
        for cube in space.clone().iter() {
            let neighbors = neighbors(cube.0);
            for n in neighbors {
                if !space.contains_key(&n) {
                    space.insert(n, Cube::Inactive);
                }
            }
        }

        for cube in space.iter() {
            let loc = cube.0;
            let v = cube.1;

            // judge neighbors
            let num_active_neighbors = neighbors(loc)
                .iter()
                .filter(|n| space.get(n).unwrap_or(&Cube::Inactive) == &Cube::Active)
                .count();

            new_space.insert(
                *loc,
                match v {
                    Cube::Active if (2..=3).contains(&num_active_neighbors) => Cube::Active,
                    Cube::Inactive if num_active_neighbors == 3 => Cube::Active,
                    _ => Cube::Inactive,
                },
            );
        }
        space = new_space;
    }

    let active_cubes = space.iter().filter(|(_, v)| *v == &Cube::Active).count();

    println!("{}", active_cubes);
}

fn neighbors(loc: &Loc) -> Vec<Loc> {
    let mut neighbors = Vec::new();
    let dir = vec![-1 as i32, 0 as i32, 1 as i32];

    for dx in &dir {
        for dy in &dir {
            for dz in &dir {
                for dw in &dir {
                    if dx == &0 && dy == &0 && dz == &0 && dw == &0 {
                        continue;
                    }
                    neighbors.push((loc.0 + dx, loc.1 + dy, loc.2 + dz, loc.3 + dw));
                }
            }
        }
    }
    neighbors
}
