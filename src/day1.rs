use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_frequency(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[isize]) -> isize {
    let result = input.iter().sum();
    result
}

#[aoc(day1, part2)]
pub fn part2(input: &[isize]) -> isize {
    let mut freq = 0;
    let mut frequencies = HashSet::new();

    frequencies.insert(freq);
    for num in input.iter().cycle() {
        freq += num;
        if frequencies.contains(&freq) {
            return freq;
        }
        frequencies.insert(freq);
    }

    unreachable!()
}
