fn main() {
    let input = include_str!("../../puzzle_inputs/day4.txt");
    println!("Part1: {}", part1::process(input));
    println!("Part2: {}", part2::process(input));
}

mod part1 {
    use crate::shared::*;

    pub fn process(input: &str) -> usize {
        let matrix = to_matrix(input);
        let m = matrix.len() - 1;
        let n = matrix[0].len() - 1;

        let mut count = 0usize;

        // Finding horizontal
        for mat in matrix.iter() {
            count += find(mat);
        }

        let mut temp_slice = Vec::with_capacity(matrix.len());

        // Finding Vertical
        for j in 0..=n {
            for row in matrix.iter().take(m + 1) {
                temp_slice.push(row[j])
            }

            count += find(&temp_slice);
            temp_slice.clear();
        }

        // left diagonal half
        temp_slice.clear();
        for j in 0..=n {
            let alpha = 0;
            let beta = j;
            let mut k = 0;
            while alpha + k <= m && beta + k <= n {
                temp_slice.push(matrix[alpha + k][beta + k]);
                k += 1;
            }

            count += find(&temp_slice);
            temp_slice.clear();
        }

        // left diagonal second half
        temp_slice.clear();
        for i in 1..=m {
            let alpha = i;
            let beta = 0;
            let mut k = 0;
            while alpha + k <= m && beta + k <= n {
                temp_slice.push(matrix[alpha + k][beta + k]);
                k += 1;
            }

            count += find(&temp_slice);
            temp_slice.clear();
        }

        // right diagonal
        temp_slice.clear();
        for j in 0..=n {
            let alpha = 0;
            let beta = j;
            let mut k = 0;
            while alpha + k <= m && beta >= k {
                temp_slice.push(matrix[alpha + k][beta - k]);
                k += 1;
            }

            count += find(&temp_slice);
            temp_slice.clear();
        }

        // right diagonal second half;
        for i in 1..=m {
            let alpha = i;
            let beta = n;
            let mut k = 0;
            while alpha + k <= m && beta >= k {
                temp_slice.push(matrix[alpha + k][beta - k]);
                k += 1;
            }
            count += find(&temp_slice);
            temp_slice.clear();
        }

        count
    }

    // Counts number of XMAS in a &[char] forward and backward
    fn find(slice: &[char]) -> usize {
        if slice.len() < 4 {
            return 0;
        }

        slice
            .windows(4)
            .filter(|window| {
                window.iter().copied().eq("XMAS".chars())
                    || window.iter().copied().eq("SAMX".chars())
            })
            .count()
    }

    #[cfg(test)]
    mod test {
        use crate::part1;

        #[test]
        fn test_part1() {
            let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

            assert_eq!(18, part1::process(input))
        }
    }
}

mod part2 {
    use crate::shared::*;

    pub fn process(input: &str) -> usize {
        let matrix = to_matrix(input);
        let mut count = 0;
        let mut slice = Vec::with_capacity(9);

        for i in 0..matrix.len().saturating_sub(2) {
            for j in 0..matrix[0].len().saturating_sub(2) {
                slice.clear();
                for row in matrix.iter().skip(i).take(3) {
                    for ch in row.iter().skip(j).take(3) {
                        slice.push(*ch);
                    }
                }

                if is_x_mas(&slice) {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_x_mas(input: &[char]) -> bool {
        matches!(
            input,
            ['M', _, 'M', _, 'A', _, 'S', _, 'S']
                | ['S', _, 'M', _, 'A', _, 'S', _, 'M']
                | ['M', _, 'S', _, 'A', _, 'M', _, 'S']
                | ['S', _, 'S', _, 'A', _, 'M', _, 'M']
        )
    }

    #[cfg(test)]
    mod test {
        use crate::part2;

        #[test]
        fn test_part2() {
            let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

            assert_eq!(9, part2::process(input));
        }
    }
}

mod shared {
    pub fn to_matrix(input: &str) -> Vec<Vec<char>> {
        let mut matrix = vec![];
        input.trim().lines().for_each(|l| {
            matrix.push(Vec::from_iter(l.chars()));
        });
        matrix
    }
}
