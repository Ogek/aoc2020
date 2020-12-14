#[derive(Debug, Eq, PartialEq, Clone)]
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
        let (new_seats_map, changes_count) = move_people(seats_map, apply_rules_1);
        if changes_count == 0 {
            return count_occupied_seats(&new_seats_map);
        }
        seats_map = new_seats_map;
    }
}

pub fn p2(input: &str) -> usize {
    let mut seats_map = parse(input);
    loop {
        let (new_seats_map, changes_count) = move_people(seats_map, apply_rules_2);
        if changes_count == 0 {
            return count_occupied_seats(&new_seats_map);
        }
        seats_map = new_seats_map;
    }
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

fn move_people<F>(seats_map: SeatsMap, apply_rules: F) -> (SeatsMap, usize)
where
    F: Fn(&SeatsMap, &SeatType, Coordinates) -> Option<SeatType>,
{
    let mut count_changes = 0;
    let new = seats_map
        .iter()
        .enumerate()
        .map(|(i, seats_line)| {
            seats_line
                .iter()
                .enumerate()
                .map(
                    |(j, s)| match apply_rules(&seats_map, s, (j as isize, i as isize)) {
                        Some(new) => {
                            count_changes += 1;
                            new
                        }
                        _ => s.clone(),
                    },
                )
                .collect()
        })
        .collect();

    (new, count_changes)
}

fn apply_rules_1(
    seats_map: &SeatsMap,
    seat: &SeatType,
    coordinates: Coordinates,
) -> Option<SeatType> {
    match seat {
        SeatType::Occupied => {
            if iter_adjacents(seats_map, coordinates)
                .filter(|adja| matches!(adja, SeatType::Occupied))
                .count()
                >= 4
            {
                Some(SeatType::Empty)
            } else {
                None
            }
        }
        SeatType::Empty => {
            if iter_adjacents(seats_map, coordinates)
                .filter(|adja| matches!(adja, SeatType::Occupied))
                .count()
                == 0
            {
                Some(SeatType::Occupied)
            } else {
                None
            }
        }
        SeatType::Floor => None,
    }
}

fn apply_rules_2(
    seats_map: &SeatsMap,
    seat: &SeatType,
    coordinates: Coordinates,
) -> Option<SeatType> {
    match seat {
        SeatType::Occupied => {
            if iter_visible(seats_map, coordinates)
                .filter(|adja| matches!(adja, SeatType::Occupied))
                .count()
                >= 5
            {
                Some(SeatType::Empty)
            } else {
                None
            }
        }
        SeatType::Empty => {
            if iter_visible(seats_map, coordinates)
                .filter(|adja| matches!(adja, SeatType::Occupied))
                .count()
                == 0
            {
                Some(SeatType::Occupied)
            } else {
                None
            }
        }
        SeatType::Floor => None,
    }
}

static DIRECTIONS: [Coordinates; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

fn get_seat(seats_map: &SeatsMap, coordinates: Coordinates) -> Option<&SeatType> {
    seats_map
        .get(coordinates.1 as usize)
        .and_then(|line| line.get(coordinates.0 as usize))
}

fn iter_adjacents<'a>(
    seats_map: &'a SeatsMap,
    coordinates: Coordinates,
) -> impl Iterator<Item = &'a SeatType> + 'a {
    DIRECTIONS
        .iter()
        .filter_map(move |(x, y)| get_seat(seats_map, (coordinates.0 + x, coordinates.1 + y)))
}

fn iter_visible<'a>(
    seats_map: &'a SeatsMap,
    coordinates: Coordinates,
) -> impl Iterator<Item = &'a SeatType> + 'a {
    fn find_visible(
        seats_map: &SeatsMap,
        direction: Coordinates,
        target: Coordinates,
    ) -> Option<&SeatType> {
        match get_seat(&seats_map, target) {
            Some(seat) => match seat {
                SeatType::Floor => find_visible(
                    seats_map,
                    direction,
                    (target.0 + direction.0, target.1 + direction.1),
                ),
                _ => Some(seat),
            },
            None => None,
        }
    };

    DIRECTIONS.iter().filter_map(move |(x, y)| {
        find_visible(seats_map, (*x, *y), (coordinates.0 + x, coordinates.1 + y))
    })
}

#[test]
fn test_adjacents() {
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
    assert_eq!(
        iter_adjacents(&parsed, (0, 0)).collect::<Vec<&SeatType>>(),
        vec![&SeatType::Occupied, &SeatType::Empty, &SeatType::Floor]
    );
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
fn test_visible() {
    let data = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";
    let parsed = parse(data);
    assert_eq!(
        iter_visible(&parsed, (0, 0)).collect::<Vec<&SeatType>>(),
        vec![&SeatType::Occupied, &SeatType::Occupied]
    );
    assert_eq!(
        iter_visible(&parsed, (3, 4)).collect::<Vec<&SeatType>>(),
        vec![&SeatType::Occupied; 8]
    );
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day11.txt")), 2275);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day11.txt")), 2121);
}
