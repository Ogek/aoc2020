use serde_scan::scan;
use std::vec::Vec;

struct PassportField {
    key: String,
    val: String,
}

impl PassportField {
    fn from_str(raw_field: &str) -> Self {
        let (key, val) = scan!("{}:{}" <- raw_field).unwrap();
        PassportField { key, val }
    }

    fn ecl_vals() -> [&'static str; 7] {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
    }

    fn is_valid(&self) -> bool {
        match self.key.as_str() {
            "byr" => (1920..=2002).contains(&self.val.parse::<u32>().unwrap()),
            "iyr" => (2010..=2020).contains(&self.val.parse::<u32>().unwrap()),
            "eyr" => (2020..=2030).contains(&self.val.parse::<u32>().unwrap()),
            "hgt" => {
                let last2 = &self.val[(self.val.len() - 2)..];
                match last2 {
                    "cm" => (150..=193)
                        .contains(&self.val[..self.val.len() - 2].parse::<usize>().unwrap()),
                    "in" => (59..=76)
                        .contains(&self.val[..self.val.len() - 2].parse::<usize>().unwrap()),
                    _ => false,
                }
            }
            "hcl" => {
                self.val.len() == 7
                    && self.val.starts_with("#")
                    && self.val[1..].chars().all(|c| c.is_ascii_hexdigit())
            }
            "ecl" => Self::ecl_vals().contains(&self.val.as_str()),
            "pid" => self.val.len() == 9 && self.val.chars().all(|c| c.is_numeric()),
            "cid" => true,
            _ => unreachable!(),
        }
    }
}

struct Passport {
    fields: Vec<PassportField>,
}

impl Passport {
    fn required_field_keys() -> [&'static str; 7] {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    }

    fn has_all_required_fields(&self) -> bool {
        self.fields
            .iter()
            .filter(|f| Self::required_field_keys().contains(&f.key.as_str()))
            .count()
            == Self::required_field_keys().len()
    }

    fn is_valid(&self) -> bool {
        self.has_all_required_fields() && self.fields.iter().all(|f| f.is_valid())
    }
}

pub fn p1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(parse_passport_and_check_required_fields)
        .count()
}

pub fn p2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(parse_passport_and_check_valid_fields)
        .count()
}

fn parse_fields(raw_fields: &str) -> Vec<PassportField> {
    raw_fields.split(" ").map(PassportField::from_str).collect()
}

fn parse_passport_and_check_required_fields(line: &str) -> Option<Passport> {
    let p = Passport {
        fields: parse_fields(&line.replace("\n", " ")),
    };

    if p.has_all_required_fields() {
        return Some(p);
    }

    None
}

fn parse_passport_and_check_valid_fields(line: &str) -> Option<Passport> {
    let p = Passport {
        fields: parse_fields(&line.replace("\n", " ")),
    };

    if p.is_valid() {
        return Some(p);
    }

    None
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day04.txt")), 264);
}
#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day04.txt")), 224);
}
