use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

static MY_BAG: &'static str = "shiny gold";
static NO_DEP: &'static str = "no other bags";

type BagDependencies<'a> = HashSet<Bag<'a>>;

type BagMapEntry<'a> = (&'a str, BagDependencies<'a>);
type BagMap<'a> = HashMap<&'a str, BagDependencies<'a>>;

#[derive(Debug)]
struct Bag<'a> {
    name: &'a str,
    amount: usize,
}

impl<'a> Hash for Bag<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> PartialEq for Bag<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Eq for Bag<'a> {}

impl<'a> Borrow<&'a str> for Bag<'a> {
    fn borrow(&self) -> &&'a str {
        &self.name
    }
}

pub fn p1(input: &str) -> usize {
    let bag_map = build_bag_map(input);

    println!("{:#?}", bag_map);

    bag_map
        .keys()
        .filter(|name| bag_can_contain(&bag_map, name, MY_BAG))
        .count()
}

pub fn p2(input: &str) -> usize {
    let bag_map = build_bag_map(input);
    bag_count(&bag_map, MY_BAG, 1) - 1
}

fn build_bag_map(input: &str) -> BagMap {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> BagMapEntry {
    let kv = line
        .strip_suffix(".")
        .unwrap()
        .split_once(" contain ")
        .unwrap();

    (parse_key(kv.0), parse_dependencies(kv.1))
}

fn parse_key(raw_key: &str) -> &str {
    // raw_dependencies format {bag_name} bags
    raw_key.strip_suffix("bags").unwrap().trim()
}

fn parse_dependencies(raw_dependencies: &str) -> BagDependencies {
    // raw_dependencies format {bag_amount} {bag_name} bag(s), ...
    if raw_dependencies.contains(NO_DEP) {
        return HashSet::new();
    }

    raw_dependencies
        .split(", ")
        .map(|dep| {
            let (number, name) = dep
                .strip_suffix("s")
                .unwrap_or(dep)
                .strip_suffix("bag")
                .unwrap()
                .split_once(" ")
                .unwrap();

            Bag {
                amount: number.parse::<usize>().unwrap(),
                name: name.trim(),
            }
        })
        .collect()
}

fn bag_can_contain(bag_map: &BagMap, key: &str, bag_to_find: &str) -> bool {
    let dependencies = bag_map.get(key).unwrap();
    dependencies.contains(&bag_to_find)
        || dependencies
            .iter()
            .find(|dep| bag_can_contain(bag_map, dep.name, bag_to_find))
            .is_some()
}

fn bag_count(bag_map: &BagMap, bag: &str, count: usize) -> usize {
    let dependencies = bag_map.get(bag).unwrap();
    if dependencies.is_empty() {
        return count;
    }
    dependencies
        .iter()
        .map(|b| bag_count(bag_map, b.name, b.amount * count))
        .sum::<usize>()
        + count
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day07.txt")), 274);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day07.txt")), 158730);
}
