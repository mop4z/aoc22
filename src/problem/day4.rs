use super::Day;

trait Problem {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

impl Problem for Day {
    fn part_1(&self) -> String {
        self.read_data()
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|assignments_str| {
                        let range = assignments_str
                            .split('-')
                            .map(|id| id.parse::<i32>().unwrap())
                            .collect::<Vec<_>>();
                        range[0]..=range[1]
                    })
                    .collect::<Vec<_>>()
            })
            .map(|ranges| {
                match (ranges[0].contains(ranges[1].start()) & ranges[0].contains(ranges[1].end()))
                    | (ranges[1].contains(ranges[0].start()) & ranges[1].contains(ranges[0].end()))
                {
                    true => 1,
                    false => 0,
                }
            })
            .sum::<i32>()
            .to_string()
    }
    fn part_2(&self) -> String {
        self.read_data()
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|assignments_str| {
                        let range = assignments_str
                            .split('-')
                            .map(|id| id.parse::<i32>().unwrap())
                            .collect::<Vec<_>>();
                        range[0]..=range[1]
                    })
                    .collect::<Vec<_>>()
            })
            .map(|ranges| {
                match (ranges[0].contains(ranges[1].start()) | ranges[0].contains(ranges[1].end()))
                    | (ranges[1].contains(ranges[0].start()) | ranges[1].contains(ranges[0].end()))
                {
                    true => 1,
                    false => 0,
                }
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
        let problem = Day::new(4);
        assert_eq!(problem.part_1(), "526");
    }

    #[test]
    fn part_2() {
        let problem = Day::new(4);
        assert_eq!(problem.part_2(), "886");
    }
}
