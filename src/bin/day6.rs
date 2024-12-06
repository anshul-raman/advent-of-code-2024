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
    use std::{
        char,
        cmp::{max, min},
        collections::HashSet,
    };

    pub fn process(input: &str) -> usize {
        let matrix = parse(input);
        let (pos, dir) = find_starting_pos(&matrix);
        let mut hor_obs = hor_obs(&matrix);
        let mut ver_obs = ver_obs(&matrix);

        let mut visited_locations = HashSet::new();
        let mut cur_dir = dir;
        let mut cur_pos = pos.clone();
        while cur_pos.is_valid() {
            if let Some(next_pos) = cur_pos.move_in_dir(&cur_dir) {
                if matrix[next_pos.r_idx][next_pos.c_idx] == '#' {
                    cur_dir.turn_right();
                    continue;
                }

                visited_locations.insert((next_pos.r_idx, next_pos.c_idx));
                cur_pos = next_pos;
            } else {
                break;
            }
        }

        let mut ans = 0;
        for &(x, y) in visited_locations.iter() {
            if x == pos.r_idx && y == pos.c_idx {
                continue;
            }

            hor_obs[x].push(y);
            ver_obs[y].push(x);
            if is_loop(&pos, &dir, &hor_obs, &ver_obs) {
                ans += 1;
            }
            hor_obs[x].pop();
            ver_obs[y].pop();
        }
        ans
    }

    fn hor_obs(matrix: &[Vec<char>]) -> Vec<Vec<usize>> {
        let mut obs = vec![Vec::new(); matrix.len()];
        for (idx, r) in matrix.iter().enumerate() {
            for (cidx, el) in r.iter().enumerate() {
                if *el == '#' {
                    obs[idx].push(cidx);
                }
            }
        }

        obs
    }

    fn ver_obs(matrix: &[Vec<char>]) -> Vec<Vec<usize>> {
        let mut obs = vec![Vec::new(); matrix[0].len()];
        for j in 0..matrix[0].len() {
            for i in 0..matrix.len() {
                if matrix[i][j] == '#' {
                    obs[j].push(i);
                }
            }
        }

        obs
    }

    fn is_loop(
        pos: &Position,
        dir: &Direction,
        hor_obs: &[Vec<usize>],
        ver_obs: &[Vec<usize>],
    ) -> bool {
        let mut locations = HashSet::new();
        let mut pos = pos.clone();
        let mut dir = *dir;

        while pos.is_valid() {
            let mut next_loc = pos.clone();
            let mut next_obs: Option<usize> = None;
            match dir {
                Direction::North => {
                    for &obs in &ver_obs[pos.c_idx] {
                        if obs > pos.r_idx {
                            continue;
                        }

                        if let Some(a) = next_obs {
                            next_obs = Some(max(a, obs));
                        } else {
                            next_obs = Some(obs);
                        }
                    }
                    match next_obs {
                        None => return false,
                        Some(a) => {
                            next_loc.r_idx = a + 1;
                            dir.turn_right();
                            if locations.contains(&(next_loc.r_idx, next_loc.c_idx, dir)) {
                                return true;
                            }
                            locations.insert((next_loc.r_idx, next_loc.c_idx, dir));
                        }
                    }
                }
                Direction::East => {
                    for &obs in &hor_obs[pos.r_idx] {
                        if obs < pos.c_idx {
                            continue;
                        }

                        if let Some(a) = next_obs {
                            next_obs = Some(min(a, obs));
                        } else {
                            next_obs = Some(obs);
                        }
                    }
                    match next_obs {
                        None => return false,
                        Some(a) => {
                            next_loc.c_idx = a - 1;
                            dir.turn_right();
                            if locations.contains(&(next_loc.r_idx, next_loc.c_idx, dir)) {
                                return true;
                            }

                            locations.insert((next_loc.r_idx, next_loc.c_idx, dir));
                        }
                    }
                }
                Direction::South => {
                    for &obs in &ver_obs[pos.c_idx] {
                        if obs < pos.r_idx {
                            continue;
                        }

                        if let Some(a) = next_obs {
                            next_obs = Some(min(a, obs));
                        } else {
                            next_obs = Some(obs);
                        }
                    }
                    match next_obs {
                        None => return false,
                        Some(a) => {
                            next_loc.r_idx = a - 1;
                            dir.turn_right();
                            if locations.contains(&(next_loc.r_idx, next_loc.c_idx, dir)) {
                                return true;
                            }
                            locations.insert((next_loc.r_idx, next_loc.c_idx, dir));
                        }
                    }
                }
                Direction::West => {
                    for &obs in &hor_obs[pos.r_idx] {
                        if obs > pos.c_idx {
                            continue;
                        }

                        if let Some(a) = next_obs {
                            next_obs = Some(max(a, obs));
                        } else {
                            next_obs = Some(obs);
                        }
                    }
                    match next_obs {
                        None => return false,
                        Some(a) => {
                            next_loc.c_idx = a + 1;
                            dir.turn_right();
                            if locations.contains(&(next_loc.r_idx, next_loc.c_idx, dir)) {
                                return true;
                            }

                            locations.insert((next_loc.r_idx, next_loc.c_idx, dir));
                        }
                    }
                }
            }

            pos = next_loc;
        }

        false
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

    #[derive(Clone, Debug)]
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
            match dir {
                Direction::North => {
                    if self.r_idx > 0 {
                        let mut new_pos = self.clone();
                        new_pos.r_idx -= 1;
                        return Some(new_pos);
                    }
                }
                Direction::East => {
                    if self.c_idx < self.max_c_idx {
                        let mut new_pos = self.clone();
                        new_pos.c_idx += 1;
                        return Some(new_pos);
                    }
                }
                Direction::West => {
                    if self.c_idx > 0 {
                        let mut new_pos = self.clone();
                        new_pos.c_idx -= 1;
                        return Some(new_pos);
                    }
                }
                Direction::South => {
                    if self.r_idx < self.max_r_idx {
                        let mut new_pos = self.clone();
                        new_pos.r_idx += 1;
                        return Some(new_pos);
                    }
                }
            }
            None
        }
    }

    pub fn parse(input: &str) -> Vec<Vec<char>> {
        input.trim().lines().map(|l| l.chars().collect()).collect()
    }

    pub fn find_starting_pos(input: &[Vec<char>]) -> (Position, Direction) {
        let max_x = input.len() - 1;
        let max_y = input[0].len() - 1;

        for (x, row) in input.iter().enumerate() {
            for (y, c) in row.iter().enumerate() {
                match &c {
                    '^' => {
                        return (
                            Position {
                                r_idx: x,
                                c_idx: y,
                                max_r_idx: max_x,
                                max_c_idx: max_y,
                            },
                            Direction::North,
                        );
                    }
                    '>' => {
                        return (
                            Position {
                                r_idx: x,
                                c_idx: y,
                                max_r_idx: max_x,
                                max_c_idx: max_y,
                            },
                            Direction::East,
                        );
                    }
                    'v' => {
                        return (
                            Position {
                                r_idx: x,
                                c_idx: y,
                                max_r_idx: max_x,
                                max_c_idx: max_y,
                            },
                            Direction::South,
                        );
                    }
                    '<' => {
                        return (
                            Position {
                                r_idx: x,
                                c_idx: y,
                                max_r_idx: max_x,
                                max_c_idx: max_y,
                            },
                            Direction::West,
                        );
                    }
                    _ => continue,
                }
            }
        }

        panic!("Invalid input. Starting position not found")
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
