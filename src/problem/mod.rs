use std::fs;

struct Day {
    day: i32,
}
impl Day {
    fn new(day: i32) -> Self {
        Self { day }
    }
    fn read_data(&self) -> String {
        fs::read_to_string(format!("data/{}.txt", self.day)).expect(&format!(
            "Could not find 'data/{}.txt' data file.",
            self.day
        ))
    }
}

pub mod day1;
pub mod day2;
pub mod day3;
// pub mod day4;
// pub mod day5;
