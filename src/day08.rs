use std::collections::HashSet;

use itertools::Itertools;

fn parse_line(s: &str) -> (Vec<String>, Vec<String>) {
    s.split(" | ")
        .map(|side| side.split(" ").map(String::from).collect())
        .collect_tuple()
        .unwrap()
}

pub fn run(input: &str) {
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let result1 = {
        input
            .iter()
            .flat_map(|(_, outputs)| outputs.iter())
            .filter(|s| [2, 4, 3, 7].contains(&s.len()))
            .count()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        input
            .iter()
            .map(|(signals, outputs)| {
                let numbers: Vec<HashSet<char>> = signals
                    .iter()
                    .chain(outputs.iter())
                    .map(|word| word.chars().sorted().collect::<String>())
                    .sorted()
                    .dedup()
                    .map(|word| word.chars().collect())
                    .collect();

                let find_by_size = |size| -> &HashSet<_> {
                    numbers.iter().find(|number| number.len() == size).unwrap()
                };

                let one = find_by_size(2);
                let four = find_by_size(4);
                let seven = find_by_size(3);
                let eight = find_by_size(7);

                // 0, 6, 9
                let six_segments: Vec<_> = numbers
                    .iter()
                    .filter(|number| number.len() == 6)
                    .cloned()
                    .collect();

                // 2, 3, 5
                let five_segments: Vec<_> = numbers
                    .iter()
                    .filter(|number| number.len() == 5)
                    .cloned()
                    .collect();

                // top segment is the only one not shared by 1, 7
                let a = seven.difference(&one).next().unwrap();

                // bottom right segment is the only one shared by 0, 1, 6, 9
                let f = six_segments
                    .iter()
                    .fold(one.clone(), |acc, number| {
                        acc.intersection(&number).cloned().collect()
                    })
                    .into_iter()
                    .next()
                    .unwrap();

                // top right segment is 1's other segment
                let c = one.iter().find(|&&segment| segment != f).unwrap();

                let six: HashSet<_> = eight.difference(&HashSet::from([*c])).cloned().collect();
                assert!(six_segments.contains(&six));

                let three: HashSet<_> = five_segments
                    .iter()
                    .cloned()
                    .find(|number| number.contains(&c) && number.contains(&f))
                    .unwrap();

                let two: HashSet<_> = five_segments
                    .iter()
                    .cloned()
                    .filter(|number| *number != three)
                    .find(|number| number.contains(&c))
                    .unwrap();

                let five: HashSet<_> = five_segments
                    .iter()
                    .cloned()
                    .filter(|number| *number != three && *number != two)
                    .next()
                    .unwrap();

                let nine: HashSet<_> = six_segments
                    .iter()
                    .cloned()
                    .filter(|number| *number != six)
                    .find(|number| number.intersection(&five).count() == 5)
                    .unwrap();

                let zero: HashSet<_> = six_segments
                    .iter()
                    .cloned()
                    .filter(|number| *number != six && *number != nine)
                    .next()
                    .unwrap();

                let listing = [
                    zero,
                    one.clone(),
                    two,
                    three,
                    four.clone(),
                    five,
                    six,
                    seven.clone(),
                    eight.clone(),
                    nine,
                ];
                outputs
                    .iter()
                    .map(|word| word.chars().collect::<HashSet<_>>())
                    .map(|number| {
                        listing
                            .iter()
                            .enumerate()
                            .find(|(_, list_item)| **list_item == number)
                            .unwrap()
                            .0
                    })
                    .fold(0, |acc, digit| 10 * acc + digit)
            })
            .sum::<usize>()
    };

    println!("Part 2: {}", result2);
}
