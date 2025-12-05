fn part1(src_info: &str) -> u32 {
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut valid_ingredients = 0;
    let mut ingred_ids = false;

    for line in src_info.split("\n") {
        if line.is_empty() {
            ingred_ids = true;
            // skip line so we dont parse empty junk
            continue;
        }

        if ingred_ids {
            let ingredient_id = line.parse::<u64>().unwrap();
            println!("parsed id: {}", ingredient_id);
            for (low, high) in &ranges {
                if low <= &ingredient_id && &ingredient_id <= high {
                    valid_ingredients += 1;
                    break;
                }
            }
        } else {
            let mut src_data = line.split("-");
            let low = src_data.next().unwrap().parse::<u64>().unwrap();
            let high = src_data.next().unwrap().parse::<u64>().unwrap();
            println!("parsed range: {}-{}", low, high);
            ranges.push((low, high));
        }
    }

    valid_ingredients
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
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert_eq!(result, 3);
    }
}
