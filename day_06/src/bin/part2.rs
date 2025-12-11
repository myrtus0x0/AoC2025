fn find_column_boundaries(lines: &[&str]) -> Vec<(usize, usize)> {
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut has_content = vec![false; max_len];

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            if ch != ' ' {
                has_content[i] = true;
            }
        }
    }

    let mut columns = Vec::new();
    let mut start = None;

    for (i, &has) in has_content.iter().enumerate() {
        if has && start.is_none() {
            start = Some(i);
        } else if !has && start.is_some() {
            columns.push((start.unwrap(), i));
            start = None;
        }
    }

    if let Some(s) = start {
        columns.push((s, max_len));
    }

    columns
}

fn parse_and_transpose(input: &str) -> Vec<Vec<u64>> {
    let lines: Vec<_> = input.lines().collect();
    let number_lines = &lines[..lines.len().saturating_sub(1)];
    // calculate boundaries to help with preserving padding
    let boundaries = find_column_boundaries(number_lines);

    let mut transposed_entire_data: Vec<Vec<u64>> = vec![];

    // capture correctly padded columns so we can read top down
    let columns: Vec<Vec<String>> = boundaries
        .iter()
        .map(|(start, end)| {
            number_lines
                .iter()
                .map(|line| {
                    if line.len() >= *end {
                        line[*start..*end].to_string()
                    } else if line.len() > *start {
                        line[*start..].to_string()
                    } else {
                        " ".repeat(end - start)
                    }
                })
                .collect()
        })
        .collect();

    dbg!(&columns);

    for column in columns {
        let size = column[0].len();
        let mut transposed_row: Vec<String> = vec![];
        // turn
        // [
        //   "123",
        //   " 45",
        //   "  6",
        // ],
        // into [1, 24, 356]
        for i in 0..size {
            let new_num_str: String = column
                .iter()
                .filter_map(|x| x.chars().nth(i))
                .filter(|&s| s != ' ')
                .collect();

            transposed_row.push(new_num_str);
        }

        // convert from "356" to actual 356
        let transposed_as_ints: Vec<u64> = transposed_row
            .iter()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        transposed_entire_data.push(transposed_as_ints);
    }

    transposed_entire_data
}

fn part2(src_info: &str) -> u64 {
    let mut sum: u64 = 0;
    let data: Vec<Vec<u64>> = parse_and_transpose(src_info);
    let lines: Vec<_> = src_info.lines().collect();

    let operations: Vec<char> = lines
        .last()
        .unwrap()
        .chars()
        .filter(|c| *c == '*' || *c == '+')
        .collect();

    dbg!(&data);
    dbg!(&operations);

    for (index, operation) in operations.iter().enumerate() {
        let value = match operation {
            '*' => data[index].iter().product(),
            '+' => data[index].iter().sum(),
            _ => 0,
        };
        println!("from {:?}, adding: {}", data[index], value);
        sum += value;
    }

    sum
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
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(result, 3263827);
    }
}
