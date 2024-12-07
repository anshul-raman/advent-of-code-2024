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

    fn generate_all_ops(n: usize) -> Vec<Vec<i64>> {
        let total_sequences = 3_i64.pow(n as u32);
        let mut result = Vec::with_capacity(total_sequences as usize);

        for i in 0..total_sequences {
            let mut current = i;
            let mut sequence = vec![0; n];

            for j in (0..n).rev() {
                sequence[j] = current % 3;
                current /= 3;
            }

            result.push(sequence);
        }

        result
    }

    fn is_valid(a: i64, b: &[i64]) -> bool {
        let n = b.len() - 1;
        for ops in generate_all_ops(n) {
            let mut k = b[0];
            for (idx, &el) in b.iter().skip(1).enumerate() {
                match ops[idx] {
                    0 => k += el,
                    1 => k *= el,
                    2 => k = format!("{}{}", k, el).parse::<i64>().unwrap(),
                    _ => unreachable!(),
                }
                if k > a {
                    break;
                }
            }

            if k == a {
                return true;
            }
        }

        false
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
