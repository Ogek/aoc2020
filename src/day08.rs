use std::collections::HashSet;

#[derive(Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug, Eq, PartialEq)]
enum ProgramError {
    InfiniteLoop,
}

type Program = Vec<Instruction>;
type ProgramProcessing<'a> = Result<&'a Instruction, &'a ProgramError>;

pub fn p1(input: &str) -> isize {
    run(&parse(input)).0
}

pub fn p2(input: &str) -> isize {
    let mut program = parse(input);

    fn swap_instructions(program: &Program, index: usize) -> Instruction {
        match program[index] {
            Instruction::Nop(val) => Instruction::Jmp(val),
            Instruction::Jmp(val) => Instruction::Nop(val),
            Instruction::Acc(val) => Instruction::Acc(val),
        }
    }

    program
        .clone()
        .iter()
        .enumerate()
        .filter(|(_, instruction)| match instruction {
            Instruction::Jmp(_) | Instruction::Nop(_) => true,
            _ => false,
        })
        .find_map(|(i, _)| {
            program[i] = swap_instructions(&program, i);
            let (res, bugged) = run(&program);
            program[i] = swap_instructions(&program, i);

            match bugged {
                true => None,
                false => Some(res),
            }
        })
        .unwrap()
}

fn parse(input: &str) -> Program {
    input.lines().map(parse_instruction).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let (instruction_type, number) = line.split_once(" ").unwrap();

    match instruction_type {
        "acc" => Instruction::Acc(number.parse::<isize>().unwrap()),
        "jmp" => Instruction::Jmp(number.parse::<isize>().unwrap()),
        "nop" => Instruction::Nop(number.parse::<isize>().unwrap()),
        _ => unreachable!(),
    }
}

fn run(program: &Program) -> (isize, bool) {
    let mut bugged = false;

    let res = interpreter(program)
        .take_while(|instruction| {
            if Result::is_ok(instruction) {
                true
            } else {
                bugged = true;
                false
            }
        })
        .filter_map(|instruction| match instruction.ok().unwrap() {
            Instruction::Acc(val) => Some(val),
            _ => None,
        })
        .sum();

    (res, bugged)
}

fn interpreter<'a>(program: &'a Program) -> impl Iterator<Item = ProgramProcessing> + 'a {
    let mut i: isize = 0;
    let mut executed_instruction_indexes: HashSet<usize> = HashSet::new();

    std::iter::from_fn(move || {
        if i as usize >= program.len() {
            return None;
        }
        if executed_instruction_indexes.contains(&(i as usize)) {
            return Some(Err(&ProgramError::InfiniteLoop));
        }
        executed_instruction_indexes.insert(i as usize);

        let instruction = &program[i as usize];
        match instruction {
            Instruction::Jmp(val) => i += val,
            _ => i += 1,
        };

        Some(Ok(instruction))
    })
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day08.txt")), 1563);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day08.txt")), 767);
}
