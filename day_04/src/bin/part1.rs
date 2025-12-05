struct FloorMap {
    map: Vec<Vec<char>>,
}

impl FloorMap {
    fn is_valid_location(&self, r: usize, c: usize) -> bool {
        let height = self.map.len();
        let width = self.map[0].len();

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut adjacent_rolls = 0;

        for (dr, dc) in directions {
            let new_r = r as i32 + dr;
            let new_c = c as i32 + dc;

            if new_r >= 0 && new_r < height as i32 && new_c >= 0 && new_c < width as i32 {
                let new_r = new_r as usize;
                let new_c = new_c as usize;

                if self.map[new_r][new_c] == '@' {
                    adjacent_rolls += 1;
                }
            }
        }

        adjacent_rolls < 4
    }
}

fn part1(src_info: &str) -> u32 {
    let mut valid_locations = 0;
    let my_map = FloorMap {
        map: src_info
            .lines()
            .map(|line| line.chars().collect())
            .collect(),
    };

    for (r_index, row) in my_map.map.iter().enumerate() {
        for (c_index, col_val) in row.iter().enumerate() {
            if *col_val == '@' {
                let res = my_map.is_valid_location(r_index, c_index);
                if res {
                    valid_locations += 1;
                }
                println!(
                    "checking position ({}, {}): {} - adjacent: {}",
                    r_index, c_index, col_val, res
                );
            }
        }
    }

    valid_locations
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
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        assert_eq!(result, 13);
    }
}
