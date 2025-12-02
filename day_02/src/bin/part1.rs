fn part1(src_info: &str) -> u64 {
    let mut total = 0;

    for range_str in src_info.split(",") {
        let mut range_val = range_str.split("-");
        let lower_bound = range_val.next().unwrap().trim_end().parse::<u64>().unwrap();
        let upper_bound = range_val.next().unwrap().trim_end().parse::<u64>().unwrap();

        println!("parsed range {}-{}", lower_bound, upper_bound);
        for i in lower_bound..=upper_bound {
            if is_valid_id(i) {
                total += i;
            }
        }
    }
    total
}

fn is_valid_id(id_val: u64) -> bool {
    // get powers of 10 of number 2222 _ _ _ _
    // split into halves, 22 & 22
    // check equality

    // 4
    let num_digits = (id_val as f64).log10().floor() as u32 + 1;

    // 100
    let mid_base = 10_u32.pow(num_digits / 2);

    // _ _ 22
    let back_half = id_val % mid_base as u64;

    // 22 _ _
    let front_half = id_val / mid_base as u64;

    front_half == back_half
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
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(result, 1227775554);
    }
}
