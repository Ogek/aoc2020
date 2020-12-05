use std::vec::Vec;

type Forest = Vec<Vec<char>>;

struct WalkIntoForest {
    forest: Forest,
    x: usize,
    y: usize,
    slope: (usize, usize),
}

impl WalkIntoForest {
    fn new(forest: &Forest, slope: (usize, usize)) -> Self {
        WalkIntoForest {
            x: 0,
            y: 0,
            forest: forest.to_vec(),
            slope,
        }
    }
}

impl Iterator for WalkIntoForest {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.forest.len() {
            return None;
        }
        let c = Some(self.forest[self.y][self.x]);
        self.x = (self.x + self.slope.0) % self.forest[0].len();
        self.y += self.slope.1;
        c
    }
}

pub fn p1(input: &str) -> usize {
    count_trees(&parse_forest(input), (3, 1))
}

pub fn p2(input: &str) -> usize {
    let forest = parse_forest(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&slope| count_trees(&forest, slope))
        .product()
}

fn parse_forest(input: &str) -> Forest {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn count_trees(forest: &Forest, slope: (usize, usize)) -> usize {
    WalkIntoForest::new(forest, slope)
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
