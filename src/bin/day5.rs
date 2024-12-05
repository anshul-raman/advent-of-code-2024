fn main() {
    let input = include_str!("../../puzzle_inputs/day5.txt");
    println!("Part1: {}", part1::process(input));
    println!("Part2: {}", part2::process(input));
}

mod part1 {

    use crate::shared::*;

    pub fn process(input: &str) -> i64 {
        let input_data = parse(input);
        let after_rules = make_after_rules(&input_data.pairs);
        let before_rules = make_before_rules(&input_data.pairs);

        input_data
            .pages
            .iter()
            .filter(|p| {
                for x in 0..p.len() {
                    for y in (x + 1)..p.len() {
                        if !is_correct(p[x], p[y], &after_rules, &before_rules) {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|p| {
                // middle value only possible for odd len ?
                assert!(p.len() % 2 != 0);

                // get the middle value
                p[p.len() / 2]
            })
            .sum()
    }

}

mod part2 {

    use std::cmp::Ordering;
    use crate::shared::*;

    pub fn process(input: &str) -> i64 {
        let mut input_data = parse(input);
        let after_rules = make_after_rules(&input_data.pairs);
        let before_rules = make_before_rules(&input_data.pairs);

        input_data
            .pages
            .iter_mut()
            .filter(|p| {
                for x in 0..p.len() {
                    for y in (x + 1)..p.len() {
                        if !is_correct(p[x], p[y], &after_rules, &before_rules) {
                            return true;
                        }
                    }
                }
                false
            })
            .map(|p| {
                p.sort_by(|a, b| {
                    if !is_correct(*a, *b, &after_rules, &before_rules) {
                        return Ordering::Less;
                    }
                    Ordering::Equal
                });
                p[p.len() / 2]
            })
            .sum()
    }


}

mod shared {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug)]
    pub struct Pair {
        pub x: i64,
        pub y: i64,
    }

    pub struct InputData {
        pub pairs: Vec<Pair>,
        pub pages: Vec<Vec<i64>>,
    }

    pub fn parse(input: &str) -> InputData {
        let mut pairs = Vec::new();
        let mut pages = Vec::new();
        let mut reading_pair = true;

        for line in input.lines() {
            if line.is_empty() {
                reading_pair = false;
                continue;
            }

            if reading_pair {
                let parts: Vec<&str> = line.split('|').collect();
                assert_eq!(2, parts.len());

                let x = parts[0].parse::<i64>().expect("Page rules must be integer");
                let y = parts[1].parse::<i64>().expect("Page rules must be integer");

                pairs.push(Pair { x, y })
            } else {
                let page = line
                    .split(',')
                    .map(|n| n.trim().parse::<i64>().expect("expected array of integers"))
                    .collect();
                pages.push(page);
            }
        }

        InputData { pairs, pages }
    }

    pub fn make_after_rules(pairs: &Vec<Pair>) -> HashMap<i64, HashSet<i64>> {
        let mut after_rules: HashMap<i64, HashSet<i64>> = HashMap::new();
        pairs.iter().for_each(|p| {
            after_rules
                .entry(p.x)
                .and_modify(|e| {
                    e.insert(p.y);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(p.y);
                    set
                });
        });
        after_rules
    }

    pub fn make_before_rules(pairs: &Vec<Pair>) -> HashMap<i64, HashSet<i64>> {
        let mut before_rules: HashMap<i64, HashSet<i64>> = HashMap::new();
        pairs.iter().for_each(|p| {
            before_rules
                .entry(p.y)
                .and_modify(|e| {
                    e.insert(p.x);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(p.x);
                    set
                });
        });
        before_rules
    }

    /// Checks if a|b is satisfied
    pub fn is_correct(
        a: i64,
        b: i64,
        after_rules: &HashMap<i64, HashSet<i64>>,
        before_rules: &HashMap<i64, HashSet<i64>>,
    ) -> bool {
        // a should not come after b;
        if after_rules.get(&b).map_or(false, |s| s.contains(&a)) {
            return false;
        }

        // b should not come before a
        if before_rules.get(&a).map_or(false, |s| s.contains(&b)) {
            return false;
        }

        true
    }
}




#[cfg(test)]
mod test {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(143, part1::process(input));
        assert_eq!(123, part2::process(input));
    }
}