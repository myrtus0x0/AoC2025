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

fn part2(src_info: &str) -> u32 {
    let mut valid_locations = 0;
    let mut my_map = FloorMap {
        map: src_info
            .lines()
            .map(|line| line.chars().collect())
            .collect(),
    };

    let height = my_map.map.len();
    let width = my_map.map[0].len();

    loop {
        let mut did_change = false;

        for r_index in 0..height {
            for c_index in 0..width {
                if my_map.map[r_index][c_index] == '@' {
                    let res = my_map.is_valid_location(r_index, c_index);
                    if res {
                        my_map.map[r_index][c_index] = '.';
                        valid_locations += 1;
                        did_change = true;
                    }
                    println!(
                        "checking position ({}, {}): {} - adjacent: {}",
                        r_index, c_index, my_map.map[r_index][c_index], res
                    );
                }
            }
        }

        if !did_change {
            break;
        }
    }

    valid_locations
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
        assert_eq!(result, 43);
    }
}
