#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Elf {
    calories: u32
}

impl Elf {
    pub fn new() -> Self {
        Self { calories: 0 }
    }

    pub fn add(&mut self, value: u32) {
        self.calories += value;
    }
}

fn convert_to_elfs(input: &str) -> Vec<Elf> {
    let mut elves = Vec::new();
    elves.push(Elf::new());
    let lines = input.lines();
    lines.for_each(|line| {
        match line.parse::<u32>() {
            Ok(value) => elves.last_mut().unwrap().add(value),
            Err(_) => elves.push(Elf::new()),
        }
    });

    elves.sort();

    elves
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = convert_to_elfs(input);

    Some(elves.last().unwrap().calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = convert_to_elfs(input);
    let mut sum = 0;
    for i in 0..3 {
        sum += elves[elves.len() - 1 - i].calories;
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
