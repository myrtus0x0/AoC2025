use std::collections::{HashMap, HashSet};

fn dfs<'a>(
    node: &'a str,
    end: &str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
) -> usize {
    if node == end {
        return 1;
    }

    // dfs - visit node, turns out we dont actually need visited
    visited.insert(node);
    let mut count = 0;

    if let Some(neighbors) = graph.get(node) {
        // if we already havent visited node, visit it and begin dfs again
        for &neighbor in neighbors {
            if !visited.contains(neighbor) {
                count += dfs(neighbor, end, graph, visited);
            }
        }
    }

    // unmark node so it can be visited in other paths
    visited.remove(node);
    count
}

fn part1(src_info: &str) -> u32 {
    let map: HashMap<&str, Vec<&str>> = src_info
        .lines()
        .map(|line| {
            let (src, dests) = line.split_once(": ").unwrap();
            let neighbors = dests.split_whitespace().collect();
            (src, neighbors)
        })
        .collect();

    for entry in &map {
        println!("{:?}", entry);
    }

    dfs("you", "out", &map, &mut HashSet::new()) as u32
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
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        );
        assert_eq!(result, 5);
    }
}
