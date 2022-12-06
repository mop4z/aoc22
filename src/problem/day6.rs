use std::collections::HashSet;

use super::Day;

trait Problem {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

impl Problem for Day {
    fn part_1(&self) -> String {
        (self
            .read_data()
            .chars()
            .collect::<Vec<_>>()
            .windows(4)
            .map(|input| {
                input
                    .iter()
                    .map(|i| i.to_owned())
                    .collect::<HashSet<char>>()
                    .len()
                    == 4
            })
            .take_while(|x| x == &false)
            .count()
            + 4)
        .to_string()
    }
    fn part_2(&self) -> String {
        let data = self.read_data();
        todo!()
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
        assert_eq!(problem.part_1(), "3");
    }

    #[test]
    fn part_2() {
        let problem = Day::new(
            (std::module_path!().split("::").collect::<Vec<_>>()[2])[3..]
                .parse::<i32>()
                .unwrap(),
        );
        assert_eq!(problem.part_2(), "");
    }
}
