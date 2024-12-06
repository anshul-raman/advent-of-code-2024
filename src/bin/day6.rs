use std::time::Instant;

fn main() {
    let input = include_str!("../../puzzle_inputs/day6.txt");

    let start = Instant::now();
    let part1_result = part1::process(input);
    let part1_time = start.elapsed();

    let start = Instant::now();
    let part2_result = part2::process(input);
    let part2_time = start.elapsed();

    println!("Part1: {} (took {:?})", part1_result, part1_time);
    println!("Part2: {} (took {:?})", part2_result, part2_time);
}

mod part1 {
    use crate::shared::*;
    use std::collections::HashSet;

    pub fn process(input: &str) -> usize {
        let matrix = parse(input);
        let (mut pos, mut dir) = find_starting_pos(&matrix);

        let mut locations = HashSet::new();
        while pos.is_valid() {
            if let Some(next_pos) = pos.move_in_dir(&dir) {
                // change direction if obstacle is found
                if matrix[next_pos.r_idx][next_pos.c_idx] == '#' {
                    dir.turn_right();
                    continue;
                }

                // add to visited location.
                locations.insert((next_pos.r_idx, next_pos.c_idx));
                pos = next_pos;
            } else {
                break;
            }
        }

        locations.len()
    }
}

mod part2 {
    use crate::shared::*;
    use std::{char, collections::HashSet};

    pub fn process(input: &str) -> usize {
        let matrix = parse(input);
        let (pos, dir) = find_starting_pos(&matrix);
        let (mut hor_obs, mut ver_obs) = build_obstacle_maps(&matrix);
        let mut visited = initial_path(&matrix, pos, dir);
        visited.remove(&(pos.r_idx, pos.c_idx));

        visited
            .iter()
            .filter(|&&(x, y)| {
                hor_obs[x].push(y);
                ver_obs[y].push(x);
                let res = is_loop(pos, dir, &hor_obs, &ver_obs);
                hor_obs[x].pop();
                ver_obs[y].pop();
                res
            })
            .count()
    }

    fn initial_path(
        matrix: &[Vec<char>],
        start_pos: Position,
        start_dir: Direction,
    ) -> HashSet<(usize, usize)> {
        let mut visited = HashSet::new();
        let mut pos = start_pos;
        let mut dir = start_dir;

        while let Some(next_pos) = pos.move_in_dir(&dir) {
            if matrix[next_pos.r_idx][next_pos.c_idx] == '#' {
                dir.turn_right();
                continue;
            }
            visited.insert((next_pos.r_idx, next_pos.c_idx));
            pos = next_pos;
        }
        visited
    }

    fn build_obstacle_maps(matrix: &[Vec<char>]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
        let mut hor = vec![Vec::new(); matrix.len()];
        let mut ver = vec![Vec::new(); matrix[0].len()];

        for (i, row) in matrix.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == '#' {
                    hor[i].push(j);
                    ver[j].push(i);
                }
            }
        }
        (hor, ver)
    }

    fn is_loop(
        pos: Position,
        dir: Direction,
        hor_obs: &[Vec<usize>],
        ver_obs: &[Vec<usize>],
    ) -> bool {
        let mut visited = HashSet::new();
        let mut cur_pos = pos;
        let mut cur_dir = dir;

        loop {
            let next_obs = match cur_dir {
                Direction::North => ver_obs[cur_pos.c_idx]
                    .iter()
                    .filter(|&&x| x <= cur_pos.r_idx)
                    .max(),
                Direction::South => ver_obs[cur_pos.c_idx]
                    .iter()
                    .filter(|&&x| x >= cur_pos.r_idx)
                    .min(),
                Direction::East => hor_obs[cur_pos.r_idx]
                    .iter()
                    .filter(|&&x| x >= cur_pos.c_idx)
                    .min(),
                Direction::West => hor_obs[cur_pos.r_idx]
                    .iter()
                    .filter(|&&x| x <= cur_pos.c_idx)
                    .max(),
            };

            let next_obs = match next_obs {
                Some(&x) => x,
                None => return false,
            };

            match cur_dir {
                Direction::North => cur_pos.r_idx = next_obs + 1,
                Direction::South => cur_pos.r_idx = next_obs - 1,
                Direction::East => cur_pos.c_idx = next_obs - 1,
                Direction::West => cur_pos.c_idx = next_obs + 1,
            }

            cur_dir.turn_right();

            if !visited.insert((cur_pos.r_idx, cur_pos.c_idx, cur_dir)) {
                return true;
            }
        }
    }
}

mod shared {

    #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
    pub enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Direction {
        pub fn turn_right(&mut self) {
            match self {
                Direction::North => *self = Direction::East,
                Direction::East => *self = Direction::South,
                Direction::South => *self = Direction::West,
                Direction::West => *self = Direction::North,
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Position {
        pub r_idx: usize,
        pub c_idx: usize,
        max_r_idx: usize,
        max_c_idx: usize,
    }

    impl Position {
        pub fn is_valid(&self) -> bool {
            self.r_idx <= self.max_r_idx && self.c_idx <= self.max_c_idx
        }

        pub fn move_in_dir(&self, dir: &Direction) -> Option<Self> {
            let mut new_pos = *self;
            match dir {
                Direction::North if self.r_idx > 0 => new_pos.r_idx -= 1,
                Direction::East if self.c_idx < self.max_c_idx => new_pos.c_idx += 1,
                Direction::South if self.r_idx < self.max_r_idx => new_pos.r_idx += 1,
                Direction::West if self.c_idx > 0 => new_pos.c_idx -= 1,
                _ => return None,
            }
            Some(new_pos)
        }
    }

    pub fn parse(input: &str) -> Vec<Vec<char>> {
        input.trim().lines().map(|l| l.chars().collect()).collect()
    }

    pub fn find_starting_pos(input: &[Vec<char>]) -> (Position, Direction) {
        let max_r = input.len() - 1;
        let max_c = input[0].len() - 1;

        for (x, row) in input.iter().enumerate() {
            for (y, &c) in row.iter().enumerate() {
                let dir = match c {
                    '^' => Direction::North,
                    '>' => Direction::East,
                    'v' => Direction::South,
                    '<' => Direction::West,
                    _ => continue,
                };
                return (
                    Position {
                        r_idx: x,
                        c_idx: y,
                        max_r_idx: max_r,
                        max_c_idx: max_c,
                    },
                    dir,
                );
            }
        }
        panic!("Invalid input. Starting position not found");
    }
}

#[cfg(test)]
mod test {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(41, part1::process(input));
        assert_eq!(6, part2::process(input));
    }
}
