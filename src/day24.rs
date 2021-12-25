use std::ops::{Index, IndexMut};

#[allow(unused)]
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::{pair, preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Register {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

enum Expression {
    Register(Register),
    Immediate(i64),
}

impl Expression {
    fn value(&self, state: &State) -> i64 {
        match self {
            Self::Register(r) => state[*r],
            Self::Immediate(i) => *i,
        }
    }
}

enum Instruction {
    Inp(Register),
    Add(Register, Expression),
    Mul(Register, Expression),
    Div(Register, Expression),
    Mod(Register, Expression),
    Eql(Register, Expression),
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    alt((
        value(Register::W, tag("w")),
        value(Register::X, tag("x")),
        value(Register::Y, tag("y")),
        value(Register::Z, tag("z")),
    ))(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(parse_register, Expression::Register),
        map(nom::character::complete::i64, Expression::Immediate),
    ))(input)
}

fn parse_bin_instruction<'a>(
    name: &'static str,
    mut constructor: impl FnMut(Register, Expression) -> Instruction,
) -> impl FnMut(&'a str) -> IResult<&'a str, Instruction> {
    map(
        preceded(
            pair(tag(name), tag(" ")),
            separated_pair(parse_register, tag(" "), parse_expression),
        ),
        move |(l, r)| constructor(l, r),
    )
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(preceded(tag("inp "), parse_register), Instruction::Inp),
        parse_bin_instruction("add", Instruction::Add),
        parse_bin_instruction("mul", Instruction::Mul),
        parse_bin_instruction("div", Instruction::Div),
        parse_bin_instruction("mod", Instruction::Mod),
        parse_bin_instruction("eql", Instruction::Eql),
    ))(input)
}

#[derive(Default)]
struct State {
    registers: [i64; 4],
}

impl Index<Register> for State {
    type Output = i64;

    fn index(&self, index: Register) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<Register> for State {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

fn step(state: &mut State, input: &mut impl Iterator<Item = i64>, instruction: &Instruction) {
    match instruction {
        Instruction::Inp(r) => state[*r] = input.next().unwrap(),
        Instruction::Add(a, b) => state[*a] = state[*a] + b.value(state),
        Instruction::Mul(a, b) => state[*a] = state[*a] * b.value(state),
        Instruction::Div(a, b) => state[*a] = state[*a] / b.value(state),
        Instruction::Mod(a, b) => state[*a] = state[*a] % b.value(state),
        Instruction::Eql(a, b) => state[*a] = (state[*a] == b.value(state)) as i64,
    }
}

fn execute(mut input: impl Iterator<Item = i64>, program: &Vec<Instruction>) -> State {
    let mut state = State::default();
    for instruction in program.iter() {
        step(&mut state, &mut input, instruction);
    }
    state
}

pub fn run(input: &str) {
    let program: Vec<_> = input
        .lines()
        .map(|line| parse_instruction(&line).unwrap().1)
        .collect();

    let mut buf = String::with_capacity(11);
    let mut number = [1; 14];
    loop {
        std::io::stdin().read_line(&mut buf).unwrap();
        let (pos, val) = buf
            .trim()
            .split(' ')
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap();
        let val = val as i64;
        number[pos - 1] = val;

        let input_stream = number.iter().copied();
        let final_state = execute(input_stream, &program);
        println!(
            "Number: {}",
            number.iter().map(i64::to_string).collect::<String>()
        );
        println!("Z: {}", final_state[Register::Z]);
        println!();

        buf.clear();
    }

    // let result1 = {
    //     let start: i64 = 99_999_999_999_999;
    //     let end: i64 = 11_111_111_111_111;
    //     (end..=start)
    //         .into_par_iter()
    //         .filter(|x| !x.to_string().contains('0'))
    //         .find_any(|x| {
    //             let xs = x.to_string();
    //             let input_stream = xs.bytes().map(|c| (c - b'0') as i64);

    //             let final_state = execute(input_stream, &program);
    //             final_state[Register::Z] == 0
    //         })
    //         .unwrap()
    // };

    // println!("Part 1: {}", result1);

    // let result2 = {
    //     // Part 2
    //     0
    // };

    // println!("Part 2: {}", result2);
}
