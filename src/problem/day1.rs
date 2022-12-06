use super::Day;

trait Problem {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

impl Problem for Day {
    fn part_1(&self) -> String {
        let data = self.read_data();
        let mut calories: Vec<i32> = vec![];
        let mut total_held = 0;
        for amount_str in data.split("\n") {
            match amount_str.parse::<i32>() {
                Ok(value) => {
                    total_held += value;
                }
                Err(_) => {
                    calories.push(total_held);
                    total_held = 0;
                }
            }
        }
        calories.iter().max().unwrap().to_string()
    }
    fn part_2(&self) -> String {
        let data = self.read_data();
        let mut calories: Vec<i32> = vec![];
        let mut total_held = 0;
        for amount_str in data.split("\n") {
            match amount_str.parse::<i32>() {
                Ok(value) => {
                    total_held += value;
                }
                Err(_) => {
                    calories.push(total_held);
                    total_held = 0;
                }
            }
        }
        calories.sort();
        calories[calories.len() - 3..]
            .into_iter()
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let problem = Day::new(
            (std::module_path!().split("::").collect::<Vec<_>>()[2])[3..]
                .parse::<i32>()
                .unwrap(),
        );
        assert_eq!(problem.part_1(), "66487");
    }

    #[test]
    fn part_2() {
        let problem = Day::new(
            (std::module_path!().split("::").collect::<Vec<_>>()[2])[3..]
                .parse::<i32>()
                .unwrap(),
        );
        assert_eq!(problem.part_2(), "197301");
    }
}
