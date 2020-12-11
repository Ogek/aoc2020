use itertools::Itertools;

pub fn p1(input: &str) -> usize {
    find_invalid(&parse(input), 25).unwrap()
}

fn find_invalid(data: &Vec<usize>, preamble_len: usize) -> Option<usize> {
    data.windows(preamble_len + 1).find_map(|w| {
        match w[0..=preamble_len]
            .iter()
            .tuple_combinations()
            .any(|(n, m)| n + m == w[preamble_len])
        {
            true => None,
            false => Some(w[preamble_len]),
        }
    })
}

pub fn p2(input: &str) -> usize {
    let data = parse(input);
    let invalid = find_invalid(&data, 25).unwrap();
    (2..data.len())
        .find_map(|set_len| {
            data.windows(set_len)
                .find_map(|set| match set.iter().sum::<usize>() == invalid {
                    true => Some(set.iter().max().unwrap() + set.iter().min().unwrap()),
                    false => None,
                })
        })
        .unwrap()
}

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

#[test]
fn test_resolve() {
    assert_eq!(
        find_invalid(
            &vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
            6
        )
        .unwrap(),
        127
    );
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day09.txt")), 1309761972);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day09.txt")), 177989832);
}
