fn main() {
    let input = include_str!("../../puzzle_inputs/day9.txt");
    println!("Part1: {}", part1::process(input));
    println!("Part2: {}", part2::process(input));
}

#[derive(Copy, Clone, Debug)]
enum Item {
    File(i64, i64),
    Empty(i64),
}

mod part1 {

    use core::panic;

    use crate::Item;

    pub fn process(input: &str) -> i64 {
        let mut files: Vec<Item> = parse(input);
        let mut processed: Vec<Item> = vec![];
        let mut a = 0;
        let mut b = files.len() - 1;
        while a <= b {
            let el = files[a];
            match el {
                Item::File(_, _) => {
                    processed.push(el);
                    a += 1;
                }
                Item::Empty(a_s) => {
                    //
                    let l_el = files[b];
                    match l_el {
                        Item::Empty(_) => b -= 1,
                        Item::File(b_id, b_s) => match a_s.cmp(&b_s) {
                            std::cmp::Ordering::Less => {
                                processed.push(Item::File(b_id, a_s));
                                files[b] = Item::File(b_id, b_s - a_s);
                                a += 1;
                            }
                            std::cmp::Ordering::Greater => {
                                processed.push(Item::File(b_id, b_s));
                                files[a] = Item::Empty(a_s - b_s);
                                b -= 1;
                            }
                            std::cmp::Ordering::Equal => {
                                processed.push(Item::File(b_id, b_s));
                                b -= 1;
                                a += 1;
                            }
                        },
                    }
                }
            }
        }

        let mut ans = 0;
        let mut pos = 0;
        let mut p = 0;
        while pos < processed.len() {
            let el = processed[pos];
            match el {
                Item::Empty(_) => panic!("invalid processed array"),
                Item::File(id, s) => {
                    // assert!(s != 0);
                    let asum = (s - 1) * s / 2;
                    ans += id * (p * s + asum);
                    pos += 1;
                    p += s;
                }
            }
        }

        ans
    }

    fn parse(input: &str) -> Vec<Item> {
        let mut id = 0;
        input
            .trim()
            .chars()
            .enumerate()
            .map(|(idx, el)| {
                let num = el.to_string().parse().unwrap();
                if idx % 2 == 0 {
                    assert!(num != 0);
                    let it = Item::File(id, num);
                    id += 1;
                    it
                } else {
                    Item::Empty(num)
                }
            })
            .collect()
    }
}

mod part2 {
    use crate::Item;

    pub fn process(input: &str) -> i64 {
        let mut files: Vec<Item> = parse(input);
        let mut temp: Vec<Item> = Vec::with_capacity(files.len());
        let ln = files.len();

        for idx in (0..ln).rev() {
            match files[idx] {
                Item::Empty(_) => continue,
                Item::File(_, s) => {
                    //
                    // find empty slot from right;
                    let mut slot = 0;
                    let mut found = false;
                    let mut esf = 0;
                    for j in 0..idx {
                        if let Item::Empty(es) = files[j] {
                            if es >= s {
                                // set vars
                                slot = j;
                                found = true;
                                esf = es;
                                break;
                            }
                        }
                    }

                    // Update the slot.
                    if found {
                        if esf == s {
                            files[slot] = files[idx];
                            files[idx] = Item::Empty(s);
                        } else {
                            files.insert(slot, files[idx]);
                            files[slot + 1] = Item::Empty(esf - s);
                            files[idx + 1] = Item::Empty(s);
                        }
                    }
                }
            }

            // join empty slots;
            while let Some(i) = files.pop() {
                temp.push(i);
            }

            while let Some(i) = temp.pop() {
                if files.is_empty() {
                    files.push(i);
                    continue;
                }

                match i {
                    Item::File(_, _) => files.push(i),
                    Item::Empty(is) => {
                        //
                        let tel = files[files.len() - 1];
                        match tel {
                            Item::File(_, _) => files.push(i),
                            Item::Empty(tels) => {
                                let ln = files.len() - 1;
                                files[ln] = Item::Empty(tels + is);
                            }
                        }
                    }
                }
            }
        }

        // calculate checksum

        let mut ans = 0;
        let mut pos = 0;
        let mut p = 0;
        while pos < files.len() {
            let el = files[pos];
            match el {
                Item::Empty(s) => {
                    pos += 1;
                    p += s;
                }
                Item::File(id, s) => {
                    // assert!(s != 0);
                    let asum = (s - 1) * s / 2;
                    ans += id * (p * s + asum);
                    pos += 1;
                    p += s;
                }
            }
        }

        ans
    }

    fn parse(input: &str) -> Vec<Item> {
        let mut id = 0;
        input
            .trim()
            .chars()
            .enumerate()
            .map(|(idx, el)| {
                let num = el.to_string().parse().unwrap();
                if idx % 2 == 0 {
                    assert!(num != 0);
                    let it = Item::File(id, num);
                    id += 1;
                    it
                } else {
                    Item::Empty(num)
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test() {
        let input = "2333133121414131402";
        assert_eq!(1928, part1::process(input));
        assert_eq!(2858, part2::process(input));
    }
}
