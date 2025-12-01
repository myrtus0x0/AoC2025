fn part1(src_info: &str) -> u32 {
    let mut value: i16 = 50;
    let max_value = 100;
    let mut zeros = 0;

    let rotations: Vec<&str> = src_info.split("\n").collect();
    for action in rotations {
        if action.len() == 0 {
            break;
        }

        let direction_str = action.chars().next().unwrap();
        let direction: i16;
        match direction_str {
            'L' => direction = -1,
            'R' => direction = 1,
            _ => return 0,
        }

        let absolute_movement = action.get(1..).unwrap().parse::<i16>().unwrap();
        let movement = absolute_movement * direction;
        value += movement;
        value %= max_value;

        if value == 0 {
            zeros += 1;
        }
    }

    zeros
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part1(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part1(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
        assert_eq!(result, 3);
    }
}
