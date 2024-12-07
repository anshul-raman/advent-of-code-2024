fn main() {
    let input = include_str!("../../puzzle_inputs/day7.txt");
    println!("Part1: {}", part1::process(input));
    println!("Part2: {}", part2::process(input));
}

mod part1 {
    pub fn process(input: &str) -> i64 {
        input
            .trim()
            .lines()
            .map(|l| {
                let l: Vec<&str> = l.split(':').collect();
                let a = l
                    .first()
                    .expect("invalid input")
                    .parse::<i64>()
                    .expect("invalid input");

                let b: Vec<i64> = l
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse().expect("must be integer array"))
                    .collect();
                (a, b)
            })
            .filter(|(a, b)| is_valid(*a, b))
            .map(|(a, _)| a)
            .sum()
    }

    fn is_valid(a: i64, b: &[i64]) -> bool {
        let n = b.len() - 1;
        let final_mask = (1i64 << n) - 1;
        for mask in 0..=final_mask {
            let mut k = b[0];

            for (idx, &el) in b.iter().skip(1).enumerate() {
                if mask & (1 << idx) != 0 {
                    k += el;
                } else {
                    k *= el;
                }
            }

            if k == a {
                return true;
            }
        }

        false
    }
}

mod part2 {

    pub fn process(input: &str) -> i64 {
        input
            .trim()
            .lines()
            .map(|l| {
                let l: Vec<&str> = l.split(':').collect();
                let a = l
                    .first()
                    .expect("invalid input")
                    .parse::<i64>()
                    .expect("invalid input");

                let b: Vec<i64> = l
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse().expect("must be integer array"))
                    .collect();
                (a, b)
            })
            .filter(|(a, b)| is_valid(*a, b))
            .map(|(a, _)| a)
            .sum()
    }

    fn is_valid(a: i64, b: &[i64]) -> bool {
        match b {
            [n] => a == *n,
            [n1, n2] => {
                a == (*n1 + *n2)
                    || a == (*n1 * *n2)
                    || a == format!("{}{}", *n1, *n2).parse::<i64>().unwrap()
            }
            [n1, n2, rest @ ..] => {
                let mut v1 = vec![*n1 + *n2];
                let mut v2 = vec![*n1 * *n2];
                let mut v3 = vec![format!("{}{}", *n1, *n2).parse::<i64>().unwrap()];
                v1.extend(rest);
                v2.extend(rest);
                v3.extend(rest);
                is_valid(a, &v1) || is_valid(a, &v2) || is_valid(a, &v3)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(3749, part1::process(input));
        assert_eq!(11387, part2::process(input));
    }
}
