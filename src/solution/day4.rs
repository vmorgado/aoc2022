pub mod day4 {

    impl crate::solution::advent_of_code_2022::Solution {
        pub fn day4(input: &str) -> i32 {
            let groups = input.split("\n");
            let mut result = 0;

            for group in groups {
                let elfs: Vec<&str> = group.split(',').collect();

                let elf_one: Vec<&str> = elfs.get(0).unwrap().split('-').collect();
                let elf_two: Vec<&str> = elfs.get(1).unwrap().split('-').collect();

                let elf_one_start = elf_one.get(0).unwrap().parse::<i32>().unwrap();
                let elf_one_end = elf_one.get(1).unwrap().parse::<i32>().unwrap();

                let elf_two_start = elf_two.get(0).unwrap().parse::<i32>().unwrap();
                let elf_two_end = elf_two.get(1).unwrap().parse::<i32>().unwrap();

                if elf_one_start >= elf_two_start && elf_one_end <= elf_two_end {
                    result = result + 1;
                    continue;
                }
                if elf_one_start <= elf_two_start && elf_one_end >= elf_two_end {
                    result = result + 1;
                    continue;
                }
            }

            result
        }

        pub fn day4_part2(input: &str) -> i32 {
            let groups = input.split("\n");
            let mut result = 0;

            for group in groups {
                let elfs: Vec<&str> = group.split(',').collect();

                let elf_one: Vec<&str> = elfs.get(0).unwrap().split('-').collect();
                let elf_two: Vec<&str> = elfs.get(1).unwrap().split('-').collect();

                let elf_one_start = elf_one.get(0).unwrap().parse::<i32>().unwrap();
                let elf_one_end = elf_one.get(1).unwrap().parse::<i32>().unwrap();

                let elf_two_start = elf_two.get(0).unwrap().parse::<i32>().unwrap();
                let elf_two_end = elf_two.get(1).unwrap().parse::<i32>().unwrap();

                let max_start = std::cmp::max(elf_one_start, elf_two_start);
                let min_end = std::cmp::min(elf_one_end, elf_two_end);

                let overlap = min_end - max_start;

                if overlap >= 0 {
                    result = result + 1;
                }
            }
            result
        }
    }
}
