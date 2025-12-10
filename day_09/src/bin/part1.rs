fn part1(src_info: &str) -> u64 {
    let mut max_size: u64 = 0;

    let coordinates: Vec<(u32, u32)> = src_info
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            let a = parts.next().unwrap().parse::<u32>().unwrap();
            let b = parts.next().unwrap().parse::<u32>().unwrap();
            (a, b)
        })
        .collect();

    for (r, c) in &coordinates {
        for (n_r, n_c) in &coordinates {
            let r_diff: u64 = (r.abs_diff(*n_r) + 1) as u64;
            let c_diff: u64 = (c.abs_diff(*n_c) + 1) as u64;

            if r_diff * c_diff > max_size {
                max_size = r_diff * c_diff;
            }
        }
    }

    max_size
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
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(result, 50);
    }
}
