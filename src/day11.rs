#[derive(Debug, Eq, PartialEq)]
enum SeatType {
    Empty,
    Occupied,
    Floor,
}

impl SeatType {
    fn from_char(c: char) -> Self {
        match c {
            '#' => SeatType::Occupied,
            'L' => SeatType::Empty,
            '.' => SeatType::Floor,
            _ => unreachable!(),
        }
    }
}

type SeatsMap = Vec<Vec<SeatType>>;
type Coordinates = (isize, isize);

pub fn p1(input: &str) -> usize {
    let mut seats_map = parse(input);
    loop {
        let (new_seats_map, changes_count) = apply_rules(seats_map);
        if changes_count == 0 {
            return count_occupied_seats(&new_seats_map);
        }
        seats_map = new_seats_map;
    }
}

pub fn p2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> SeatsMap {
    input
        .lines()
        .map(|line| line.chars().map(SeatType::from_char).collect())
        .collect()
}

fn count_occupied_seats(seats_map: &SeatsMap) -> usize {
    seats_map
        .iter()
        .flatten()
        .filter(|s| matches!(s, SeatType::Occupied))
        .count()
}

fn apply_rules(seats_map: SeatsMap) -> (SeatsMap, usize) {
    let mut count_changes = 0;
    let new = seats_map
        .iter()
        .enumerate()
        .map(|(i, seats_line)| {
            seats_line
                .iter()
                .enumerate()
                .map(|(j, s)| match s {
                    SeatType::Occupied => {
                        match apply_occupied_seat_rule(&seats_map, (j as isize, i as isize)) {
                            Some(new) => {
                                count_changes += 1;
                                new
                            }
                            _ => SeatType::Occupied,
                        }
                    }
                    SeatType::Empty => {
                        match apply_empty_seat_rule(&seats_map, (j as isize, i as isize)) {
                            Some(new) => {
                                count_changes += 1;
                                new
                            }
                            _ => SeatType::Empty,
                        }
                    }
                    SeatType::Floor => SeatType::Floor,
                })
                .collect()
        })
        .collect();
    (new, count_changes)
}

fn apply_occupied_seat_rule(seats_map: &SeatsMap, coordinate: Coordinates) -> Option<SeatType> {
    if iter_adjacents(seats_map, coordinate)
        .filter(|adja| matches!(adja, SeatType::Occupied))
        .count()
        >= 4
    {
        Some(SeatType::Empty)
    } else {
        None
    }
}

fn apply_empty_seat_rule(seats_map: &SeatsMap, coordinate: Coordinates) -> Option<SeatType> {
    if iter_adjacents(seats_map, coordinate)
        .filter(|adja| matches!(adja, SeatType::Occupied))
        .count()
        == 0
    {
        Some(SeatType::Occupied)
    } else {
        None
    }
}

static ADJACENTS_DEF: [Coordinates; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

fn iter_adjacents<'a>(
    seats_map: &'a SeatsMap,
    coordinates: Coordinates,
) -> impl Iterator<Item = &'a SeatType> + 'a {
    ADJACENTS_DEF.iter().filter_map(move |(x, y)| {
        seats_map
            .get((coordinates.1 + y) as usize)
            .and_then(|line| line.get((coordinates.0 + x) as usize))
    })
}

#[test]
fn test_adjacents_border() {
    let data = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";
    let parsed = parse(data);
    // assert_eq!(
    //     iter_adjacents(&parsed, (0, 0)).collect::<Vec<&SeatType>>(),
    //     vec![&SeatType::Occupied, &SeatType::Empty, &SeatType::Floor]
    // );
    assert_eq!(
        iter_adjacents(&parsed, (5, 5)).collect::<Vec<&SeatType>>(),
        vec![
            &SeatType::Floor,
            &SeatType::Occupied,
            &SeatType::Occupied,
            &SeatType::Floor,
            &SeatType::Floor,
            &SeatType::Occupied,
            &SeatType::Empty,
            &SeatType::Empty
        ]
    )
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day11.txt")), 2275);
}

#[test]
fn test_p2() {
    //assert_eq!(p2(include_str!("../inputs/day10.txt")), 1157018619904);
}
