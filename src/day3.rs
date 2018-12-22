// Day 3: No Matter How You Slice It //

use lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn sheet_builder(rectangles: &[Rectangle]) -> HashMap<(usize, usize), usize> {
    let mut fabric = HashMap::new();
    for rectangle in rectangles {
        for (x, y) in occupancy_checker(rectangle) {
            let position = fabric.entry((x, y)).or_insert(0);
            *position += 1;
        }
    }
    fabric
}

fn occupancy_checker(rectangle: &Rectangle) -> Vec<(usize, usize)> {
    let mut occupancy = Vec::new();
    for x in rectangle.x..(rectangle.w + rectangle.x) {
        for y in rectangle.y..(rectangle.h + rectangle.y) {
            occupancy.push((x, y));
        }
    }
    occupancy
}

fn reg_exp_helper(text: &str) -> Rectangle {
    lazy_static::lazy_static! {
        static ref RE: Regex =
        Regex::new(r"(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let cap = RE.captures(text).unwrap();
    Rectangle {
        id: cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        x: cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        y: cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        w: cap.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        h: cap.get(5).unwrap().as_str().parse::<usize>().unwrap(),
    }
}

#[aoc_generator(day3)]
pub fn input_claims(input: &str) -> Vec<Rectangle> {
    input.lines().map(|l| reg_exp_helper(l)).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Rectangle>) -> usize {
    sheet_builder(input)
        .iter()
        .filter(|(_, &count)| count > 1)
        .count()
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Rectangle>) -> usize {
    let fabric = sheet_builder(input);

    for rectangle in input {
        let occupancy = occupancy_checker(rectangle);
        if occupancy.iter().all(|(x, y)| fabric[&(*x, *y)] == 1) {
            return rectangle.id;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "#1 @ 1,3: 4x4\n\
                             #2 @ 3,1: 4x4\n\
                             #3 @ 5,5: 2x2";

    #[test]
    fn grok_input() {
        let expected = vec![
            Rectangle {
                id: 1,
                x: 1,
                y: 3,
                w: 4,
                h: 4,
            },
            Rectangle {
                id: 2,
                x: 3,
                y: 1,
                w: 4,
                h: 4,
            },
            Rectangle {
                id: 3,
                x: 5,
                y: 5,
                w: 2,
                h: 2,
            },
        ];

        assert_eq!(input_claims(TEST_STR), expected);
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_claims(TEST_STR)), 4);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_claims(TEST_STR)), 3);
    }
}
