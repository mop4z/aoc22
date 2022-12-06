use std::collections::HashSet;

use super::Day;

trait Problem {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

fn get_priority(&letter: &char) -> i32 {
    match letter.is_lowercase() {
        true => 1 + letter as i32 - 'a' as i32,
        false => 2 + 'z' as i32 - 'a' as i32 + letter as i32 - 'A' as i32,
    }
}

impl Problem for Day {
    fn part_1(&self) -> String {
        self.read_data()
            .lines()
            .map(|items| {
                get_priority(
                    items[..items.len() / 2]
                        .chars()
                        .collect::<HashSet<char>>()
                        .intersection(&items[items.len() / 2..].chars().collect::<HashSet<char>>())
                        .last()
                        .unwrap(),
                )
            })
            .sum::<i32>()
            .to_string()
    }
    fn part_2(&self) -> String {
        self.read_data()
            .lines()
            .map(|items| items.chars().collect::<HashSet<char>>())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|sets| {
                get_priority(
                    sets.iter()
                        .fold(sets[0].clone(), |set_1, set_2| {
                            set_1
                                .intersection(&set_2)
                                .map(|int| int.to_owned())
                                .collect::<HashSet<char>>()
                        })
                        .iter()
                        .last()
                        .unwrap(),
                )
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let problem = Day::new(3);
        assert_eq!(problem.part_1(), "8109");
    }

    #[test]
    fn part_2() {
        let problem = Day::new(3);
        assert_eq!(problem.part_2(), "2738");
    }
}
