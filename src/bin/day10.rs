fn main() {
    let input = include_str!("../../puzzle_inputs/day10.txt");
    println!("Part1: {}", part1::process(input));
    println!("Part2: {}", part2::process(input));
}

mod part1 {
    use std::collections::HashSet;

    pub fn process(input: &str) -> i32 {
        let matrix = parse(input);

        let mut scores = vec![vec![(-1, HashSet::new()); matrix[0].len()]; matrix.len()];

        for (ridx, r) in matrix.iter().enumerate() {
            for (cidx, el) in r.iter().enumerate() {
                calc_score(ridx, cidx, &mut scores, &matrix);
            }
        }

        let mut ans = 0;
        for (ridx, r) in matrix.iter().enumerate() {
            for (cidx, el) in r.iter().enumerate() {
                if *el == 0 {
                    ans += scores[ridx][cidx].0;
                }
            }
        }

        ans
    }

    fn calc_score(
        ridx: usize,
        cidx: usize,
        scores: &mut Vec<Vec<(i32, HashSet<(usize, usize)>)>>,
        matrix: &Vec<Vec<u32>>,
    ) {
        if ridx >= matrix.len() || cidx >= matrix[0].len() {
            return;
        }

        if scores[ridx][cidx].0 != -1 {
            return;
        }

        let cr = matrix[ridx][cidx];

        if cr == 9 {
            scores[ridx][cidx].0 = 1;
            scores[ridx][cidx].1.insert((ridx, cidx));
            return;
        }

        // Create a temporary HashSet to collect all values
        let mut temp_set = HashSet::new();

        // check up
        if ridx != 0 && matrix[ridx - 1][cidx] == cr + 1 {
            calc_score(ridx - 1, cidx, scores, matrix);
            temp_set.extend(scores[ridx - 1][cidx].1.iter().copied());
        }

        // check right
        if cidx + 1 != matrix[0].len() && matrix[ridx][cidx + 1] == cr + 1 {
            calc_score(ridx, cidx + 1, scores, matrix);
            temp_set.extend(scores[ridx][cidx + 1].1.iter().copied());
        }

        // check down
        if ridx + 1 != matrix.len() && matrix[ridx + 1][cidx] == cr + 1 {
            calc_score(ridx + 1, cidx, scores, matrix);
            temp_set.extend(scores[ridx + 1][cidx].1.iter().copied());
        }

        // check left
        if cidx != 0 && matrix[ridx][cidx - 1] == cr + 1 {
            calc_score(ridx, cidx - 1, scores, matrix);
            temp_set.extend(scores[ridx][cidx - 1].1.iter().copied());
        }

        // Update the final set all at once
        scores[ridx][cidx].1.extend(temp_set);
        let vs = scores[ridx][cidx].1.len();
        scores[ridx][cidx].0 = vs as i32;
    }

    fn parse(input: &str) -> Vec<Vec<u32>> {
        input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }
}

mod part2 {

    pub fn process(input: &str) -> i32 {
        let matrix = parse(input);

        let mut scores = vec![vec![-1; matrix[0].len()]; matrix.len()];

        for (ridx, r) in matrix.iter().enumerate() {
            for (cidx, el) in r.iter().enumerate() {
                if *el == 0 {
                    calc_score(ridx, cidx, &mut scores, &matrix);
                }
            }
        }

        let mut ans = 0;
        for (ridx, r) in matrix.iter().enumerate() {
            for (cidx, el) in r.iter().enumerate() {
                if *el == 0 {
                    ans += scores[ridx][cidx];
                }
            }
        }

        ans
    }

    fn calc_score(ridx: usize, cidx: usize, scores: &mut Vec<Vec<i32>>, matrix: &Vec<Vec<u32>>) {
        if ridx >= matrix.len() || cidx >= matrix[0].len() {
            return;
        }

        if scores[ridx][cidx] != -1 {
            return;
        }

        let cr = matrix[ridx][cidx];

        if cr == 9 {
            scores[ridx][cidx] = 1;
            return;
        }

        let mut uniq_trails = 0;

        // check up
        if ridx != 0 && matrix[ridx - 1][cidx] == cr + 1 {
            calc_score(ridx - 1, cidx, scores, matrix);
            uniq_trails += scores[ridx - 1][cidx];
        }

        // check right
        if cidx + 1 != matrix[0].len() && matrix[ridx][cidx + 1] == cr + 1 {
            calc_score(ridx, cidx + 1, scores, matrix);
            uniq_trails += scores[ridx][cidx + 1];
        }

        // check down
        if ridx + 1 != matrix.len() && matrix[ridx + 1][cidx] == cr + 1 {
            calc_score(ridx + 1, cidx, scores, matrix);
            uniq_trails += scores[ridx + 1][cidx];
        }

        // check left
        if cidx != 0 && matrix[ridx][cidx - 1] == cr + 1 {
            calc_score(ridx, cidx - 1, scores, matrix);
            uniq_trails += scores[ridx][cidx - 1];
        }

        scores[ridx][cidx] = uniq_trails;
    }

    fn parse(input: &str) -> Vec<Vec<u32>> {
        input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(36, part1::process(input));
        assert_eq!(81, part2::process(input));
    }
}
