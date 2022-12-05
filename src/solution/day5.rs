pub mod day5 {
    fn read_board(input: &str) -> (Vec<Vec<char>>, usize) {
        let mut board: Vec<Vec<char>> = Vec::<Vec<char>>::new();
        let lines: Vec<&str> = input.split("\n").collect();

        let mut board_complete = false;
        let mut index = 0;
        while !board_complete && index < lines.len() {
            let line = lines.get(index).unwrap();
            let mut char_index = 1;

            if line.chars().nth(char_index).unwrap() == '1' {
                board_complete = true;
                continue;
            }

            let mut i = 0;
            while char_index < line.len() {
                let c = line.chars().nth(char_index).unwrap();

                match board.get_mut(i) {
                    Some(col) => {
                        if c != ' ' {
                            col.push(c);
                        }
                    }
                    None => {
                        let mut line_vec: Vec<char> = Vec::<char>::new();
                        if c != ' ' {
                            line_vec.push(c);
                        }
                        board.push(line_vec);
                    }
                }

                char_index = char_index + 4;
                i = i + 1;
            }

            index = index + 1;
        }
        (board, index)
    }

    fn parse_moves(input: &str, index: usize) -> Vec<Vec<&str>> {
        let mut read_line = index + 2;
        let lines: Vec<&str> = input.split("\n").collect();
        let mut instructions: Vec<Vec<&str>> = Vec::<Vec<&str>>::new();
        while read_line < lines.len() {
            let line = lines.get(read_line).unwrap();
            let inst: Vec<&str> = line.split(" ").collect();
            instructions.push(inst);
            read_line = read_line + 1;
        }
        instructions
    }

    impl crate::solution::advent_of_code_2022::Solution {
        pub fn day5(input: &str) -> String {
            let (mut board, index) = read_board(input);
            let instructions = parse_moves(input, index);
            for inst in instructions {
                let mv = inst.get(1).unwrap().parse::<usize>().unwrap();
                let frm = inst.get(3).unwrap().parse::<usize>().unwrap() - 1;
                let to = inst.get(5).unwrap().parse::<usize>().unwrap() - 1;

                for _i in 0..mv {
                    let source = board.get_mut(frm).unwrap();
                    let to_move = source.remove(0);
                    let destination = board.get_mut(to).unwrap();
                    destination.insert(0, to_move);
                }
            }

            let mut result = "".to_owned();
            for col in board {
                result.push_str(&col.get(0).unwrap().to_string());
            }

            result.clone()
        }
        pub fn day5_part2(input: &str) -> String {
            let (mut board, index) = read_board(input);
            let instructions = parse_moves(input, index);
            for inst in instructions {
                let mv = inst.get(1).unwrap().parse::<usize>().unwrap();
                let frm = inst.get(3).unwrap().parse::<usize>().unwrap() - 1;
                let to = inst.get(5).unwrap().parse::<usize>().unwrap() - 1;

                for i in 0..mv {
                    let source = board.get_mut(frm).unwrap();
                    let to_move = source.remove(0);
                    let destination = board.get_mut(to).unwrap();
                    destination.insert(i, to_move);
                }
            }

            let mut result = "".to_owned();
            for col in board {
                result.push_str(&col.get(0).unwrap().to_string());
            }

            result.clone()
        }
    }
}
