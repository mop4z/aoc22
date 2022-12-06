use super::Day;

trait Problem {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

enum Move {
    Rock,
    Paper,
    Scissors,
}
impl Move {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn play(&self, opponent: &Self) -> i32 {
        self.score()
            + match self {
                Self::Rock => match opponent {
                    Self::Rock => Outcome::Draw,
                    Self::Paper => Outcome::Loss,
                    Self::Scissors => Outcome::Win,
                },
                Self::Paper => match opponent {
                    Self::Rock => Outcome::Win,
                    Self::Paper => Outcome::Draw,
                    Self::Scissors => Outcome::Loss,
                },
                Self::Scissors => match opponent {
                    Self::Rock => Outcome::Loss,
                    Self::Paper => Outcome::Win,
                    Self::Scissors => Outcome::Draw,
                },
            }
            .score()
    }
}
enum Outcome {
    Win,
    Loss,
    Draw,
}
impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}

impl Problem for Day {
    fn part_1(&self) -> String {
        self.read_data()
            .lines()
            .map(|input| {
                let inputs: Vec<&str> = input.split(" ").collect();
                let opponent = match inputs[0] {
                    "A" => Move::Rock,
                    "B" => Move::Paper,
                    "C" => Move::Scissors,
                    _ => Move::Rock,
                };
                let me = match inputs[1] {
                    "X" => Move::Rock,
                    "Y" => Move::Paper,
                    "Z" => Move::Scissors,
                    _ => Move::Rock,
                };
                me.play(&opponent)
            })
            .sum::<i32>()
            .to_string()
    }
    fn part_2(&self) -> String {
        self.read_data()
            .lines()
            .map(|input| {
                let inputs: Vec<&str> = input.split(" ").collect();
                let opponent = match inputs[0] {
                    "A" => Move::Rock,
                    "B" => Move::Paper,
                    "C" => Move::Scissors,
                    _ => Move::Rock,
                };
                let me = match inputs[1] {
                    "X" => match opponent {
                        Move::Rock => &Move::Scissors,
                        Move::Paper => &Move::Rock,
                        Move::Scissors => &Move::Paper,
                    },
                    "Y" => &opponent,
                    "Z" => match opponent {
                        Move::Rock => &Move::Paper,
                        Move::Paper => &Move::Scissors,
                        Move::Scissors => &Move::Rock,
                    },
                    _ => &Move::Rock,
                };
                me.play(&opponent)
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
        let problem = Day::new(2);
        assert_eq!(problem.part_1(), "12855");
    }

    #[test]
    fn part_2() {
        let problem = Day::new(2);
        assert_eq!(problem.part_2(), "13726");
    }
}
