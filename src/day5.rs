pub fn react_polymer(polymer: &str) -> usize {
    let search_iter = (97..123)
        .map(|x: u8| {
            let c = x as char;
            format!("{}{}", c , c.to_uppercase())
        })
        .chain((97..123).map(|x: u8| {
            let c = x as char;
            format!("{}{}", c.to_uppercase(), c)
        }));

    let mut parsed: String = polymer.to_string();

    loop {
        let search_iter_clone = search_iter.clone();
        let mut scanned = false; 

        for search in search_iter_clone {
            let temp = parsed.replace(search.as_str(), "");
            if temp.len() < parsed.len() {
                scanned = true;
                parsed = temp;
            }
        }

        if !scanned {
            break;
        }

    }
    parsed.len()
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    println!("{}", b'a' );
    react_polymer(input)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let remove_iter = (97..123).map(|x: u8| {x as char});

    let mut min = input.len();
    for removed in remove_iter {
        let scrubbed = input.replace(removed, "").replace(removed.to_ascii_uppercase(), "");
        if react_polymer(&scrubbed) < min {
            min = react_polymer(&scrubbed);
        };
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&"dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&"dabAcCaCBAcCcaDA"), 4);
    }
}
