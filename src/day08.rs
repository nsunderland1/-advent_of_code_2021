use itertools::Itertools;

fn parse_line(s: &str) -> (Vec<u8>, Vec<u8>) {
    s.split(" | ")
        .map(|side| {
            side.split(" ")
                // map a to 0b1, b to 0b10, c to 0b100...
                .map(|s| s.chars().map(|c| 1 << (c as u8 - b'a')).sum())
                .collect()
        })
        .collect_tuple()
        .unwrap()
}

pub fn run(input: &str) {
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let result1 = {
        input
            .iter()
            .flat_map(|(_, outputs)| outputs.iter())
            .filter(|n| [2, 4, 3, 7].contains(&n.count_ones()))
            .count()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        input
            .iter()
            .map(|(signals, outputs)| {
                let numbers: Vec<u8> = signals
                    .iter()
                    .chain(outputs.iter())
                    .sorted()
                    .dedup()
                    .copied()
                    .collect();

                let find_by_size = |size| -> u8 {
                    *numbers
                        .iter()
                        .find(|number| number.count_ones() == size)
                        .unwrap()
                };

                let one = find_by_size(2);
                let four = find_by_size(4);
                let seven = find_by_size(3);
                let eight = find_by_size(7);

                // 0, 6, 9
                let six_segments: Vec<_> = numbers
                    .iter()
                    .filter(|number| number.count_ones() == 6)
                    .cloned()
                    .collect();

                // 2, 3, 5
                let five_segments: Vec<_> = numbers
                    .iter()
                    .copied()
                    .filter(|number| number.count_ones() == 5)
                    .collect();

                // bottom right segment is the only one shared by 0, 1, 6, 9
                let f = six_segments
                    .iter()
                    .copied()
                    .fold(one, |acc, number| acc & number);

                // top right segment is 1's other segment
                let c = one & !f;

                // six is eight minus the top right segment
                let six = eight & !c;
                assert!(six_segments.contains(&six));

                // three is the only 5-segment number containing the two right segments
                let three = five_segments
                    .iter()
                    .copied()
                    .find(|&number| number & (c | f) == c | f)
                    .unwrap();

                // two is the only other 5-segment number containing the top right segment
                let two = five_segments
                    .iter()
                    .copied()
                    .filter(|&number| number != three)
                    .find(|&number| number & c == c)
                    .unwrap();

                // five is the only remaining 5-segment number
                let five = five_segments
                    .iter()
                    .copied()
                    .find(|&number| number != three && number != two)
                    .unwrap();

                // nine intersects with five along 5 segments, whereas zero only intersects with 5 on 4 segments
                let nine = six_segments
                    .iter()
                    .copied()
                    .filter(|&number| number != six)
                    .find(|&number| (number & five).count_ones() == 5)
                    .unwrap();

                // zero is the only remaining 6-segment number
                let zero = six_segments
                    .iter()
                    .copied()
                    .find(|&number| number != six && number != nine)
                    .unwrap();

                let listing = [zero, one, two, three, four, five, six, seven, eight, nine];
                outputs
                    .iter()
                    .map(|&number| {
                        listing
                            .iter()
                            .enumerate()
                            .find(|(_, &list_item)| list_item == number)
                            .unwrap()
                            .0
                    })
                    .fold(0, |acc, digit| 10 * acc + digit)
            })
            .sum::<usize>()
    };

    println!("Part 2: {}", result2);
}
