pub fn part_one(input: &str) -> Option<u32> {
    let mut last = 0;
    let mut total: i32 = -1;
    for elem in input.lines() {
        let num = elem.parse::<i32>().unwrap();
        if num > last {
            total += 1;
        }
        last = num;
    }
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut last_peak = 0;
    let mut last_window = [0; 3];
    let mut total: i32 = -3;
    for (i, elem) in input.lines().enumerate() {
        let num = elem.parse::<i32>().unwrap();
        last_window[i % 3] = num;
        let curr_peak = last_window.iter().sum();
        if curr_peak > last_peak {
            total += 1;
        }
        last_peak = curr_peak;
    }
    Some(total as u32)
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
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
