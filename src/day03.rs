use std::vec::Vec;

type Forest = Vec<Vec<char>>;

pub fn p1(input: &str) -> usize {
    count_trees(&parse_forest(input), (3, 1))
}

pub fn p2(input: &str) -> usize {
    let forest = &parse_forest(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&slope| count_trees(forest, slope))
        .product()
}

fn parse_forest(input: &str) -> Forest {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn walk_into_forest<'a>(
    forest: &'a Forest,
    slope: (usize, usize),
) -> impl Iterator<Item = char> + 'a {
    let mut x = 0;
    let mut y = 0;
    std::iter::from_fn(move || {
        if y >= forest.len() {
            return None;
        }

        let c = Some(forest[y][x]);
        x = (x + slope.0) % forest[0].len();
        y += slope.1;
        c
    })
}

fn count_trees(forest: &Forest, slope: (usize, usize)) -> usize {
    walk_into_forest(forest, slope)
        .filter(|c| match c {
            '#' => return true,
            '.' => return false,
            _ => unreachable!(),
        })
        .count()
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day03.txt")), 187);
}
#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day03.txt")), 4723283400);
}
