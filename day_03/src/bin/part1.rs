fn part1(src_info: &str) -> u32 {
    let mut highest_power = 0;

    for entry in src_info.split("\n") {
        let power = return_highest_power(entry);
        println!("returning: {}", power);
        highest_power += power;
    }

    highest_power
}

fn return_highest_power(bank_str: &str) -> u32 {
    let mut base1 = 0;
    let digits: Vec<u32> = bank_str.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() <= 1 {
        return 0;
    }

    let base10 = digits[..digits.len() - 1]
        .iter()
        .max()
        .copied()
        .unwrap_or(0);

    let mut can_start = false;
    for bank_num in digits {
        if can_start && bank_num > base1 {
            base1 = bank_num;
        }

        if bank_num == base10 {
            can_start = true;
        }
    }

    (base10 * 10) + base1
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
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(result, 357);
    }
}
