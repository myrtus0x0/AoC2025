fn parse_to_vectors(input: &str) -> Vec<Vec<u64>> {
    let lines: Vec<_> = input.lines().collect();
    lines[..lines.len().saturating_sub(1)]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect()
        })
        .collect()
}

fn part1(src_info: &str) -> u64 {
    let data: Vec<Vec<u64>> = parse_to_vectors(src_info);
    let lines: Vec<_> = src_info.lines().collect();
    let operations: Vec<char> = lines
        .last()
        .unwrap()
        .chars()
        .filter(|c| *c == '*' || *c == '+')
        .collect();

    operations
        .iter()
        .enumerate()
        .map(|(col_index, opeartion)| {
            let column_values: Vec<u64> = data
                .iter()
                .filter_map(|row| row.get(col_index).copied())
                .collect();

            match *opeartion {
                '*' => column_values.iter().product(),
                '+' => column_values.iter().sum(),
                _ => 0,
            }
        })
        .sum()
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
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(result, 4277556);
    }
}
