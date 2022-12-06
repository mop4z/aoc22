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
            .enumerate()
            .map(|(idx, input)| {
                (
                    idx,
                    input
                        .iter()
                        .map(|i| i.to_owned())
                        .collect::<HashSet<char>>()
                        .len()
                        == 4,
                )
            })
            .find(|(_, input)| input == &true)
            .unwrap()
            .0
            + 4)
        .to_string()
    }
    fn part_2(&self) -> String {
        (self
            .read_data()
            .chars()
            .collect::<Vec<_>>()
            .windows(14)
            .enumerate()
            .map(|(idx, input)| {
                (
                    idx,
                    input
                        .iter()
                        .map(|i| i.to_owned())
                        .collect::<HashSet<char>>()
                        .len()
                        == 14,
                )
            })
            .find(|(_, input)| input == &true)
            .unwrap()
            .0
            + 14)
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
        assert_eq!(problem.part_1(), "1848");
    }

    #[test]
    fn part_2() {
        let problem = Day::new(
            (std::module_path!().split("::").collect::<Vec<_>>()[2])[3..]
                .parse::<i32>()
                .unwrap(),
        );
        assert_eq!(problem.part_2(), "2308");
    }
}
