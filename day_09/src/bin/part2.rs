fn pretty_print_graph(graph: &Vec<Vec<char>>) {
    for line in graph {
        for c in line {
            print!(" {}", c);
        }
        println!();
    }
}

// ty claude
fn build_prefix_sum(graph: &Vec<Vec<char>>) -> Vec<Vec<u64>> {
    let rows = graph.len();
    let cols = graph[0].len();
    let mut prefix = vec![vec![0u64; cols + 1]; rows + 1];

    for r in 0..rows {
        for c in 0..cols {
            let filled = if graph[r][c] != '.' { 1 } else { 0 };
            prefix[r + 1][c + 1] = filled + prefix[r][c + 1] + prefix[r + 1][c] - prefix[r][c];
        }
    }

    prefix
}

fn is_valid_box_old(graph: &Vec<Vec<char>>, coord_1: (u32, u32), coord_2: (u32, u32)) -> bool {
    let (r, c) = coord_1;
    let (n_r, n_c) = coord_2;

    for d_c in c.min(n_c)..c.max(n_c) {
        for d_r in r.min(n_r)..r.max(n_r) {
            if graph[d_r as usize][d_c as usize] == '.' {
                return false;
            }
        }
    }

    true
}

fn is_valid_box(prefix: &Vec<Vec<u64>>, coord_1: (u32, u32), coord_2: (u32, u32)) -> bool {
    let (r1, c1) = coord_1;
    let (r2, c2) = coord_2;

    let min_r = r1.min(r2) as usize;
    let max_r = r1.max(r2) as usize;
    let min_c = c1.min(c2) as usize;
    let max_c = c1.max(c2) as usize;

    let width = (max_c - min_c + 1) as u64;
    let height = (max_r - min_r + 1) as u64;
    let expected_filled = width * height;

    let sum = prefix[max_r + 1][max_c + 1] + prefix[min_r][min_c]
        - prefix[min_r][max_c + 1]
        - prefix[max_r + 1][min_c];

    sum == expected_filled
}

fn flood_fill(graph: &mut Vec<Vec<char>>, size_graph: u32) {
    let mut stack = vec![(0, 0)];
    while let Some((row, col)) = stack.pop() {
        if row >= size_graph || col >= size_graph {
            continue;
        }

        if graph[row as usize][col as usize] == '.' {
            graph[row as usize][col as usize] = 'O';
            if row > 0 {
                stack.push((row - 1, col));
            }
            if col > 0 {
                stack.push((row, col - 1));
            }
            stack.push((row + 1, col));
            stack.push((row, col + 1));
        }
    }

    // take inverse of outside and coordinate and wall, thatll be the inside
    for row in 0..size_graph {
        for col in 0..size_graph {
            if graph[row as usize][col as usize] == '.' {
                graph[row as usize][col as usize] = 'X';
            }
        }
    }

    // change all the Os back to period
    for row in 0..size_graph {
        for col in 0..size_graph {
            if graph[row as usize][col as usize] == 'O' {
                graph[row as usize][col as usize] = '.';
            }
        }
    }
}

fn part2(src_info: &str) -> u64 {
    let mut max_area: u64 = 0;

    let coordinates: Vec<(u32, u32)> = src_info
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            let a = parts.next().unwrap().parse::<u32>().unwrap();
            let b = parts.next().unwrap().parse::<u32>().unwrap();
            (a, b)
        })
        .collect();

    // determine how big our graph needs to be
    let mut max_size = 0;
    for i in &coordinates {
        if i.0 > max_size {
            max_size = i.0;
        }

        if i.1 > max_size {
            max_size = i.1;
        }
    }

    // add padding to graph
    max_size += 2;

    println!("creating graph");
    let mut graph: Vec<Vec<char>> = vec![];
    for _ in 0..max_size {
        graph.push(vec!['.'; max_size as usize]);
    }

    println!("drawing corners");
    for (col, row) in &coordinates {
        graph[*row as usize][*col as usize] = '#';
    }

    println!("drawing edges");
    for i in 0..coordinates.len() {
        let (current_col, current_row) = coordinates[i];
        let (next_col, next_row) = coordinates[(i + 1) % coordinates.len()];

        if current_row == next_row {
            for diff in current_col.min(next_col) + 1..current_col.max(next_col) {
                graph[current_row as usize][diff as usize] = 'X';
            }
        } else if current_col == next_col {
            for diff in current_row.min(next_row) + 1..current_row.max(next_row) {
                graph[diff as usize][current_col as usize] = 'X';
            }
        }
    }

    // fill in the graph
    println!("prefill");
    // pretty_print_graph(&graph);
    flood_fill(&mut graph, max_size);
    println!("filled");
    // pretty_print_graph(&graph);

    let prefix = build_prefix_sum(&graph);
    println!("prefix sum built");

    for i in 0..coordinates.len() {
        let (col_i, row_i) = coordinates[i];
        for j in i..coordinates.len() {
            let (col_j, row_j) = coordinates[j];
            if is_valid_box(&prefix, (row_i, col_i), (row_j, col_j)) {
                let col_diff: u64 = (col_i.abs_diff(col_j) + 1) as u64;
                let row_diff: u64 = (row_i.abs_diff(row_j) + 1) as u64;
                if row_diff * col_diff > max_area {
                    max_area = row_diff * col_diff;
                }
            }
        }
    }

    max_area
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
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(result, 24);
    }
}
