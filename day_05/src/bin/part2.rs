fn merge_ranges(ranges: &mut Vec<(u64, u64)>) {
    if ranges.is_empty() {
        return;
    }

    // sort by the low values
    ranges.sort_by_key(|r| r.0);
    // [(3, 5)]
    let mut merged = vec![ranges[0]];

    for &(low, high) in &ranges[1..] {
        // (12, 18)
        let last = merged.last_mut().unwrap();
        // if 10 <= 12 +1
        if low <= last.1 + 1 {
            // 12 = 18
            last.1 = last.1.max(high);
            // (10, 18)
        } else {
            // [(3, 5), (10, 20)]
            merged.push((low, high));
        }
    }

    *ranges = merged;
}

fn part2(src_info: &str) -> u64 {
    let mut ranges: Vec<(u64, u64)> = vec![];
    for line in src_info.split("\n") {
        if line.is_empty() {
            break;
        }

        let mut src_data = line.split("-");
        let low = src_data.next().unwrap().parse::<u64>().unwrap();
        let high = src_data.next().unwrap().parse::<u64>().unwrap();
        println!("parsed range: {}-{}", low, high);
        // cant do set lookups as there are too many numbers :(
        ranges.push((low, high));
    }

    merge_ranges(&mut ranges);
    println!("merged range: {}", ranges.len());
    // dbg!(ranges);

    ranges.iter().map(|(low, high)| high - low + 1).sum()
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
        assert_eq!(result, 14);
    }
}
