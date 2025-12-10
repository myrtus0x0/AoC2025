use std::collections::{HashMap, HashSet, VecDeque};

struct Coord {
    x: u32,
    y: u32,
    z: u32,
}

// math hakcs
fn euclidean_distance_squared(a: &Coord, b: &Coord) -> u64 {
    let dx = a.x.abs_diff(b.x) as u64;
    let dy = a.y.abs_diff(b.y) as u64;
    let dz = a.z.abs_diff(b.z) as u64;

    dx * dx + dy * dy + dz * dz
}

fn part1(src_info: &str) -> u32 {
    let coordinates: Vec<Coord> = src_info
        .lines()
        .map(|x| {
            let mut parts = x.split(",");
            let x = parts.next().unwrap().parse::<u32>().unwrap();
            let y = parts.next().unwrap().parse::<u32>().unwrap();
            let z = parts.next().unwrap().parse::<u32>().unwrap();
            Coord { x, y, z }
        })
        .collect();

    let mut edges: Vec<(u64, usize, usize)> = Vec::new();

    // generate all edges from every node, avoiding duplicates
    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let distance = euclidean_distance_squared(&coordinates[i], &coordinates[j]);
            edges.push((distance, i, j));
        }
    }

    // sort my distance
    edges.sort_by_key(|e| e.0);

    // index into coordinates -> vec of closest neighbors
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut connections_made = 0;

    for (_, i, j) in edges.iter() {
        // make N connections between the closest coordinates
        if connections_made >= 1000 {
            break;
        }

        adjacency.entry(*i).or_insert_with(Vec::new).push(*j);
        adjacency.entry(*j).or_insert_with(Vec::new).push(*i);
        connections_made += 1;
    }

    // bfs to find all connected components
    let mut visited = HashSet::new();
    let mut network_sizes = Vec::new();

    for start in 0..coordinates.len() {
        // if our network already our location, skip
        if visited.contains(&start) {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited.insert(start);
        let mut network_size = 0;

        while let Some(node) = queue.pop_front() {
            network_size += 1;

            if let Some(neighbors) = adjacency.get(&node) {
                for &neighbor in neighbors {
                    // if we havent visited the nieghbor, move to it
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        network_sizes.push(network_size);
    }

    // sort our sizes by greatest
    network_sizes.sort_by(|a, b| b.cmp(a));

    network_sizes[0] * network_sizes[1] * network_sizes[2]
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
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );
        assert_eq!(result, 40);
    }
}
