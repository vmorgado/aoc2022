pub mod day1 {
    impl crate::solution::advent_of_code_2022::Solution {
        pub fn day1(input: &str) -> i32 {
            let elfs = input.split("\n");
            let mut curr_elf_food = 0;
            let mut most_food = 0;

            for food in elfs {
                if food == "" {
                    most_food = std::cmp::max(most_food, curr_elf_food);
                    curr_elf_food = 0;
                    continue;
                }

                let food_as_number = food.parse::<i32>().unwrap();
                curr_elf_food = curr_elf_food + food_as_number;
            }

            most_food = std::cmp::max(most_food, curr_elf_food);
            most_food
        }
        pub fn day1_part2(input: &str) -> i32 {
            let elfs = input.split("\n");
            let mut curr_elf_food = 0;
            let mut food_list = Vec::new();

            for food in elfs {
                if food == "" {
                    food_list.push(curr_elf_food);
                    curr_elf_food = 0;
                    continue;
                }

                let food_as_number = food.parse::<i32>().unwrap();
                curr_elf_food = curr_elf_food + food_as_number;
            }

            food_list.push(curr_elf_food);
            food_list.sort_by(|a, b| b.cmp(a));

            food_list[0] + food_list[1] + food_list[2]
        }
    }
}
