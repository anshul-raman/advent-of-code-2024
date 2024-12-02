fn main() {
    let input = include_str!("../../puzzle_inputs/day2.txt");
    println!("Part1 : {}", part1::process(input));
}

mod part1 {

    pub fn process(input: &str) -> i64 {
        input
            .trim()
            .lines()
            .map(parse_line)
            .map(is_safe)
            .map(|f| match f {
                true => 1,
                false => 0,
            })
            .sum()
    }

    fn parse_line(line: &str) -> Vec<i64> {
        line.split_whitespace()
            .map(str::parse::<i64>)
            .map(|f| f.expect("each line must contain only integers"))
            .collect()
    }

    fn is_safe(nums: Vec<i64>) -> bool {

        // j
    }

    #[cfg(test)]
    mod test {
        use crate::part1;

        #[test]
        fn test_part1() {
            let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
            assert_eq!(2, part1::process(input));
        }
    }
}
