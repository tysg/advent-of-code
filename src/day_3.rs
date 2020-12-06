#[derive(Debug)]
struct Game<'a> {
    map: &'a Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

pub fn solve(input: String) {
    let map = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let game = Game {
        width: map[0].len(),
        height: map.len(),
        map: &map,
    };

    let policies = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: u64 = policies
        .iter()
        .map(|(dx, dy)| count_policy_hits(*dx, *dy, &game))
        .filter(|n| n != &0)
        .product();

    println!("{}", product);
}

fn count_policy_hits(dx: usize, dy: usize, game: &Game) -> u64 {
    let mut pos = (0, 0);
    let mut count = 0;

    while let Some(npos) = next(pos, dx, dy, game.width, game.height) {
        let (nx, ny) = npos;
        if game.map[ny][nx] == '#' as u8 {
            count += 1;
        }
        pos = npos;
    }

    count
}

fn next(
    (x, y): (usize, usize),
    dx: usize,
    dy: usize,
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    let ny = y + dy;
    if ny >= height {
        return None;
    }

    Some(((x + dx) % width, ny))
}
