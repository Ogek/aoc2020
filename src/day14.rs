use itertools::{EitherOrBoth::*, Itertools};
use serde_scan::scan;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction<'a> {
    Mask(&'a str),
    Malloc(usize, usize),
}

type BitMask = usize;
type Memory = HashMap<usize, usize>;

#[derive(Debug)]
struct DecoderChip<'a> {
    and_mask: BitMask,
    or_mask: BitMask,
    raw_mask: &'a str,
    memory: Memory,
}

impl<'a> DecoderChip<'a> {
    fn new() -> Self {
        DecoderChip {
            and_mask: 0,
            or_mask: 0,
            raw_mask: "",
            memory: Memory::new(),
        }
    }
}

fn calc_addr_mask(addr: usize, mask: &str) -> String {
    let bits = format!("{:b}", addr);

    mask.chars()
        .rev()
        .zip_longest(bits.chars().rev())
        .map(|pair| match pair {
            Left(mask_val) => mask_val,
            Both(mask_val, bit_val) => match mask_val {
                '0' => bit_val,
                '1' | 'X' => mask_val,
                _ => unreachable!(),
            },
            Right(_) => unreachable!(),
        })
        .collect::<String>()
}

fn resolve_floating_bits(mask: String) -> Vec<String> {
    let a = mask.replacen("X", "1", 1);
    let b = mask.replacen("X", "0", 1);

    if a.contains("X") {
        return vec![resolve_floating_bits(a), resolve_floating_bits(b)]
            .into_iter()
            .flatten()
            .collect();
    }
    vec![a, b]
}

fn run_instruction_v2<'a>(chip: DecoderChip<'a>, instruction: Instruction<'a>) -> DecoderChip<'a> {
    match instruction {
        Instruction::Mask(raw_mask) => DecoderChip { raw_mask, ..chip },
        Instruction::Malloc(addr, val) => {
            let masked_addr = calc_addr_mask(addr, chip.raw_mask);
            let mut memory = chip.memory;

            for masked_addr in resolve_floating_bits(masked_addr) {
                let masked_addr = usize::from_str_radix(masked_addr.as_str(), 2).unwrap();
                memory.insert(masked_addr, val);
            }

            DecoderChip { memory, ..chip }
        }
    }
}

fn run_instruction_v1<'a>(chip: DecoderChip<'a>, instruction: Instruction<'a>) -> DecoderChip<'a> {
    match instruction {
        Instruction::Mask(mask) => {
            let and_mask = usize::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
            let or_mask = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
            DecoderChip {
                and_mask,
                or_mask,
                ..chip
            }
        }
        Instruction::Malloc(addr, val) => {
            let mut memory = chip.memory;
            memory.insert(addr, val | chip.or_mask & chip.and_mask);
            DecoderChip { memory, ..chip }
        }
    }
}

pub fn p1(input: &str) -> usize {
    let decoder_chip = parse(input).fold(DecoderChip::new(), run_instruction_v1);

    decoder_chip.memory.values().sum()
}

pub fn p2(input: &str) -> usize {
    let decoder_chip = parse(input).fold(DecoderChip::new(), run_instruction_v2);

    decoder_chip.memory.values().sum()
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    input.lines().map(parse_instruction)
}

fn parse_instruction(line: &str) -> Instruction {
    if line.starts_with("mask") {
        let bits = scan!("mask = {}" <- line).unwrap();
        return Instruction::Mask(bits);
    } else {
        let (addr, val) = scan!("mem[{}] = {}" <- line).unwrap();
        return Instruction::Malloc(addr, val);
    }
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day14.txt")), 12610010960049);
}

#[test]
fn test_resolve_floating_bits() {
    let expected = vec!["11011", "11010", "10011", "10010"];
    assert_eq!(expected, resolve_floating_bits(String::from("1X01X")))
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day14.txt")), 3608464522781);
}
