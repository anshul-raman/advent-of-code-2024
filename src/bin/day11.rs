use std::collections::HashMap;

fn main() {
    let input = include_str!("../../puzzle_inputs/day11.txt");
    let mut cache = HashMap::new();
    println!("Part1: {}", process(input, 25, &mut cache));
    println!("Part2: {}", process(input, 75, &mut cache));
}

fn process(input: &str, count: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .map(|n| blink(n, count, cache))
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
            Some((left, right)) => {
                blink(left, count - 1, cache) + blink(right, count - 1, cache)
            }
            None => blink(a * 2024, count - 1, cache)
        }
    };

    cache.insert((a, count), result);
    result
}

fn split_digit(mut n: usize) -> Option<(usize, usize)> {
    // Count digits
    let mut len = 0;
    let mut temp = n;
    while temp > 0 {
        len += 1;
        temp /= 10;
    }
    
    if len % 2 != 0 {
        return None;
    }
    
    // Calculate divisor for splitting (10^(len/2))
    let divisor = 10_usize.pow((len / 2) as u32);
    let right = n % divisor;
    let left = n / divisor;
    
    Some((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        let mut cache = HashMap::new();
        assert_eq!(55312, process(input, 25, &mut cache));
    }
}