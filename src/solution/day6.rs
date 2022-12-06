pub mod day6 {
    use std::collections::HashMap;
    impl crate::solution::advent_of_code_2022::Solution {
        pub fn day6(input: &str) -> usize {
            let s_chars: Vec<char> = input.chars().collect();
            let mut letter_hash = HashMap::<char, usize>::new();

            let mut min_diff = 0;
            let mut char_index = 0;
            for c in s_chars {
                if min_diff >= 5 {
                    return char_index;
                }
                match letter_hash.get_mut(&c) {
                    Some(v) => {
                        min_diff = std::cmp::min(min_diff, char_index - *v);
                        *v = char_index;
                    }
                    None => {
                        letter_hash.insert(c, char_index);
                    }
                }

                char_index = char_index + 1;
                min_diff = min_diff + 1;
            }

            char_index
        }
        pub fn day6_part2(input: &str) -> usize {
            let s_chars: Vec<char> = input.chars().collect();
            let mut letter_hash = HashMap::<char, usize>::new();

            let mut min_diff = 0;
            let mut char_index = 0;
            for c in s_chars {
                if min_diff >= 15 {
                    return char_index;
                }
                match letter_hash.get_mut(&c) {
                    Some(v) => {
                        min_diff = std::cmp::min(min_diff, char_index - *v);
                        *v = char_index;
                    }
                    None => {
                        letter_hash.insert(c, char_index);
                    }
                }

                char_index = char_index + 1;
                min_diff = min_diff + 1;
            }

            char_index
        }
    }
}
