fn main() {
    let input = include_str!("../../puzzle_inputs/day1.txt");
    println!("Part1 : {}", part1::process(input));
    println!("Part2 : {}", part2::process(input));
}

fn parse_line(line: &str) -> (i64, i64) {
    let mut nums = line.split_whitespace().map(str::parse::<i64>);
    match (nums.next(), nums.next()) {
        (Some(Ok(a)), Some(Ok(b))) => (a, b),
        _ => panic!("Each line must contain exactly two numbers"),
    }
}

mod part1 {
    use crate::parse_line;

    pub fn process(input: &str) -> i64 {
        let numbers: Vec<(i64, i64)> = input.trim().lines().map(parse_line).collect();
        let (mut lefts, mut rights): (Vec<_>, Vec<_>) = numbers.into_iter().unzip();

        lefts.sort_unstable();
        rights.sort_unstable();

        lefts
            .iter()
            .zip(&rights)
            .map(|(&l, &r)| (r - l).abs())
            .sum()
    }

    #[cfg(test)]
    mod test {
        use crate::part1::process;

        #[test]
        fn part1_test() {
            let input = "3   4
4   3
2   5
1   3
3   9
3   3";
            assert_eq!(11, process(input))
        }
    }
}

mod part2 {
    use std::collections::HashMap;

    use crate::parse_line;

    pub fn process(input: &str) -> i64 {
        let numbers: Vec<(i64, i64)> = input.lines().map(parse_line).collect();
        let (lefts, rights): (Vec<_>, Vec<_>) = numbers.into_iter().unzip();

        let right_counts: HashMap<_, _> =
            rights.into_iter().fold(HashMap::new(), |mut acc, num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            });

        lefts
            .into_iter()
            .map(|num| num * right_counts.get(&num).unwrap_or(&0))
            .sum()
    }

    #[cfg(test)]
    mod test {
        use crate::part2::process;

        #[test]
        fn part1_test() {
            let input = "3   4
4   3
2   5
1   3
3   9
3   3";
            assert_eq!(31, process(input))
        }
    }
}
