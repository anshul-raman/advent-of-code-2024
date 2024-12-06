fn main() {
    let input = include_str!("../../puzzle_inputs/day6.txt");
    println!("Part1: {}", part1::process(input));
    println!("Part2: {}", part2::process(input));
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
                if matrix[next_pos.x][next_pos.y] == '#' {
                    dir.turn_right();
                    continue;
                }

                // add to visited location.
                locations.insert((next_pos.x, next_pos.y));
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
        let mut matrix = parse(input);
        let (pos, dir) = find_starting_pos(&matrix);

        let mut visited_locations = HashSet::new();
        let mut cur_dir = dir;
        let mut cur_pos = pos.clone();
        while cur_pos.is_valid() {
            if let Some(next_pos) = cur_pos.move_in_dir(&cur_dir) {
                if matrix[next_pos.x][next_pos.y] == '#' {
                    cur_dir.turn_right();
                    continue;
                }

                visited_locations.insert((next_pos.x, next_pos.y));
                cur_pos = next_pos;
            } else {
                break;
            }
        }

        let mut ans = 0;
        for &(x, y) in visited_locations.iter() {
            if x == pos.x && y == pos.y {
                continue;
            }

            matrix[x][y] = '#';
            if is_loop(&matrix, &pos, &dir) {
                ans += 1;
            }
            matrix[x][y] = '.';
        }
        ans
    }

    fn is_loop(matrix: &[Vec<char>], pos: &Position, dir: &Direction) -> bool {
        let mut locations = HashSet::new();
        let mut pos = pos.clone();
        let mut dir = *dir;

        while pos.is_valid() {
            if let Some(next_pos) = pos.move_in_dir(&dir) {
                if matrix[next_pos.x][next_pos.y] == '#' {
                    dir.turn_right();
                    continue;
                }

                if locations.contains(&(pos.x, pos.y, dir)) {
                    return true;
                }

                locations.insert((pos.x, pos.y, dir));
                pos = next_pos;
            } else {
                break;
            }
        }

        false
    }
}

mod shared {

    #[derive(Copy, Clone, Hash, PartialEq, Eq)]
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
        pub x: usize,
        pub y: usize,
        max_x: usize,
        max_y: usize,
    }

    impl Position {
        pub fn is_valid(&self) -> bool {
            self.x <= self.max_x && self.y <= self.max_y
        }

        pub fn move_in_dir(&self, dir: &Direction) -> Option<Self> {
            match dir {
                Direction::North => {
                    if self.x > 0 {
                        let mut new_pos = self.clone();
                        new_pos.x -= 1;
                        return Some(new_pos);
                    }
                }
                Direction::East => {
                    if self.y < self.max_y {
                        let mut new_pos = self.clone();
                        new_pos.y += 1;
                        return Some(new_pos);
                    }
                }
                Direction::West => {
                    if self.y > 0 {
                        let mut new_pos = self.clone();
                        new_pos.y -= 1;
                        return Some(new_pos);
                    }
                }
                Direction::South => {
                    if self.x < self.max_x {
                        let mut new_pos = self.clone();
                        new_pos.x += 1;
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
                        return (Position { x, y, max_x, max_y }, Direction::North);
                    }
                    '>' => {
                        return (Position { x, y, max_x, max_y }, Direction::East);
                    }
                    'v' => {
                        return (Position { x, y, max_x, max_y }, Direction::South);
                    }
                    '<' => {
                        return (Position { x, y, max_x, max_y }, Direction::West);
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
