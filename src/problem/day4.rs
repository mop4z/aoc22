use super::Day;

trait Problem {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

impl Problem for Day {
    fn part_1(&self) -> String {
        let data = self.read_data();
        let x = data
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|assignments_str| {
                        assignments_str
                            .split('-')
                            .map(|id| id.parse::<i32>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        todo!()
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
        let problem = Day::new(4);
        problem.part_1();
        // assert_eq!(problem.part_1(), "");
    }

    #[test]
    fn part_2() {
        let problem = Day::new(4);
        assert_eq!(problem.part_2(), "");
    }
}
