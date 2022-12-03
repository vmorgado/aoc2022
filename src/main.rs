mod solution;
// use crate::solution::advent_of_code_2022::Solution;

fn main() {}

#[cfg(test)]
mod tests {

    use crate::solution::advent_of_code_2022::Solution;
    #[test]
    fn day1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = Solution::day1(&input);
        let result2 = Solution::day1_part2(&input);
        assert_eq!(result, 24000);
        assert_eq!(result2, 45000);
    }

    #[test]
    fn day2() {
        let input = "A Y
B X
C Z";

        let result = Solution::day2(&input);
        let result2 = Solution::day2_part2(&input);
        assert_eq!(result, 15);
        assert_eq!(result2, 12);
    }

    #[test]
    fn day3() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let result = Solution::day3(&input);
        let result2 = Solution::day3_part2(&input);
        assert_eq!(result, 157);
        assert_eq!(result2, 70);
    }
}
