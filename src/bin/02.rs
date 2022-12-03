// Opponent
// A -> Rock
// B -> Paper
// C -> Scissors

// Me
// X -> Rock (1)
// Y -> Paper (2)
// Z -> Scissors (3)

// Score
// 0 -> Loss
// 3 -> Draw
// 6 -> Win

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    Some(lines.fold(0, |acc, line| {
        let enemy_action = line.chars().nth(0).unwrap();
        let my_action = line.chars().nth(2).unwrap();
        let result = match (enemy_action, my_action) {
            ('A', 'X') => 3 + 1,
            ('A', 'Y') => 6 + 2,
            ('A', 'Z') => 0 + 3,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 6 + 1,
            ('C', 'Y') => 0 + 2,
            ('C', 'Z') => 3 + 3,
            _ => panic!("Combination not possible!"),
        };

        acc + result
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    Some(lines.fold(0, |acc, line| {
        let enemy_action = line.chars().nth(0).unwrap();
        let expected_result = line.chars().nth(2).unwrap();

        // Result
        // X -> Lose
        // Y -> Draw
        // Z -> Win

        let result = match (enemy_action, expected_result) {
            ('A', 'X') => 0 + 3,
            ('A', 'Y') => 3 + 1,
            ('A', 'Z') => 6 + 2,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 0 + 2,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 6 + 1,
            _ => panic!("Combination not possible!"),
        };

        acc + result
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
