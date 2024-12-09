use std::ops::{Add, Sub};

fn main() {
    let input = include_str!("../../puzzle_inputs/day8.txt");
    println!("Part1: {}", part1::process(input));
    println!("Part2: {}", part2::process(input));
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    fn mul<T>(self, factor: T) -> Point
    where
        i64: std::ops::Mul<T, Output = i64>,
        T: Copy,
    {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

mod part1 {
    use std::collections::{HashMap, HashSet};

    use crate::Point;

    pub fn process(input: &str) -> usize {
        let matrix = parse(input);

        let mut antenas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for (idx, r) in matrix.iter().enumerate() {
            for (cidsx, el) in r.iter().enumerate() {
                match *el {
                    '0'..='9' | 'a'..='z' | 'A'..='Z' => antenas
                        .entry(*el)
                        .and_modify(|v| v.push((idx, cidsx)))
                        .or_insert_with(|| {
                            let v = vec![(idx, cidsx)];
                            v
                        }),
                    _ => continue,
                };
            }
        }

        let mut locations = HashSet::new();

        for (_, ant_locations) in antenas.iter() {
            for (i, a) in ant_locations.iter().enumerate() {
                for b in ant_locations.iter().skip(i + 1) {
                    let a = Point::from(*a);
                    let b = Point::from(*b);

                    let delta = b - a;
                    locations.insert(a - delta);
                    locations.insert(b + delta);
                }
            }
        }

        dbg!(&locations);

        locations.len()
    }

    fn parse(input: &str) -> Vec<Vec<char>> {
        input
            .trim()
            .lines()
            .map(|l| l.trim().chars().collect())
            .collect()
    }
}

mod part2 {
    use crate::Point;
    use std::collections::{HashMap, HashSet};

    pub fn process(input: &str) -> usize {
        let matrix = parse(input);

        let mut antenas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for (idx, r) in matrix.iter().enumerate() {
            for (cidsx, el) in r.iter().enumerate() {
                match *el {
                    '0'..='9' | 'a'..='z' | 'A'..='Z' => antenas
                        .entry(*el)
                        .and_modify(|v| v.push((idx, cidsx)))
                        .or_insert_with(|| {
                            let v = vec![(idx, cidsx)];
                            v
                        }),
                    _ => continue,
                };
            }
        }

        let mut locations = HashSet::new();

        for (_, ant_locations) in antenas.iter() {
            for (i, a) in ant_locations.iter().enumerate() {
                for b in ant_locations.iter().skip(i + 1) {
                    let a = Point::from(*a);
                    let b = Point::from(*b);

                    let delta = b - a;
                    for dir in [-1, 1] {
                        let dir_delta = delta.mul(dir);
                        let mut cur = a;
                        while is_valid(cur, matrix.len(), matrix[0].len()) {
                            locations.insert(cur);
                            cur = cur + dir_delta;
                        }
                    }
                }
            }
        }

        locations.len()
    }

    fn is_valid(p: Point, rmax: usize, cmax: usize) -> bool {
        0 <= p.x && (p.x as usize) < rmax && 0 <= p.y && (p.y as usize) < cmax
    }

    fn parse(input: &str) -> Vec<Vec<char>> {
        input
            .trim()
            .lines()
            .map(|l| l.trim().chars().collect())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(14, part1::process(input));
        assert_eq!(34, part2::process(input));
    }
}
