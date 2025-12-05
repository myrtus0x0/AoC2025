fn part2(src_info: &str) -> u64 {
    let mut highest_power = 0;

    for entry in src_info.split("\n") {
        let power = return_highest_power(entry);
        println!("returning: {}", power);
        highest_power += power;
    }

    highest_power
}

fn return_highest_power(bank_str: &str) -> u64 {
    let mut largest_power: u64 = 0;
    let mut needed_digits = 12;
    let mut base: u64 = 10_u64.pow(needed_digits - 1);
    let mut start_index = 0;
    let string_size = bank_str.len();

    let digits: Vec<u64> = bank_str
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u64))
        .collect();

    if string_size <= 1 {
        return 0;
    }

    while needed_digits > 0 {
        let mut largest_num: u64 = 0;
        let mut max_index = start_index;

        let end_index = string_size - needed_digits as usize + 1;
        for (i, bank_num) in digits[start_index..end_index].iter().enumerate() {
            if *bank_num > largest_num {
                largest_num = *bank_num;
                max_index = start_index + i;
            }
        }

        // add to our final value
        largest_power += largest_num * base;
        // shift our start index
        start_index = max_index + 1;
        // setup next base so the addition is correct
        base /= 10;
        // since we have a new digit we need one less, affects end index
        needed_digits -= 1;
    }

    largest_power
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
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(result, 3121910778619);
    }
}
