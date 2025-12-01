fn part2(src_info: &str) -> u32 {
    let mut value: i16 = 50;
    let max_value = 100;
    let mut zeros = 0;

    let rotations: Vec<&str> = src_info.split("\n").collect();
    for action in rotations {
        if action.len() == 0 {
            break;
        }

        let direction_str = action.chars().next().unwrap();
        let direction = match direction_str {
            'L' => -1,
            'R' => 1,
            _ => return 0,
        };

        let absolute_movement = action.get(1..).unwrap().parse::<i16>().unwrap();
        let old_val = value;

        // account for full rotations
        zeros += absolute_movement / 100;

        // get remainder
        let move_remaining = absolute_movement % 100;
        let movement = move_remaining * direction;
        value += movement;

        // did we exceed bounds
        if value >= 100 || value <= 0 {
            // put back in range
            value = value.rem_euclid(max_value);

            if value != old_val && old_val != 0 {
                zeros += 1;
            }
        };
    }

    zeros.try_into().unwrap()
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part2(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2(
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
        assert_eq!(result, 6);
    }
}
