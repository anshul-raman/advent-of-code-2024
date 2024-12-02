fn main() {
    let input = include_str!("../../puzzle_inputs/day2.txt");
    println!("Part1 : {}", part1::process(input));
    println!("Part2 : {}", part2::process(input));
}

mod part1 {

    pub fn process(input: &str) -> usize {
        input
            .trim()
            .lines()
            .map(crate::shared::parse_line)
            .filter(|r| is_safe(r))
            .count()
    }

    fn is_safe(nums: &[i64]) -> bool {
        assert!(nums.len() > 1);

        if !(1..=3).contains(&(nums[1] - nums[0]).abs()) {
            return false;
        }

        let is_inc = nums[1] > nums[0];

        for i in 2..nums.len() {
            let next_is_inc = nums[i] > nums[i - 1];
            if next_is_inc != is_inc {
                return false;
            }

            if !(1..=3).contains(&(nums[i] - nums[i - 1]).abs()) {
                return false;
            }
        }

        true
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

mod part2 {
    pub fn process(input: &str) -> i64 {
        input
            .trim()
            .lines()
            .map(crate::shared::parse_line)
            .collect::<Vec<Vec<i64>>>()
            .iter()
            .map(|r| try_is_safe(r))
            .map(|f| match f {
                true => 1,
                false => 0,
            })
            .sum()
    }

    fn try_is_safe(nums: &[i64]) -> bool {
        if is_safe(nums) {
            return true;
        }

        for idx in 0..nums.len() {
            // removing idx
            let mut copied = Vec::with_capacity(nums.len() - 1);
            for (inner_idx, &n) in nums.iter().enumerate() {
                if idx == inner_idx {
                    continue;
                }
                copied.push(n);
            }

            if is_safe(&copied) {
                return true;
            }
        }

        false
    }

    fn is_safe(nums: &[i64]) -> bool {
        assert!(nums.len() > 1);

        if !(1..=3).contains(&(nums[1] - nums[0]).abs()) {
            return false;
        }

        let is_inc = nums[1] > nums[0];

        for i in 2..nums.len() {
            let next_is_inc = nums[i] > nums[i - 1];
            if next_is_inc != is_inc {
                return false;
            }

            if !(1..=3).contains(&(nums[i] - nums[i - 1]).abs()) {
                return false;
            }
        }

        true
    }

    #[cfg(test)]
    mod test {
        use crate::part2;

        #[test]
        fn test_part2() {
            let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
            assert_eq!(4, part2::process(input));
        }
    }
}

mod shared {

    pub fn parse_line(line: &str) -> Vec<i64> {
        line.split_whitespace()
            .map(str::parse::<i64>)
            .map(|f| f.expect("each line must contain only integers"))
            .collect()
    }
}
