#[derive(Debug, PartialEq)]
struct Ship {
    facing: u8, // North: 0; East: 1, South: 2, West: 3;
    x: i32,
    y: i32,
}

pub fn solve(input: &str) {
    let start_state = Ship {
        facing: 1,
        x: 0,
        y: 0,
    };
    let result_state = input.lines().fold(start_state, |acc, i| next_state(acc, i));

    println!("{:?}", result_state);
    println!("{}", result_state.x.abs() + result_state.y.abs());
}

fn next_state(s: Ship, action: &str) -> Ship {
    let num = action[1..].parse::<i32>().unwrap();
    match &action[0..1] {
        "N" => Ship { y: s.y + num, ..s },
        "E" => Ship { x: s.x + num, ..s },
        "S" => Ship { y: s.y - num, ..s },
        "W" => Ship { x: s.x - num, ..s },
        "L" => Ship {
            facing: turn(s.facing, -num),
            ..s
        },
        "R" => Ship {
            facing: turn(s.facing, num),
            ..s
        },
        "F" => Ship {
            x: s.x + num * dx(s.facing),
            y: s.y + num * dy(s.facing),
            ..s
        },
        _ => panic!("unexpected token"),
    }
}

fn turn(facing: u8, rotation: i32) -> u8 {
    // assume all rotation % 90 == 0
    assert_eq!(rotation % 90, 0);
    let turns = rotation / 90;
    ((facing as i32 + turns + 4) % 4) as u8
}

fn dx(facing: u8) -> i32 {
    match facing {
        1 | 3 => 2 - facing as i32,
        _ => 0,
    }
}
fn dy(facing: u8) -> i32 {
    match facing {
        0 | 2 => 1 - facing as i32,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_right() {
        assert_eq!(turn(3, 180), 1);
    }

    #[test]
    fn turn_left() {
        assert_eq!(turn(0, -180), 2);
    }

    #[test]
    fn next_state_fwd_n() {
        assert_eq!(
            next_state(
                Ship {
                    x: 0,
                    y: 0,
                    facing: 0
                },
                "F11"
            ),
            Ship {
                x: 0,
                y: 11,
                facing: 0
            }
        );
    }

    #[test]
    fn next_state_fwd_e() {
        assert_eq!(
            next_state(
                Ship {
                    x: 0,
                    y: 0,
                    facing: 1
                },
                "F11"
            ),
            Ship {
                x: 11,
                y: 0,
                facing: 1
            }
        );
    }
    #[test]
    fn next_state_fwd_s() {
        assert_eq!(
            next_state(
                Ship {
                    x: 0,
                    y: 0,
                    facing: 2
                },
                "F11"
            ),
            Ship {
                x: 0,
                y: -11,
                facing: 2
            }
        );
    }
    #[test]
    fn next_state_fwd_w() {
        assert_eq!(
            next_state(
                Ship {
                    x: 0,
                    y: 0,
                    facing: 3
                },
                "F11"
            ),
            Ship {
                x: -11,
                y: 0,
                facing: 3
            }
        );
    }
    #[test]
    fn next_state_n() {
        assert_eq!(
            next_state(
                Ship {
                    x: 0,
                    y: 0,
                    facing: 0
                },
                "N11"
            ),
            Ship {
                x: 0,
                y: 11,
                facing: 0
            }
        );
    }
    #[test]
    fn next_state_e() {
        assert_eq!(
            next_state(
                Ship {
                    x: 0,
                    y: 0,
                    facing: 0
                },
                "E11"
            ),
            Ship {
                x: 11,
                y: 0,
                facing: 0
            }
        );
    }
    #[test]
    fn next_state_s() {
        assert_eq!(
            next_state(
                Ship {
                    x: 0,
                    y: 0,
                    facing: 0
                },
                "S11"
            ),
            Ship {
                x: 0,
                y: -11,
                facing: 0
            }
        );
    }

    #[test]
    fn next_state_w() {
        assert_eq!(
            next_state(
                Ship {
                    x: 0,
                    y: 0,
                    facing: 0
                },
                "W11"
            ),
            Ship {
                x: -11,
                y: 0,
                facing: 0
            }
        );
    }
}
