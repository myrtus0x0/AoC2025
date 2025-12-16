use std::collections::HashMap;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

fn parse_input(text: &str) -> (HashMap<usize, Vec<String>>, Vec<Region>) {
    let lines: Vec<&str> = text.lines().collect();
    let mut shapes: HashMap<usize, Vec<String>> = HashMap::new();
    let mut regions: Vec<Region> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            if let Ok(shape_id) = parts[0].parse::<usize>() {
                let mut shape_lines = Vec::new();
                i += 1;

                while i < lines.len() && !lines[i].trim().is_empty() && !lines[i].contains(':') {
                    shape_lines.push(lines[i].trim().to_string());
                    i += 1;
                }

                shapes.insert(shape_id, shape_lines);
                continue;
            }
        }

        if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            let dimensions: Vec<&str> = parts[0].trim().split('x').collect();

            if let (Ok(width), Ok(height)) = (
                dimensions[0].parse::<usize>(),
                dimensions[1].parse::<usize>(),
            ) {
                let shape_counts: Vec<usize> = parts[1]
                    .trim()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                regions.push(Region {
                    width,
                    height,
                    shape_counts,
                });
            }
        }

        i += 1;
    }

    (shapes, regions)
}

fn part1(src_info: &str) -> u32 {
    let mut possible_orientations = 0;
    let (shapes, regions) = parse_input(src_info);

    for region in &regions {
        let mut total_required_area = 0;
        for (index, count) in region.shape_counts.iter().enumerate() {
            let shape_info = shapes.get(&index).unwrap();
            // count how many blocks are required for the given shape
            let shape_area: usize = shape_info
                .iter()
                .map(|x| x.chars().filter(|&c| c == '#').count())
                .sum();

            // multiply total blocks by how many shapes are required
            total_required_area += shape_area * count;
        }

        if total_required_area <= region.height * region.width {
            possible_orientations += 1;
        }
    }
    possible_orientations
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
            "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2",
        );
        assert_eq!(result, 2);
    }
}
