static ROWS: usize = 128;
static COLS: usize = 8;

pub fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|l| row_col_to_ticket_id(ticket_to_row_col_pair(l)))
        .max()
        .unwrap()
}

pub fn p2(input: &str) -> usize {
    let mut seats = input
        .lines()
        .map(|l| row_col_to_ticket_id(ticket_to_row_col_pair(l)))
        .collect::<Vec<usize>>();
    seats.sort();
    seats
        .iter()
        .enumerate()
        .find_map(|(i, &s)| {
            if seats[i + 1] == s + 1 {
                return None;
            }
            Some(s + 1)
        })
        .unwrap()
}

fn ticket_to_row_col_pair(ticket: &str) -> (usize, usize) {
    let mut row = 0..ROWS;
    let mut col = 0..COLS;

    ticket.chars().for_each(|c| {
        match c {
            'F' => row = row.start..(row.end - (row.end - row.start) / 2),
            'B' => row = (row.end - (row.end - row.start) / 2)..row.end,
            'L' => col = col.start..(col.end - (col.end - col.start) / 2),
            'R' => col = (col.end - (col.end - col.start) / 2)..col.end,
            _ => unreachable!(),
        };
    });

    (row.start, col.start)
}

fn row_col_to_ticket_id(row_col: (usize, usize)) -> usize {
    row_col.0 * COLS + row_col.1
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day05.txt")), 933);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day05.txt")), 711);
}
