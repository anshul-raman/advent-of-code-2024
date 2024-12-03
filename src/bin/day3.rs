fn main() {
    let input = include_str!("../../puzzle_inputs/day3.txt");
    println!("Part1: {}", part1::process(input));
    println!("Part2: {}", part2::process(input));
}

mod part1 {

    use regex::Regex;

    pub fn process(input: &str) -> i64 {
        let matches = get_matches(input);
        matches.iter().map(|(a, b)| a * b).sum()
    }

    fn get_matches(input: &str) -> Vec<(i64, i64)> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        re.captures_iter(input)
            .map(|cap| {
                let a = cap[1].parse::<i64>().unwrap();
                let b = cap[2].parse::<i64>().unwrap();
                (a, b)
            })
            .collect()
    }

    #[cfg(test)]
    mod test {
        use crate::part1;

        #[test]
        fn test_part1() {
            let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

            assert_eq!(161, part1::process(input))
        }
    }
}

mod part2 {
    use regex::Regex;

    enum Instructions {
        Do,
        Dont,
        Mul(i64, i64),
    }

    pub fn process(input: &str) -> i64 {
        let matches = get_matches(input);
        let mut to_inclued = true;
        let mut ans = 0i64;
        for ins in matches.iter() {
            match *ins {
                Instructions::Do => to_inclued = true,
                Instructions::Dont => to_inclued = false,
                Instructions::Mul(a, b) => {
                    if to_inclued {
                        ans += a * b;
                    }
                }
            }
        }

        ans
    }

    fn get_matches(input: &str) -> Vec<Instructions> {
        let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let do_re = Regex::new(r"do\(\)").unwrap();
        let dont_re = Regex::new(r"don't\(\)").unwrap();

        let mut all_matches = Vec::new();

        for cap in mul_re.captures_iter(input) {
            let pos = cap.get(0).unwrap().start();
            let a = cap[1].parse::<i64>().unwrap();
            let b = cap[2].parse::<i64>().unwrap();
            all_matches.push((pos, Instructions::Mul(a, b)));
        }

        for m in do_re.find_iter(input) {
            all_matches.push((m.start(), Instructions::Do));
        }

        for m in dont_re.find_iter(input) {
            all_matches.push((m.start(), Instructions::Dont));
        }

        // Sort by position to maintain order of instructions
        all_matches.sort_by_key(|&(pos, _)| pos);

        // Return just the instructions in order
        all_matches.into_iter().map(|(_, ins)| ins).collect()
    }

    #[cfg(test)]
    mod test {
        use crate::part2;

        #[test]
        fn test_part2() {
            let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
            assert_eq!(48, part2::process(input))
        }
    }
}
