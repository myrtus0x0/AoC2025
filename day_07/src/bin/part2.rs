use std::collections::{HashMap, HashSet};

fn count_paths(
    state: (usize, usize),
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    memo: &mut HashMap<(usize, usize), usize>,
    n_rows: usize,
) -> usize {
    // return cached result
    if let Some(&count) = memo.get(&state) {
        return count;
    }

    let (r, _) = state;

    // base case: if we've reached the bottom row, this is 1 path
    if r == n_rows - 1 {
        memo.insert(state, 1);
        return 1;
    }

    let neighbors = &graph[&state];

    // base case: if this node has no neighbors, it's 1 path
    if neighbors.is_empty() {
        memo.insert(state, 1);
        return 1;
    }

    // recursive case: sum paths from all neighbors
    let total = neighbors
        .iter()
        .map(|&next_state| count_paths(next_state, graph, memo, n_rows))
        .sum();

    memo.insert(state, total);
    total
}

fn part2(src_info: &str) -> usize {
    let grid: Vec<Vec<char>> = src_info
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut start_pos = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_pos = (r, c);
                break;
            }
        }
    }

    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut queue = vec![(start_pos.0, start_pos.1, 0)];
    let mut visited = HashSet::new();
    let mut beam_positions = HashSet::new();

    while let Some((r, c, mut dir)) = queue.pop() {
        if !visited.insert((r, c, dir)) {
            continue;
        }

        beam_positions.insert((r, c, dir));

        if grid[r][c] == '^' {
            let mut neighbors = Vec::new();

            let left = (r as u32, c as u32 - 1);
            let right = (r as u32, c as u32 + 1);

            if left.1 < n_cols as u32 {
                neighbors.push((r, c - 1));
                queue.push((left.0 as usize, left.1 as usize, 1));
            }
            if right.1 < n_cols as u32 {
                neighbors.push((r, c + 1));
                queue.push((right.0 as usize, right.1 as usize, 2));
            }

            graph.insert((r, c), neighbors);
            continue;
        } else {
            // needed here as the default direction the beam goes is down. So unless we hit a
            // split, we are always going down
            dir = 0;
        }

        let (nr, nc) = match dir {
            0 => (r as u32 + 1, c as u32), // down
            1 => (r as u32, c as u32 - 1), // left
            2 => (r as u32, c as u32 + 1), // right
            _ => continue,
        };

        // we've gone over the edges of the graph
        if nr >= n_rows as u32 || nc >= n_cols as u32 {
            graph.insert((r, c), vec![]);
            continue;
        }

        let (nr, nc) = (nr as usize, nc as usize);
        graph.insert((r, c), vec![(nr, nc)]);
        queue.push((nr, nc, dir));
    }

    // at this point our graph is generated, now we need to determine how many unique paths there
    // are

    for (location, neighbors) in &graph {
        println!("loc: {:?} neighbors: {:?}", location, neighbors);
    }

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    count_paths((start_pos.0, start_pos.1), &graph, &mut memo, n_rows)
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part2(input);
    println!("\nFinal answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        // assert_eq!(result, 0);
        assert_eq!(result, 40);
    }
}
