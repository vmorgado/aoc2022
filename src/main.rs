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

    #[test]
    fn day4() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let result = Solution::day4(&input);
        let result2 = Solution::day4_part2(&input);
        assert_eq!(result, 2);
        assert_eq!(result2, 4);
    }

    #[test]
    fn day5() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let result = Solution::day5(&input);
        let result2 = Solution::day5_part2(&input);
        assert_eq!(result, "CMZ");
        assert_eq!(result2, "MCD");
    }
}
