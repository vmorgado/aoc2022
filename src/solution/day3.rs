pub mod day3 {
    use std::collections::HashMap;

    pub fn get_priority(item: char) -> i32 {
        let ascii_code: i32 = item as i32 + 0;
        if ascii_code < 97 {
            return ascii_code - 64 + 26;
        }

        ascii_code - 96
    }

    impl crate::solution::advent_of_code_2022::Solution {
        pub fn day3(input: &str) -> i32 {
            let mut prio_agr = 0;

            for line in input.split("\n") {
                let mut existing_one = HashMap::<char, bool>::new();
                let mut existing_two = HashMap::<char, bool>::new();
                let inv_len: i32 = line.chars().count() as i32 / 2;
                let mut i = 0;
                for c in line.chars() {
                    let prio: i32 = get_priority(c);
                    i = i + 1;

                    if i > inv_len {
                        if existing_one.contains_key(&c) && !existing_two.contains_key(&c) {
                            prio_agr = prio + prio_agr;
                            existing_two.insert(c, true);
                        }
                        continue;
                    }
                    existing_one.insert(c, true);
                }
            }

            prio_agr
        }
        pub fn day3_part2(input: &str) -> i32 {
            let mut prio_agr = 0;

            let lines: Vec<&str> = input.split("\n").collect();
            for group in lines.chunks(3) {
                let mut existing = HashMap::<char, i32>::new();
                for bag in group {
                    let mut already = HashMap::<char, bool>::new();
                    for c in bag.chars() {
                        if already.contains_key(&c) {
                            continue;
                        }

                        already.insert(c, true);

                        if !existing.contains_key(&c) {
                            existing.insert(c, 1);
                            continue;
                        }

                        let val: i32 = *existing.get(&c).unwrap();
                        existing.insert(c, val + 1);
                    }
                }
                let (item, _times) = existing
                    .into_iter()
                    .filter(|&(_k, v)| v == 3)
                    .nth(0)
                    .unwrap();
                prio_agr = prio_agr + get_priority(item);
            }

            prio_agr
        }
    }
}
