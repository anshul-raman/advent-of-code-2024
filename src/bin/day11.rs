use std::collections::HashMap;

fn main() {
    let input = include_str!("../../puzzle_inputs/day11.txt");
    println!("Part1: {}", process(input, 25));
    println!("Part2: {}", process(input, 75));
}

fn process(input: &str, count: usize) -> usize {
    input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .map(|n| blink(n, count, &mut HashMap::new()))
        .sum()
}

fn blink(a: usize, count: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if count == 0 {
        return 1;
    }

    if let Some(&v) = cache.get(&(a, count)) {
        return v;
    }

    let result = if a == 0 {
        blink(1, count - 1, cache)
    } else {
        match split_digit(a) {
            Some((left, right)) => blink(left, count - 1, cache) + blink(right, count - 1, cache),
            None => blink(a * 2024, count - 1, cache),
        }
    };

    cache.insert((a, count), result);
    result
}

fn split_digit(a: usize) -> Option<(usize, usize)> {
    let digits: Vec<_> = a
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .collect();

    if digits.len() % 2 != 0 {
        return None;
    }

    let mid = digits.len() / 2;
    Some((
        digits[..mid].iter().fold(0, |acc, &d| acc * 10 + d),
        digits[mid..].iter().fold(0, |acc, &d| acc * 10 + d),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        assert_eq!(55312, process(input, 25));
    }
}
