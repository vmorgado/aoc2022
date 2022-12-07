pub mod day7 {
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Pathing {
        previous_paths: Vec<String>,
        current_path: String,
    }

    impl crate::solution::advent_of_code_2022::Solution {
        pub fn day7(input: &str) -> usize {
            let mut dir_sizes: HashMap<String, usize> = HashMap::new();
            let lines: Vec<&str> = input.split("\n").collect();

            let mut state = Pathing {
                previous_paths: Vec::new(),
                current_path: "/".to_string(),
            };

            let mut i = 0;
            while i < lines.len() {
                let line = lines.get(i).unwrap();
                let parsed: Vec<&str> = line.split(" ").collect();
                let line_type = parsed.get(0).unwrap();

                match *line_type {
                    "$" => {
                        let cmd_type = parsed.get(1).unwrap();
                        match *cmd_type {
                            "cd" => {
                                let dir = parsed.get(2).unwrap();
                                match *dir {
                                    "/" => {
                                        state.previous_paths.clear();
                                        state.current_path = "/".to_string();
                                    }
                                    ".." => {
                                        let prev = state.previous_paths.pop().unwrap();
                                        state.current_path = prev;
                                    }
                                    _ => {
                                        state.previous_paths.push(state.current_path);
                                        state.current_path = dir.to_string();
                                    }
                                }
                            }
                            "ls" => {}
                            _ => {}
                        }
                    }
                    "dir" => {}
                    _ => {
                        let file_size = parsed.get(0).unwrap().parse::<usize>().unwrap();
                        let mut parent = "".to_owned();
                        let mut key = "".to_owned();

                        for path in &state.previous_paths {
                            if path != "/" {
                                key.push_str("/");
                                parent.push_str("/");
                                key.push_str(&path);
                                parent.push_str(&path);
                                let previous_parent =
                                    *dir_sizes.get(&parent.to_string()).unwrap_or(&0);
                                dir_sizes.insert(parent.to_string(), file_size + previous_parent);
                            }
                        }
                        key.push_str("/");
                        key.push_str(&state.current_path);

                        let previous_key = *dir_sizes.get(&key.to_string()).unwrap_or(&0);
                        dir_sizes.insert(key.to_string(), file_size + previous_key);
                    }
                }

                i = i + 1;
            }

            let filtered: HashMap<String, usize> = dir_sizes
                .into_iter()
                .filter(|(_k, v)| v <= &100000)
                .collect();
            let mut res = 0;

            for (_k, value) in filtered.into_iter() {
                res = res + value;
            }
            res
        }
        pub fn day7_part2(input: &str) -> usize {
            let mut dir_sizes: HashMap<String, usize> = HashMap::new();
            let lines: Vec<&str> = input.split("\n").collect();
            let mut total_disk_used = 0;

            let mut state = Pathing {
                previous_paths: Vec::new(),
                current_path: "/".to_string(),
            };

            let mut i = 0;
            while i < lines.len() {
                let line = lines.get(i).unwrap();
                let parsed: Vec<&str> = line.split(" ").collect();
                let line_type = parsed.get(0).unwrap();

                match *line_type {
                    "$" => {
                        let cmd_type = parsed.get(1).unwrap();
                        match *cmd_type {
                            "cd" => {
                                let dir = parsed.get(2).unwrap();
                                match *dir {
                                    "/" => {
                                        state.previous_paths.clear();
                                        state.current_path = "/".to_string();
                                    }
                                    ".." => {
                                        let prev = state.previous_paths.pop().unwrap();
                                        state.current_path = prev;
                                    }
                                    _ => {
                                        state.previous_paths.push(state.current_path);
                                        state.current_path = dir.to_string();
                                    }
                                }
                            }
                            "ls" => {}
                            _ => {}
                        }
                    }
                    "dir" => {}
                    _ => {
                        let file_size = parsed.get(0).unwrap().parse::<usize>().unwrap();
                        let mut parent = "".to_owned();
                        let mut key = "".to_owned();

                        for path in &state.previous_paths {
                            if path != "/" {
                                key.push_str("/");
                                parent.push_str("/");
                                key.push_str(&path);
                                parent.push_str(&path);
                                let previous_parent =
                                    *dir_sizes.get(&parent.to_string()).unwrap_or(&0);
                                dir_sizes.insert(parent.to_string(), file_size + previous_parent);
                            }
                        }
                        key.push_str("/");
                        key.push_str(&state.current_path);

                        let previous_key = *dir_sizes.get(&key.to_string()).unwrap_or(&0);
                        dir_sizes.insert(key.to_string(), file_size + previous_key);
                        total_disk_used = total_disk_used + file_size;
                    }
                }

                i = i + 1;
            }

            let max_disk_space = 70000000;
            let available_space = max_disk_space - total_disk_used;

            let for_update = 30000000;
            let need_to_delete = for_update - available_space;

            let filtered: HashMap<String, usize> = dir_sizes
                .into_iter()
                .filter(|(_k, v)| v >= &need_to_delete && _k != "//")
                .collect();

            let mut min = max_disk_space;
            let mut key = "";

            for item in filtered.iter() {
                let (_k, value) = item;

                if value < &min {
                    min = *value - need_to_delete;
                    key = _k;
                }
            }
            println!("{:?}", key);
            *filtered.get(key).unwrap()
        }
    }
}
