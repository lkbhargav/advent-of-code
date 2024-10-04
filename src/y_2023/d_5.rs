use std::{collections::HashSet, isize};

use crate::prelude::*;

#[derive(Debug)]
struct SourceToDestinationRangeMap {
    source: isize,
    destination: isize,
    range: isize,
}

impl SourceToDestinationRangeMap {
    fn fetch_from_range(&self, num: isize) -> Option<isize> {
        if num >= self.source && num <= self.source + self.range {
            return Some((self.destination - self.source) + num);
        }

        None
    }
}

#[derive(Debug, Default)]
struct PlantsInfo {
    seeds: Vec<isize>,
    seed_to_soil: Vec<SourceToDestinationRangeMap>,
    soil_to_fertilizer: Vec<SourceToDestinationRangeMap>,
    fertilizer_to_water: Vec<SourceToDestinationRangeMap>,
    water_to_light: Vec<SourceToDestinationRangeMap>,
    light_to_temperature: Vec<SourceToDestinationRangeMap>,
    temperature_to_humidity: Vec<SourceToDestinationRangeMap>,
    humidity_to_location: Vec<SourceToDestinationRangeMap>,
}

impl PlantsInfo {
    fn fetch_from_range(num: isize, lookup: &Vec<SourceToDestinationRangeMap>) -> isize {
        for v in lookup {
            match v.fetch_from_range(num) {
                None => continue,
                Some(d) => return d,
            };
        }

        num
    }

    fn fetch_ranges_from_ranges(
        ranges: &mut Vec<(isize, isize)>,
        lookup: &Vec<SourceToDestinationRangeMap>,
    ) {
        let mut resp = vec![];

        let mut visited = HashSet::new();

        while ranges.len() > 0 {
            let (start, end) = ranges.remove(0);

            if visited.contains(&(start, end)) {
                continue;
            }

            visited.insert((start, end));

            let resp_len_before = resp.len();

            for v in lookup {
                let diff = v.destination - v.source;
                let source_start = v.source;
                let source_end = v.source + v.range - 1; // inclusive..exclusive, so we have to do -1 at the end

                if end < v.source || start > source_end || resp.len() > resp_len_before {
                    continue;
                }

                // for records that are inside the ranges
                if start >= source_start && end <= source_end {
                    resp.push((start + diff, end + diff));
                } else if start >= source_start && end > source_end {
                    // for records that start inside and end outside the range
                    ranges.push((source_end + 1, end));

                    resp.push((start + diff, source_end + diff - 1));
                } else if start < source_start && end <= source_end {
                    // for records that start outside and end inside the range
                    ranges.push((start, source_start - 1));

                    resp.push((source_start + diff, end + diff));
                } else if start < source_start && end > source_end {
                    // for records that start and end outside the range covering the range itself
                    ranges.push((source_end + 1, end));
                    ranges.push((start, source_start - 1));

                    resp.push((source_start + diff, source_end + diff - 1));
                }
            }

            // for records that are outside the ranges
            if resp.len() == resp_len_before {
                resp.push((start, end));
            }
        }

        *ranges = resp;
    }
}

pub struct Day5 {
    data: PlantsInfo,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2023/5.txt";
// const TEST_FILE_NAME: &str = "inputs/2023/5a.txt";

impl Questions for Day5 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let data = contents.split("\n\n").map(|s| s).collect::<Vec<&str>>();

        let mut plants_info = PlantsInfo::default();

        // seeds is at index 0
        plants_info.seeds = data[0]
            .split_once(":")
            .expect("expect two parts to exist")
            .1
            .trim()
            .split(" ")
            .map(|n| n.trim().parse().expect("expected a number"))
            .collect::<Vec<isize>>();

        // seed to soil at index 1
        plants_info.seed_to_soil = Day5::parse_raw_data(data[1]);
        // soil to fertilizer at index 2
        plants_info.soil_to_fertilizer = Day5::parse_raw_data(data[2]);
        // fertilizer to water at index 3
        plants_info.fertilizer_to_water = Day5::parse_raw_data(data[3]);
        // water to light at index 4
        plants_info.water_to_light = Day5::parse_raw_data(data[4]);
        // light to temperature at index 5
        plants_info.light_to_temperature = Day5::parse_raw_data(data[5]);
        // temperature to humidity at index 6
        plants_info.temperature_to_humidity = Day5::parse_raw_data(data[6]);
        // humidity to location at index 7
        plants_info.humidity_to_location = Day5::parse_raw_data(data[7]);

        self.data = plants_info;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut least_location = isize::MAX;

        for seed in &self.data.seeds {
            let soil = PlantsInfo::fetch_from_range(*seed, &self.data.seed_to_soil);
            let fertilizer = PlantsInfo::fetch_from_range(soil, &self.data.soil_to_fertilizer);
            let water = PlantsInfo::fetch_from_range(fertilizer, &self.data.fertilizer_to_water);
            let light = PlantsInfo::fetch_from_range(water, &self.data.water_to_light);
            let temperature = PlantsInfo::fetch_from_range(light, &self.data.light_to_temperature);
            let humidity =
                PlantsInfo::fetch_from_range(temperature, &self.data.temperature_to_humidity);
            let location = PlantsInfo::fetch_from_range(humidity, &self.data.humidity_to_location);

            if location < least_location {
                least_location = location;
            }
        }

        let ans = least_location.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut least_location = isize::MAX;
        let mut iter = self.data.seeds.chunks(2);

        let mut ranges = vec![];

        loop {
            let set = match iter.next() {
                None => break,
                Some(v) => v,
            };

            ranges.push((set[0], set[0] + set[1] - 1));
        }

        PlantsInfo::fetch_ranges_from_ranges(&mut ranges, &self.data.seed_to_soil);
        PlantsInfo::fetch_ranges_from_ranges(&mut ranges, &self.data.soil_to_fertilizer);
        PlantsInfo::fetch_ranges_from_ranges(&mut ranges, &self.data.fertilizer_to_water);
        PlantsInfo::fetch_ranges_from_ranges(&mut ranges, &self.data.water_to_light);
        PlantsInfo::fetch_ranges_from_ranges(&mut ranges, &self.data.light_to_temperature);
        PlantsInfo::fetch_ranges_from_ranges(&mut ranges, &self.data.temperature_to_humidity);
        PlantsInfo::fetch_ranges_from_ranges(&mut ranges, &self.data.humidity_to_location);

        for (start, _) in ranges {
            if least_location > start {
                least_location = start;
            }
        }

        let ans = least_location.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            data: Default::default(),
        }
    }

    fn parse_raw_data(to_parse: &str) -> Vec<SourceToDestinationRangeMap> {
        to_parse
            .lines()
            .filter(|l| !l.contains(":"))
            .map(|l| {
                l.trim()
                    .split(" ")
                    .map(|n| n.trim().parse().expect("expected a number"))
                    .collect::<Vec<isize>>()
            })
            .map(|d| SourceToDestinationRangeMap {
                source: d[1],
                destination: d[0],
                range: d[2],
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("35");

        let mut day5 = Day5::new();

        day5.init("inputs/2023/5a.txt")
            .expect("error trying to init day5");

        let q1 = day5.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("46");

        let mut day5 = Day5::new();

        day5.init("inputs/2023/5a.txt")
            .expect("error trying to init day5");

        let q2 = day5.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2_works_2() {
        let expected_q2 = String::from("37384986");

        let mut day5 = Day5::new();

        day5.init("inputs/2023/5b.txt")
            .expect("error trying to init day5");

        let q2 = day5.question_two().unwrap();

        assert_eq!(expected_q2, q2, "Amish's input");
    }

    #[test]
    fn q2_works_3() {
        let expected_q2 = String::from("57451709");

        let mut day5 = Day5::new();

        day5.init("inputs/2023/5c.txt")
            .expect("error trying to init day5");

        let q2 = day5.question_two().unwrap();

        assert_eq!(expected_q2, q2, "Testing my input");
    }
}
