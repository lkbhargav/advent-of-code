use crate::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Results {
    locations: Vec<String>,
}

impl Results {
    pub fn new(locations: Vec<String>) -> Self {
        Self { locations }
    }

    pub fn simpilfy(&self, distance_references: &HashMap<(String, String), usize>) {
        let mut locations = String::new();
        let mut total_distance = 0;

        for (idx, location) in self.locations.iter().enumerate() {
            if idx + 1 == self.locations.len() {
                locations = format!("{locations}{location}");
                continue;
            }

            total_distance += distance_references
                .get(&(location.to_owned(), self.locations[idx + 1].to_owned()))
                .expect("expected distance to be found");

            locations = format!("{locations}{location} -> ");
        }

        println!("{locations} => {total_distance}");
    }
}

pub struct Day9 {
    data: HashMap<String, Vec<String>>,
    distances_reference: HashMap<(String, String), usize>,
    number_of_cities: usize,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2015/9.txt";

impl Questions for Day9 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let mut unique_cities = HashSet::new();

        let regex_parser =
            RegexParser::new(r#"(?P<citya>\w+) to (?P<cityb>\w+) = (?P<distance>\d+)"#);

        contents.lines().filter(|f| !f.is_empty()).for_each(|v| {
            let captures = regex_parser.parse(v);

            let citya = captures.get_name("citya");
            let cityb = captures.get_name("cityb");
            let distance = captures.get_name_usize("distance") as usize;

            unique_cities.insert(citya.clone());
            unique_cities.insert(cityb.clone());

            self.get_ready(
                citya.clone().into_owned(),
                cityb.clone().into_owned(),
                distance,
            );
            self.get_ready(
                cityb.into_owned().clone(),
                citya.into_owned().clone(),
                distance,
            );
        });

        self.number_of_cities = unique_cities.len() as usize;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut min = 100_000_000;

        for result in self.run() {
            let mut distance = 0;
            for (idx, location) in result.locations.iter().enumerate() {
                if idx + 1 == result.locations.len() {
                    break;
                }
                distance += *self
                    .distances_reference
                    .get(&(location.clone(), result.locations[idx + 1].clone()))
                    .unwrap();
            }

            if distance < min {
                min = distance;
            }
        }

        let ans = min.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut max = 0;

        for result in self.run() {
            let mut distance = 0;
            for (idx, location) in result.locations.iter().enumerate() {
                if idx + 1 == result.locations.len() {
                    break;
                }
                distance += *self
                    .distances_reference
                    .get(&(location.clone(), result.locations[idx + 1].clone()))
                    .unwrap();
            }

            if distance > max {
                max = distance;
            }
        }

        let ans = max.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {
            data: HashMap::new(),
            distances_reference: HashMap::new(),
            number_of_cities: 0,
        }
    }

    pub fn run(&mut self) -> Vec<Results> {
        let mut results = vec![];

        for (city_a, destinations) in self.data.clone() {
            let mut locations: Vec<String> = vec![];

            locations.push(city_a.clone());

            self.navigate(destinations, &mut locations, &mut results);
        }

        results
    }

    pub fn get_ready(&mut self, city_a: String, city_b: String, distance: usize) {
        let key = (city_a.clone(), city_b.clone());

        if !self.distances_reference.contains_key(&key) {
            self.distances_reference.insert(key, distance);
        }

        match self.data.clone().get_mut(&city_a) {
            Some(v) => {
                if !v.contains(&city_b) {
                    v.push(city_b);
                    self.data.insert(city_a, v.clone());
                }
            }
            None => {
                self.data.insert(city_a, vec![city_b]);
            }
        }
    }

    pub fn navigate(
        &mut self,
        destinations: Vec<String>,
        locations: &mut Vec<String>,
        results: &mut Vec<Results>,
    ) {
        for location in destinations {
            if locations.contains(&location) {
                if locations.len() as usize == self.number_of_cities {
                    let result = Results::new(locations.clone());
                    results.push(result);
                    break;
                }
                continue;
            }

            locations.push(location.clone());

            match self.data.get(&location) {
                Some(d) => {
                    self.navigate(d.clone(), locations, results);
                }
                None => (),
            };
        }
        locations.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("605");

        let mut day9 = Day9::new();

        day9.init("inputs/2015/9a.txt")
            .expect("error trying to init day9");

        let q1 = day9.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("982");

        let mut day9 = Day9::new();

        day9.init("inputs/2015/9a.txt")
            .expect("error trying to init day9");

        let q2 = day9.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
