#[derive(Debug, PartialEq)]
struct Ship {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

pub fn solve(input: &str) {
    let start_state = Ship {
        x: 0,
        y: 0,
        dx: 10,
        dy: 1
    };
    let result_state = input.lines().fold(start_state, |acc, i| next_state(acc, i));

    println!("{:?}", result_state);
    println!("{}", result_state.x.abs() + result_state.y.abs());
}

fn next_state(s: Ship, action: &str) -> Ship {
    let num = action[1..].parse::<i32>().unwrap();
    match &action[0..1] {
        "N" => Ship { dy: s.dy + num, ..s },
        "E" => Ship { dx: s.dx + num, ..s },
        "S" => Ship { dy: s.dy - num, ..s },
        "W" => Ship { dx: s.dx - num, ..s },
        "L" => {
            let (nx, ny) = turn((s.dx,s.dy), -num);
            Ship {
                dx: nx, dy:ny,
            ..s
        }},
        "R" => {
            let (nx, ny) = turn((s.dx,s.dy), num);
            Ship {
                dx: nx, dy:ny,
            ..s
            }
        },
        "F" => Ship {
            x: s.x + num * s.dx,
            y: s.y + num * s.dy,
            ..s
        },
        _ => panic!("unexpected token"),
    }
}

fn turn(waypoint: (i32, i32), rotation: i32) -> (i32,i32){
    // assume all rotation % 90 == 0
    assert_eq!(rotation % 90, 0);
    let turns = rotation / 90;
    let (x,y) = waypoint;

    match (turns + 4) % 4 {
        0 => (x,y),
        1 => (y,-x),
        2 => (-x,-y),
        3 => (-y,x),
        _ => panic!("mathematically impossible")
    }
}


