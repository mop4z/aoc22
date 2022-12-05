// Day 2
#[derive(Clone)]
enum RPC {
    Rock,
    Paper,
    Scissors,
}
fn battle(player_1: &RPC, player_2: &RPC) -> i32 {
    match player_1 {
        RPC::Rock => match player_2 {
            RPC::Rock => 1 + 3,
            RPC::Paper => 1 + 0,
            RPC::Scissors => 1 + 6,
        },
        RPC::Paper => match player_2 {
            RPC::Rock => 2 + 6,
            RPC::Paper => 2 + 3,
            RPC::Scissors => 2 + 0,
        },
        RPC::Scissors => match player_2 {
            RPC::Rock => 3 + 0,
            RPC::Paper => 3 + 6,
            RPC::Scissors => 3 + 3,
        },
    }
}
// Day 2

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[test]
    fn day_1() {
        let data = std::fs::read_to_string("data/day1.txt").unwrap();
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
        println!("{:?}", &calories[calories.len() - 1],);
        println!(
            "{:?}",
            &calories[calories.len() - 3..].into_iter().sum::<i32>()
        )
    }

    #[test]
    fn day_2() {
        let data = std::fs::read_to_string("data/day2.txt").unwrap();
        let mut score_1 = 0;
        let mut score_2 = 0;
        for input in data.lines() {
            let inputs: Vec<&str> = input.split(" ").collect();
            let opponent = match inputs[0] {
                "A" => RPC::Rock,
                "B" => RPC::Paper,
                "C" => RPC::Scissors,
                _ => RPC::Rock,
            };
            let me_1 = match inputs[1] {
                "X" => RPC::Rock,
                "Y" => RPC::Paper,
                "Z" => RPC::Scissors,
                _ => RPC::Rock,
            };
            let me_2 = match inputs[1] {
                "X" => match opponent {
                    RPC::Rock => RPC::Scissors,
                    RPC::Paper => RPC::Rock,
                    RPC::Scissors => RPC::Paper,
                },
                "Y" => opponent.clone(),
                "Z" => match opponent {
                    RPC::Rock => RPC::Paper,
                    RPC::Paper => RPC::Scissors,
                    RPC::Scissors => RPC::Rock,
                },
                _ => RPC::Rock,
            };
            score_1 += battle(&me_1, &opponent);
            score_2 += battle(&me_2, &opponent);
        }
        println!("{score_1}\n{score_2}")
    }

    #[test]
    fn day_3() {
        let data = std::fs::read_to_string("data/day3.txt").unwrap();
        let mut priorities: HashMap<char, usize> = HashMap::new();
        for (c, i) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .into_iter()
            .enumerate()
            .map(|(i, c)| (c, i + 1))
        {
            priorities.insert(c, i);
        }
        let mut priorities_sum = 0;
        for items in data.lines() {
            let set_1: HashSet<char> =
                HashSet::from_iter(items[..items.len() / 2].chars().into_iter());
            let set_2: HashSet<char> =
                HashSet::from_iter(items[items.len() / 2..].chars().into_iter());
            let duplicate_letter = set_1.intersection(&set_2).into_iter().next().unwrap();
            priorities_sum += priorities[duplicate_letter];
        }
        println!("{priorities_sum}");

        priorities_sum = 0;
        for items in data.lines().collect::<Vec<_>>().chunks(3) {
            let set_1: HashSet<char> = HashSet::from_iter(items[0].chars().into_iter());
            let set_2: HashSet<char> = HashSet::from_iter(items[1].chars().into_iter());
            let set_3: HashSet<char> = HashSet::from_iter(items[2].chars().into_iter());
            let int_1: HashSet<&char> = HashSet::from_iter(set_1.intersection(&set_2));
            let int_2: HashSet<&char> = HashSet::from_iter(set_2.intersection(&set_3));
            let duplicate_letter = int_1.intersection(&int_2).into_iter().next().unwrap();
            priorities_sum += priorities[duplicate_letter];
        }
        println!("{priorities_sum}");
    }

    #[test]
    fn day_4() {
        let data = std::fs::read_to_string("data/day4.txt").unwrap();
        let cleaned_data = data.lines().map(|line| {
            line.split(',')
                .map(|assignments_str| {
                    assignments_str
                        .split('-')
                        .map(|id| id.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        });
        let answer_1 = cleaned_data
            .clone()
            .map(|assignments| {
                if (assignments[0][0] <= assignments[1][0])
                    & (assignments[0][1] >= assignments[1][1])
                {
                    1
                } else if (assignments[0][0] >= assignments[1][0])
                    & (assignments[0][1] <= assignments[1][1])
                {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>();
        println!("{answer_1}");
        let answer_2 = cleaned_data
            .clone()
            .map(|assignments| {
                if (assignments[0][0] <= assignments[1][0])
                    & (assignments[0][1] >= assignments[1][0])
                {
                    1
                } else if (assignments[0][0] <= assignments[1][1])
                    & (assignments[0][1] >= assignments[1][1])
                {
                    1
                } else if (assignments[1][0] <= assignments[0][0])
                    & (assignments[1][1] >= assignments[0][0])
                {
                    1
                } else if (assignments[1][0] <= assignments[0][1])
                    & (assignments[1][1] >= assignments[0][1])
                {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>();
        println!("{answer_2}")
    }

    #[test]
    fn day_5() {
        let data = std::fs::read_to_string("data/day5.txt").unwrap();
        let data = data.split("\n").collect::<Vec<&str>>();
        let data = data.split(|row| row.to_owned() == "").collect::<Vec<_>>();
        let box_data = &mut data[0].iter().map(|row| row.as_bytes()).collect::<Vec<_>>();
        box_data.reverse();
        let mut initial_columns: Vec<Vec<u8>> = vec![];
        for (idx, byte) in box_data[0].iter().enumerate() {
            if byte != &32u8 {
                let mut column = vec![byte.to_owned()];
                for row in &mut box_data[1..] {
                    column.push(row[idx]);
                }
                initial_columns.push(column);
            }
        }
        let initial_columns = initial_columns
            .iter()
            .map(|column| {
                std::str::from_utf8(column)
                    .unwrap()
                    .trim()
                    .split("")
                    .filter(|c| *c != "")
                    .map(|c| c.to_owned())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut columns_1 = initial_columns.clone();
        let mut columns_2 = initial_columns.clone();
        let instructions = &data[1]
            .iter()
            .map(|instruction| {
                instruction
                    .split(" ")
                    .filter_map(|value| value.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<_>>();
        for instruction in instructions {
            for _ in 0..instruction[0] {
                // Part 1
                let from_column = &mut columns_1[instruction[1] - 1];
                let item = from_column.pop().unwrap();
                let to_column = &mut columns_1[instruction[2] - 1];
                to_column.push(item);
            }
            // // Part 2
            let from_column = columns_2[instruction[1] - 1].clone();
            let items = (&from_column[from_column.len() - instruction[0]..]).to_vec();
            columns_2[instruction[1] - 1] =
                (&from_column[..from_column.len() - instruction[0]]).to_vec();
            columns_2[instruction[2] - 1] = [columns_2[instruction[2] - 1].clone(), items].concat();
        }
        println!(
            "{:?}",
            columns_1
                .iter()
                .map(|column| column.last().unwrap().clone())
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<String>()
        );
        println!(
            "{:?}",
            columns_2
                .iter()
                .map(|column| column.last().unwrap().clone())
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<String>()
        );
    }
}
