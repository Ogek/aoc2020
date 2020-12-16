type BusId = usize;

pub fn p1(input: &str) -> usize {
    let (timestamp, bus_ids) = parse(input);

    let (bus_id, bus_time) = bus_ids
        .map(|bus_id| {
            (
                bus_id,
                bus_id * (timestamp as f32 / bus_id as f32).ceil() as usize,
            )
        })
        .min_by_key(|a| a.1)
        .unwrap();
    bus_id * (bus_time - timestamp)
}

// I've googled a lot for this
pub fn p2(input: &str) -> usize {
    let bus_ids: Vec<(usize, BusId)> = parse2(input).enumerate().collect();

    let mut timestamp = 0;
    let mut inc = bus_ids[0].1;
    for &(i, bus_id) in &bus_ids[1..] {
        if bus_id == 0 {
            continue;
        }
        // https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Computation
        while (timestamp + i) % bus_id != 0 {
            timestamp += inc;
        }
        inc *= bus_id;
    }
    timestamp
}

fn parse<'a>(input: &'a str) -> (usize, impl Iterator<Item = BusId> + 'a) {
    let (timestamp, bus_ids) = input.split_once("\n").unwrap();
    (
        timestamp.parse().unwrap(),
        bus_ids.split(",").filter_map(|b| b.parse::<usize>().ok()),
    )
}

fn parse2<'a>(input: &'a str) -> impl Iterator<Item = BusId> + 'a {
    let (_, bus_ids) = input.split_once("\n").unwrap();
    bus_ids.split(",").map(|b| b.parse::<usize>().unwrap_or(0))
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day13.txt")), 3606);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day13.txt")), 379786358533423);
}
