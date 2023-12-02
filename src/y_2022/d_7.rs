use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: usize,
    is_directory: bool,
}

impl File {
    pub fn new_file(name: String, size: usize) -> Self {
        File {
            name,
            size,
            is_directory: false,
        }
    }

    pub fn new_dir(name: String) -> Self {
        File {
            name,
            size: 0,
            is_directory: true,
        }
    }
}

pub struct Day7 {
    inp: HashMap<String, Vec<File>>,
    sizes: HashMap<String, usize>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2022/7.txt";

impl Questions for Day7 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let mut tmp_dir = "".to_string();
        let mut files = vec![];

        let mut path = vec![];

        let mut map = HashMap::new();

        contents.lines().filter(|f| !f.is_empty()).for_each(|v| {
            // command if it starts with `$`
            if v.starts_with("$") {
                let v = v.replace("$", "");

                let res = v
                    .trim()
                    .split_whitespace()
                    .filter(|f| !f.is_empty())
                    .collect::<Vec<&str>>();

                if res[0] == "cd" {
                    if !tmp_dir.is_empty() {
                        map.insert(Day7::get_file_name(&path), files.clone());

                        // hit on the reset button
                        files.clear();
                    }

                    tmp_dir = "".to_string();

                    if res[1] == ".." {
                        path.pop().unwrap();
                        return;
                    }

                    tmp_dir = res[1].to_string();

                    path.push(res[1].to_string());
                }
            } else {
                if v.starts_with("dir") {
                    let res = v
                        .split_whitespace()
                        .filter(|f| !f.is_empty())
                        .collect::<Vec<&str>>();

                    let full_file_name =
                        format!("{}-{}", Day7::get_file_name(&path), res[1].to_string());

                    files.push(File::new_dir(full_file_name));
                } else {
                    let res = v
                        .split_whitespace()
                        .filter(|f| !f.is_empty())
                        .collect::<Vec<&str>>();

                    let size = res[0]
                        .parse::<usize>()
                        .expect("error parsing string to int");

                    files.push(File::new_file(res[1].to_string(), size));
                }
            }
        });

        if files.len() > 0 {
            map.insert(Day7::get_file_name(&path), files.clone());
        }

        self.inp = map.clone();

        for (dir_name, files) in &map {
            let mut size = 0;

            for file in files {
                if file.is_directory {
                    size += self.keep_track_of_sizes(&file.name);
                }

                size += file.size;
            }

            self.sizes.insert(dir_name.to_string(), size);
        }

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut total_sum = 0;

        self.sizes
            .clone()
            .into_iter()
            .filter(|f| f.1 < 100_000)
            .for_each(|f| {
                total_sum += f.1;
            });

        let ans = total_sum.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let total_space = 70_000_000;
        let required_space = 30_000_000;

        let root_size = self.sizes.get("/").unwrap().clone();
        let available_space = total_space - root_size;

        let mut v = vec![];

        for (_, value) in self.sizes.clone() {
            v.push(value);
        }

        // sorting it descending order
        v.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let mut smallest: usize = 100_000_000;

        for i in v {
            if i + available_space >= required_space {
                if smallest > i {
                    smallest = i;
                }
            }
        }

        let ans = smallest.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            inp: HashMap::new(),
            sizes: HashMap::new(),
        }
    }

    pub fn keep_track_of_sizes(&mut self, dir_name: &str) -> usize {
        let files = self.inp.get(dir_name).unwrap();
        let mut size = 0;

        for file in files.clone() {
            size += file.size;

            if file.is_directory {
                size += self.keep_track_of_sizes(&file.name);
            }
        }

        self.sizes.insert(dir_name.to_string(), size);
        size
    }

    pub fn get_file_name(path: &Vec<String>) -> String {
        path.join("-").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("95437");

        let mut day7 = Day7::new();

        day7.init("inputs/2022/7a.txt")
            .expect("error trying to init day7");

        let q1 = day7.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("24933642");

        let mut day7 = Day7::new();

        day7.init("inputs/2022/7a.txt")
            .expect("error trying to init day7");

        let q2 = day7.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
