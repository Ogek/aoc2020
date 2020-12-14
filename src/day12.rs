#[derive(Debug)]
enum Move {
    North(isize),
    South(isize),
    Est(isize),
    West(isize),
    Forward(isize),
    Right(isize),
    Left(isize),
}

impl Move {
    fn from_line(line: &str) -> Self {
        let val = line[1..].parse::<isize>().unwrap();
        match &line[0..1] {
            "N" => Move::North(val),
            "S" => Move::South(val),
            "E" => Move::Est(val),
            "W" => Move::West(val),
            "L" => Move::Left(val),
            "R" => Move::Right(val),
            "F" => Move::Forward(val),
            _ => unreachable!(),
        }
    }
}

pub fn p1(input: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;

    for m in parse(input) {
        match m {
            Move::North(val) => y += val,
            Move::South(val) => y -= val,
            Move::Est(val) => x += val,
            Move::West(val) => x -= val,
            Move::Forward(val) => {
                x += dx * val;
                y += dy * val;
            }
            Move::Right(val) => {
                for _ in 0..val / 90 {
                    let tmp = dx;
                    dx = dy;
                    dy = -tmp;
                }
            }
            Move::Left(val) => {
                for _ in 0..val / 90 {
                    let tmp = dx;
                    dx = -dy;
                    dy = tmp;
                }
            }
        };
    }

    (x.abs() + y.abs()) as usize
}

pub fn p2(input: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = 1;

    for m in parse(input) {
        match m {
            Move::North(val) => wy += val,
            Move::South(val) => wy -= val,
            Move::Est(val) => wx += val,
            Move::West(val) => wx -= val,
            Move::Forward(val) => {
                x += wx * val;
                y += wy * val;
            }
            Move::Right(val) => {
                for _ in 0..val / 90 {
                    let tmp = wx;
                    wx = wy;
                    wy = -tmp;
                }
            }
            Move::Left(val) => {
                for _ in 0..val / 90 {
                    let tmp = wx;
                    wx = -wy;
                    wy = tmp;
                }
            }
        };
    }

    (x.abs() + y.abs()) as usize
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Move> + 'a {
    input.lines().map(Move::from_line)
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day12.txt")), 845);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day12.txt")), 27016);
}
