use itertools::Itertools;
use nom::{
    bits::complete::tag,
    bits::complete::take,
    branch::alt,
    combinator::{flat_map, map},
    multi::{length_count, many0},
    sequence::{pair, preceded},
    IResult, InputLength,
};

#[derive(Debug)]
enum PacketType {
    Literal(u64), // big enough?
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    kind: PacketType,
}

fn parse_leading_block(input: (&[u8], usize)) -> IResult<(&[u8], usize), u64> {
    preceded(tag(1, 1usize), take(4usize))(input)
}

fn parse_trailing_block(input: (&[u8], usize)) -> IResult<(&[u8], usize), u64> {
    preceded(tag(0, 1usize), take(4usize))(input)
}

fn parse_literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), PacketType> {
    map(
        pair(many0(parse_leading_block), parse_trailing_block),
        |(leading, trailing)| {
            PacketType::Literal(
                (leading.into_iter().fold(0, |acc, i| (acc << 4) + i) << 4) + trailing,
            )
        },
    )(input)
}

fn parse_operator<'a>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), PacketType> {
    map(
        alt((
            preceded(
                tag(0, 1usize),
                flat_map(take(15usize), |n: usize| {
                    move |mut input: (&'a [u8], usize)| {
                        let mut packets = Vec::new();
                        let input_len = input.input_len();

                        while input_len - input.input_len() < n {
                            let (remaining_input, next_packet) = parse_packet(input)?;
                            packets.push(next_packet);
                            input = remaining_input;
                        }

                        // TODO: return an error here. nom error handling is a pain
                        assert_eq!(input_len - input.input_len(), n);

                        Ok((input, packets))
                    }
                }),
            ),
            preceded(
                tag(1, 1usize),
                length_count(map(take(11usize), |n: usize| n), parse_packet),
            ),
        )),
        |operands| PacketType::Operator(operands),
    )(input)
}

fn parse_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    map(
        pair(
            take(3usize),
            flat_map(take(3usize), |type_id| {
                map(
                    match type_id {
                        4 => parse_literal,
                        _ => parse_operator,
                    },
                    move |kind| (type_id, kind),
                )
            }),
        ),
        |(version, (type_id, kind))| Packet {
            version,
            type_id,
            kind,
        },
    )(input)
}

fn sum_versions(packet: &Packet) -> u64 {
    packet.version as u64
        + match &packet.kind {
            PacketType::Literal(_) => 0,
            PacketType::Operator(operands) => operands.iter().map(sum_versions).sum(),
        }
}

fn evaluate(packet: &Packet) -> u64 {
    match packet {
        Packet {
            kind: PacketType::Literal(n),
            ..
        } => *n,
        Packet {
            kind: PacketType::Operator(operands),
            type_id,
            ..
        } => {
            let operands = operands.iter().map(evaluate);
            match type_id {
                0 => operands.sum(),
                1 => operands.product(),
                2 => operands.min().unwrap(),
                3 => operands.max().unwrap(),
                5 => {
                    let (a, b) = operands.collect_tuple().unwrap();
                    (a > b) as u64
                }
                6 => {
                    let (a, b) = operands.collect_tuple().unwrap();
                    (a < b) as u64
                }
                7 => {
                    let (a, b) = operands.collect_tuple().unwrap();
                    (a == b) as u64
                }
                _ => unreachable!(),
            }
        }
    }
}

pub fn run(input: &str) {
    let input: Vec<_> = input
        .chars()
        .map(|c| match c as u8 {
            c @ b'0'..=b'9' => c - b'0',
            c @ b'A'..=b'F' => c - b'A' + 0xA,
            _ => unreachable!(),
        })
        .tuples()
        .map(|(hi, lo)| (hi << 4) + lo)
        .collect();

    let packet = parse_packet((&input, 0)).unwrap().1;

    let result1 = sum_versions(&packet);

    println!("Part 1: {}", result1);

    let result2 = evaluate(&packet);

    println!("Part 2: {}", result2);
}
