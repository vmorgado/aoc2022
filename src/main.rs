mod solution;
use crate::solution::advent_of_code_2022::Solution;

fn main() {
    // let result = Solution::day1_part2(
    // );
    // println!("{}", result);
}

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
}
