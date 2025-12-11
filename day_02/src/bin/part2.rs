fn part2(src_info: &str) -> u64 {
    let mut total = 0;

    for range_str in src_info.split(",") {
        let mut range_val = range_str.split("-");
        let lower_bound = range_val.next().unwrap().trim_end().parse::<u64>().unwrap();
        let upper_bound = range_val.next().unwrap().trim_end().parse::<u64>().unwrap();

        for i in lower_bound..=upper_bound {
            if is_valid_id(i) {
                total += i;
            }
        }
    }
    total
}

fn is_valid_id(id_val: u64) -> bool {
    // fuck it, to strings we go
    let id_str = id_val.to_string();
    let len = id_str.len();

    for chunk_size in 1..len {
        // avoid processing strings that cant be multiples of the chunk
        if !len.is_multiple_of(chunk_size) {
            continue;
        }

        let chunk = &id_str[..chunk_size];
        // check if all components of the string are the target chunk
        if id_str
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .all(|c| c.iter().collect::<String>() == chunk)
        {
            return true;
        }
    }

    false
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
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(result, 4174379265);
    }
}
