use std::collections::HashMap;

fn dfs<'a>(
    node: &'a str,
    end: &str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    // return cached if already computed
    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    // reached end
    if node == end {
        return 1;
    }

    let mut count = 0;

    // explore all neighbors
    if let Some(neighbors) = graph.get(node) {
        for &neighbor in neighbors {
            count += dfs(neighbor, end, graph, memo);
        }
    }
    // insert into cache how many valid locations there are from that location
    memo.insert(node, count);
    count
}

fn part2(src_info: &str) -> usize {
    let graph: HashMap<&str, Vec<&str>> = src_info
        .lines()
        .map(|line| {
            let (src, dests) = line.split_once(": ").unwrap();
            let neighbors = dests.split_whitespace().collect();
            (src, neighbors)
        })
        .collect();

    let mut total = 0;

    // dfs(svr→ fft) * dfs(fft→ dac) * dfs(dac→ out)
    let svr_to_fft = dfs("svr", "fft", &graph, &mut HashMap::new());
    let fft_to_dac = dfs("fft", "dac", &graph, &mut HashMap::new());
    let dac_to_out = dfs("dac", "out", &graph, &mut HashMap::new());

    // dfs(svr→ dac) * dfs(dac→ fft) * dfs(fft→ out)
    let svr_to_dac = dfs("svr", "dac", &graph, &mut HashMap::new());
    let dac_to_fft = dfs("dac", "fft", &graph, &mut HashMap::new());
    let fft_to_out = dfs("fft", "out", &graph, &mut HashMap::new());

    total += svr_to_fft * fft_to_dac * dac_to_out;
    total += svr_to_dac * dac_to_fft * fft_to_out;

    total
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
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        );
        assert_eq!(result, 2);

        // Debug: let's see the intermediate values
        let graph: HashMap<&str, Vec<&str>> = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
            .lines()
            .map(|line| {
                let (src, dests) = line.split_once(": ").unwrap();
                let neighbors = dests.split_whitespace().collect();
                (src, neighbors)
            })
            .collect();

        println!(
            "svr→fft: {}",
            dfs("svr", "fft", &graph, &mut HashMap::new())
        );
        println!(
            "fft→dac: {}",
            dfs("fft", "dac", &graph, &mut HashMap::new())
        );
        println!(
            "dac→out: {}",
            dfs("dac", "out", &graph, &mut HashMap::new())
        );
        println!(
            "svr→dac: {}",
            dfs("svr", "dac", &graph, &mut HashMap::new())
        );
        println!(
            "dac→fft: {}",
            dfs("dac", "fft", &graph, &mut HashMap::new())
        );
        println!(
            "fft→out: {}",
            dfs("fft", "out", &graph, &mut HashMap::new())
        );
    }
}
