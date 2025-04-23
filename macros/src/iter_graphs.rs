use std::collections::HashSet;
use fixedbitset::FixedBitSet;

pub fn degeneracy_ordering<I>(graph: &[I], n: usize) -> Vec<usize>
where
    for<'a> &'a I: IntoIterator<Item = &'a usize>,
{
    let mut degrees: Vec<usize> = graph.iter().map(|neighbors| neighbors.into_iter().count()).collect();
    let mut order = Vec::with_capacity(n);
    let mut removed = FixedBitSet::with_capacity(n);

    for _ in 0..n {
        // Find vertex with smallest degree not yet removed
        let mut min_deg = usize::MAX;
        let mut min_v = None;
        for v in 0..n {
            if !removed.contains(v) && degrees[v] < min_deg {
                min_deg = degrees[v];
                min_v = Some(v);
            }
        }

        if let Some(v) = min_v {
            order.push(v);
            removed.insert(v);
            // Update degree of neighbors
            for &u in &graph[v] {
                if !removed.contains(u) {
                    degrees[u] -= 1;
                }
            }
        }
    }
    order
}

pub fn greedy_coloring<I>(graph: &[I], order: &[usize], n: usize) -> (usize, Vec<usize>)
where
    for<'a> &'a I: IntoIterator<Item = &'a usize>,
{
    let mut coloring = vec![usize::MAX; n];
    let mut used = HashSet::new();

    for &v in order {
        used.clear();
        for &u in &graph[v] {
            if coloring[u] != usize::MAX {
                used.insert(coloring[u]);
            }
        }

        // Find the first available color
        let color = (0..n)
            .find(|c| !used.contains(c))
            .expect("graph of n nodes should be colourable by n colors");
        coloring[v] = color;
    }

    let color_count: usize = coloring.iter().max().map_or(0, |x| x + 1);
    (color_count, coloring) // colors are 0-based
}

pub fn graph_coloring<I>(graph: &[I], n: usize) -> (usize, Vec<usize>)
where
    for<'a> &'a I: IntoIterator<Item = &'a usize>,
{
    let mut order = degeneracy_ordering(graph, n);
    order.reverse();
    greedy_coloring(graph, &order, n)
}