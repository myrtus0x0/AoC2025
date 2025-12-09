use std::collections::HashSet;
use std::thread;
use std::time::Duration;

fn pretty_print_board(
    grid: &[Vec<char>],
    activated: &HashSet<(usize, usize)>,
    beam_positions: &HashSet<(usize, usize, u8)>,
) {
    // clear screen and move cursor to top-left
    print!("\x1b[2J\x1b[H");

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    for r in 0..n_rows {
        for c in 0..n_cols {
            let cell = grid[r][c];

            if cell == 'S' {
                print!("{}", cell);
            } else if cell == '^' {
                if activated.contains(&(r, c)) {
                    print!("\x1b[91m^\x1b[0m");
                } else {
                    print!("^");
                }
            } else {
                let has_down = beam_positions.contains(&(r, c, 0));
                let has_left = beam_positions.contains(&(r, c, 1));
                let has_right = beam_positions.contains(&(r, c, 2));

                let beam_count = [has_down, has_left, has_right]
                    .iter()
                    .filter(|&&x| x)
                    .count();

                if beam_count > 0 {
                    if has_down && (has_left || has_right) {
                        print!("\x1b[93m+\x1b[0m");
                    } else if has_down {
                        print!("\x1b[92m|\x1b[0m");
                    } else if has_left || has_right {
                        print!("\x1b[94m-\x1b[0m");
                    } else {
                        print!(".");
                    }
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }

    println!("\nactivated splitters: {}", activated.len());
    thread::sleep(Duration::from_millis(50));
}

fn part1(src_info: &str) -> u32 {
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

    let mut activated = HashSet::new();
    let mut queue = vec![(start_pos.0, start_pos.1, 0)];
    let mut visited = HashSet::new();
    let mut beam_positions = HashSet::new();

    while let Some((r, c, mut dir)) = queue.pop() {
        pretty_print_board(&grid, &activated, &beam_positions);

        if !visited.insert((r, c, dir)) {
            continue;
        }

        beam_positions.insert((r, c, dir));

        if grid[r][c] == '^' {
            activated.insert((r, c));

            let left = (r as u32, c as u32 - 1);
            let right = (r as u32, c as u32 + 1);

            if left.1 < n_cols as u32 {
                queue.push((left.0 as usize, left.1 as usize, 1));
            }
            if right.1 < n_cols as u32 {
                queue.push((right.0 as usize, right.1 as usize, 2));
            }
            continue;
        } else {
            dir = 0;
        }

        let (nr, nc) = match dir {
            0 => (r as u32 + 1, c as u32), // down
            1 => (r as u32, c as u32 - 1), // left
            2 => (r as u32, c as u32 + 1), // right
            _ => continue,
        };

        if nr >= n_rows as u32 || nc >= n_cols as u32 {
            continue;
        }

        let (nr, nc) = (nr as usize, nc as usize);
        queue.push((nr, nc, dir));
    }

    pretty_print_board(&grid, &activated, &beam_positions);

    activated.len() as u32
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part1(input);
    println!("\nFinal answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part1(
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
        assert_eq!(result, 21);
    }
}
