#[allow(unused)]
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
enum Operator {
    Type0(Vec<Packet>),
    Type1(Vec<Packet>),
}

#[derive(Debug)]
enum PacketType {
    Literal(u64), // big enough?
    Operator(Operator),
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
                leading.into_iter().fold(0, |acc, i| (acc << 4) + i) << 4 + trailing,
            )
        },
    )(input)
}

fn parse_operator<'a>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), PacketType> {
    map(
        alt((
            pair(
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
            pair(
                tag(1, 1usize),
                length_count(map(take(11usize), |n: usize| n), parse_packet),
            ),
        )),
        move |(length_type_id, operands)| {
            PacketType::Operator(match length_type_id {
                0 => Operator::Type0(operands),
                1 => Operator::Type1(operands),
                _ => unreachable!(),
            })
        },
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
            PacketType::Operator(Operator::Type0(subs) | Operator::Type1(subs)) => {
                subs.iter().map(sum_versions).sum()
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

    let result1 = { sum_versions(&packet) };

    println!("Part 1: {}", result1);

    let result2 = {
        // Part 2
        0
    };

    println!("Part 2: {}", result2);
}
