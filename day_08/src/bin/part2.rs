use std::collections::HashMap;

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
    z: u32,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    num_components: usize,
}

// ty claude for showing me union find
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            num_components: n,
        }
    }

    // recursively go up tree to find the root
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    // find roots of both, and move one under the other
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // in same group
        }

        if self.rank[root_x] < self.rank[root_y] {
            // move smaller tree x under y
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            // mvoe smaller tree y under x
            self.parent[root_y] = root_x;
        } else {
            // same height, increase rank
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        self.num_components -= 1;

        true
    }
}

// math hakcs
fn euclidean_distance_squared(a: &Coord, b: &Coord) -> u64 {
    let dx = a.x.abs_diff(b.x) as u64;
    let dy = a.y.abs_diff(b.y) as u64;
    let dz = a.z.abs_diff(b.z) as u64;

    dx * dx + dy * dy + dz * dz
}

fn part2(src_info: &str) -> u64 {
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

    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();

    for (_, i, j) in edges.iter() {
        // make N connections between the closest coordinates
        adjacency.entry(*i).or_insert_with(Vec::new).push(*j);
        adjacency.entry(*j).or_insert_with(Vec::new).push(*i);
    }

    // implement union find
    // - each node is its own parent
    // - each node has a rank of 0 to start with
    // -
    // UnionFind {
    //     parent: (0..n).collect(),
    //     rank: vec![0; n],
    //     num_components: n,
    // }
    let mut uf_handler = UnionFind::new(coordinates.len());
    for edge in edges {
        let (_, node_1, node_2) = edge;
        // connect our edges
        uf_handler.union(node_1, node_2);
        if uf_handler.num_components == 1 {
            println!("node 1: {:?}", coordinates[node_1]);
            println!("node 2: {:?}", coordinates[node_2]);
            return coordinates[node_1].x as u64 * coordinates[node_2].x as u64;
        }
    }

    0
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
        assert_eq!(result, 25272);
    }
}
