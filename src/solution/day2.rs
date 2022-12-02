pub mod day2 {
    use std::collections::HashMap;

    pub fn calculate_score(elf: &str, me: &str) -> i32 {
        let points_win: i32 = 6;
        let points_draw: i32 = 3;

        let move_points: HashMap<&'static str, i32> = HashMap::from([("R", 1), ("P", 2), ("S", 3)]);

        let mut score: i32 = 0;

        let elf_points = move_points.get(elf).unwrap();
        let my_points = move_points.get(me).unwrap();
        score = score + my_points;

        if my_points % 3 == elf_points - 1 {
            return score;
        }

        if elf == me {
            return score + points_draw;
        }
        score + points_win
    }

    impl crate::solution::advent_of_code_2022::Solution {
        pub fn day2(input: &str) -> i32 {
            let elf_moves: HashMap<&'static str, &'static str> =
                HashMap::from([("A", "R"), ("B", "P"), ("C", "S")]);

            let my_moves: HashMap<&'static str, &'static str> =
                HashMap::from([("X", "R"), ("Y", "P"), ("Z", "S")]);

            let games = input.split("\n");
            let mut total_score = 0;
            for game in games {
                let plays = game.split(" ");
                let elf_play = plays.clone().nth(0).unwrap();
                let me_play = plays.clone().nth(1).unwrap();

                let elf = elf_moves.get(elf_play).unwrap();
                let me = my_moves.get(me_play).unwrap();

                total_score = total_score + calculate_score(elf, me);
            }

            total_score
        }
        pub fn day2_part2(input: &str) -> i32 {
            let elf_moves: HashMap<&'static str, &'static str> =
                HashMap::from([("A", "R"), ("B", "P"), ("C", "S")]);

            let win_moves: HashMap<&'static str, &'static str> =
                HashMap::from([("A", "P"), ("B", "S"), ("C", "R")]);

            let lose_moves: HashMap<&'static str, &'static str> =
                HashMap::from([("A", "S"), ("B", "R"), ("C", "P")]);

            let games = input.split("\n");
            let mut total_score = 0;
            for game in games {
                let plays = game.split(" ");
                let elf_play = plays.clone().nth(0).unwrap();
                let desired_result = plays.clone().nth(1).unwrap();

                let elf = elf_moves.get(elf_play).unwrap();

                let me = match desired_result {
                    "X" => lose_moves.get(elf_play).unwrap(),
                    "Y" => elf_moves.get(elf_play).unwrap(),
                    "Z" => win_moves.get(elf_play).unwrap(),
                    _ => panic!("not possible"),
                };

                total_score = total_score + calculate_score(elf, me);
            }

            total_score
        }
    }
}
