use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let (mut twos, mut threes) = (0, 0);

    for line in input.lines() {
        let mut frequencies_map = HashMap::new();

        for c in line.chars() {
            let freq = frequencies_map.entry(c).or_insert(0);
            *freq += 1
        }

        if frequencies_map.iter().any(|(_, &count)| count == 2) {
            twos += 1;
        }

        if frequencies_map.iter().any(|(_, &count)| count == 3) {
            threes += 1;
        }
    }

    twos * threes
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    for (id1, val1) in input.lines().enumerate() {
        for val2 in input.lines().skip(id1) {
            let num_diff_chars = val1
                .chars()
                .zip(val2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();

            if num_diff_chars == 1 {
                return val1
                    .chars()
                    .zip(val2.chars())
                    .filter(|(c1, c2)| c1 == c2)
                    .map(|(c, _)| c)
                    .collect();
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            part1(&"abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"),
            12
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            part2(&"abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"),
            "fgij"
        );
    }
}
