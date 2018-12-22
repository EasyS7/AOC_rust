use itertools::iterate;
use itertools::Itertools;
use lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

// Transfroms the input into a Vec of (Needed Step, Step)
#[aoc_generator(day7)]
pub fn input_steps(input: &str) -> Vec<(char, char)> {
    lazy_static::lazy_static! {
        static ref RE: Regex =
        Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin\.").unwrap();
    };

    let collected: Vec<&str> = input.lines().collect();
    collected
        .iter()
        .map(|l| {
            let cap: Vec<char> = RE
                .captures(l)
                .unwrap()
                .iter()
                .map(|c| c.unwrap().as_str().chars().next().unwrap())
                .collect();
            (cap[1], cap[2])
        })
        .collect()
}

//  Vec<(Needed Step, Step)>
//  HashMap<Step ID, Vec(Needed Steps)>
#[aoc(day7, part1)]
pub fn part1(input: &Vec<(char, char)>) -> String {
    let mut steps: BTreeMap<char, Vec<char>> = BTreeMap::new();
    for step in input {
        if !steps.contains_key(&step.1) {
            steps.insert(step.1, vec![]);
        }
        if !steps.contains_key(&step.0) {
            steps.insert(step.0, vec![step.1]);
        } else {
            steps.get_mut(&step.0).unwrap().push(step.1);
        };
    }
    let entries_vec: Vec<char> = steps.keys().map(|&entry| entry).collect();
    let mut final_order_vec: Vec<char> = vec![];
    let mut temp_matches: Vec<char> = vec![];
    let mut ready: bool = false;

    while !ready {
        for (step_id, needed_steps_vec) in steps.iter() {
            if needed_steps_vec.len() == 0 && !final_order_vec.contains(step_id) {
                final_order_vec.push(*step_id);
            } else if needed_steps_vec.len() == final_order_vec.len()
                && needed_steps_vec
                    .iter()
                    .all(|step| final_order_vec.contains(step))
            {
                temp_matches.push(*step_id)
            }
        }
        let sorted_temp: BTreeSet<char> = temp_matches.drain(..).collect();
        final_order_vec.extend(sorted_temp);

        if final_order_vec.len() == entries_vec.len() {
            ready = true
        }
    }
    // while !final_order_vec.len() == entries_vec.len() {
    //     let modified_order = steps.iter().scan((final_order_vec, temp_matches), |(final_order_vec, temp_matches), step| {
    //         for needed_step in step.1 {
    //             match step.1.len() {
    //             0 => temp_matches.push(*step.0),
    //             x => {
    //                 if x == final_order_vec.len() {
    //                     if step.1.clone().iter().all(|x| final_order_vec.contains(&x)) {
    //                         temp_matches.push(*step.0);
    //                     }
    //                 }
    //             }
    //         }
    //         }

    //         temp_matches
    //     final_order_vec
    //         .iter()
    //         .chain(modified_order.drain(..).collect::<Vec<char>>().sort());
    //     });
    println!("{:#?}", &temp_matches);

    String::from_utf8(
        final_order_vec
            .iter()
            .map(|&character| character as u8)
            .collect(),
    )
    .unwrap()
}
