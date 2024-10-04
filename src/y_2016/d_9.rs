use crate::prelude::*;

pub struct Day9 {
    data: String,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2016/9.txt";
// const TEST_FILE_NAME: &str = "inputs/2016/9a.txt";

impl Questions for Day9 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.data = contents.trim().to_string();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let count = Day9::decompressed_count_q1(&self.data);

        let ans = count.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let count = Day9::decompressed_count(&self.data);

        let ans = count.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {
            data: String::new(),
        }
    }

    fn decompressed_count_q1(inp: &str) -> usize {
        let start_part = match inp.find("(") {
            None => return inp.len(),
            Some(n) => n,
        };
        let end_part = match inp.find(")") {
            None => return inp.len(),
            Some(n) => n,
        };

        let range = inp[start_part + 1..end_part].split_once("x").unwrap();

        let (len, times) = (
            range.0.parse::<usize>().unwrap(),
            range.1.parse::<usize>().unwrap(),
        );

        return inp[..start_part].len()
            + inp[end_part + 1..end_part + len + 1].len() * times
            + Day9::decompressed_count_q1(&inp[end_part + 1 + len..]);
    }

    fn decompressed_count(inp: &str) -> usize {
        let start_part = match inp.find("(") {
            None => return inp.len(),
            Some(n) => n,
        };
        let end_part = match inp.find(")") {
            None => return inp.len(),
            Some(n) => n,
        };

        let range = inp[start_part + 1..end_part].split_once("x").unwrap();

        let (len, times) = (
            range.0.parse::<usize>().unwrap(),
            range.1.parse::<usize>().unwrap(),
        );

        return inp[..start_part].len()
            + Day9::decompressed_count(&inp[end_part + 1..end_part + len + 1]) * times
            + Day9::decompressed_count(&inp[end_part + 1 + len..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("18");

        let mut day9 = Day9::new();

        day9.init("inputs/2016/9a.txt")
            .expect("error trying to init day9");

        let q1 = day9.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("20");

        let mut day9 = Day9::new();

        day9.init("inputs/2016/9a.txt")
            .expect("error trying to init day9");

        let q2 = day9.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2_works_2() {
        let expected_q2 = String::from("445");

        let mut day9 = Day9::new();

        day9.init("inputs/2016/9b.txt")
            .expect("error trying to init day9");

        let q2 = day9.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
