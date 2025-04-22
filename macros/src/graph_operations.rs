use std::{
    collections::{BTreeSet, BinaryHeap, HashSet},
    ops::Deref,
};

use fixedbitset::FixedBitSet;

/// for this specific use case, i have tested:
/// degeneracy ordering -> greedy - low number of colors fast
/// dsatur - many colors (idk why, dsatur is supposed to be good), fast
/// inverse graph clique covering - many colors, fast
/// exact coloring (brute force backtracking solution) - way too slow

/// Compute degeneracy ordering using a simple greedy peeling algorithm
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

pub fn dsatur(graph: &Vec<FixedBitSet>, n: usize) -> (usize, Vec<usize>) {
    let mut colors = vec![None; n]; // None means uncolored
    let mut saturation_degree = vec![0; n]; // Saturation degrees
    let mut degrees = vec![0; n]; // Vertex degrees
    let mut heap = BinaryHeap::new();

    // Calculate initial degree for each vertex
    for i in 0..n {
        degrees[i] = graph[i].count_ones(..);
    }

    // Initialize the heap with all vertices
    for i in 0..n {
        heap.push((0, degrees[i], i)); // (saturation_degree, degree, vertex_index)
    }

    while let Some((_, degree, vertex)) = heap.pop() {
        // Find the lowest color that is not taken by neighbors
        let mut forbidden_colors = HashSet::new();
        for neighbor in graph[vertex].ones() {
            if let Some(color) = colors[neighbor] {
                forbidden_colors.insert(color);
            }
        }

        // Assign the smallest possible color
        let mut color = 0;
        while forbidden_colors.contains(&color) {
            color += 1;
        }
        colors[vertex] = Some(color);

        // Update the saturation degree for neighbors
        for neighbor in graph[vertex].ones() {
            if colors[neighbor].is_none() {
                // Calculate the new saturation degree
                let new_saturation_degree = graph[neighbor]
                    .ones()
                    .filter(|&adj| colors[adj].is_some())
                    .count();
                saturation_degree[neighbor] = new_saturation_degree;

                // Push the updated vertex back into the heap with the new saturation degree
                heap.push((new_saturation_degree, degrees[neighbor], neighbor));
            }
        }
    }

    // Convert colors from Option<usize> to usize, and calculate the max color used
    let mut coloring = Vec::with_capacity(n);
    let mut max_color = 0;
    for &color in colors.iter() {
        if let Some(c) = color {
            coloring.push(c);
            if c > max_color {
                max_color = c;
            }
        } else {
            // In case some vertex remains uncolored, this should not happen in DSatur
            coloring.push(usize::MAX); // Just in case of error handling
        }
    }

    (max_color + 1, coloring) // Return color count and the coloring
}

// /// Exact graph coloring using backtracking
// pub fn color_graph(graph: &Vec<FixedBitSet>, n: usize) -> (usize, Vec<usize>) {
//     let order = degeneracy_ordering(graph, n);
//     let mut colors = vec![None; n];
//     let (initial_upper_bound, initial_coloring) = greedy_coloring(graph, &order, n);
//     //let (initial_upper_bound, initial_coloring) = dsatur(graph);
//     let mut best_coloring = initial_coloring.clone();
//     let mut best_color_count = initial_upper_bound;
//     // dbg!(&best_coloring, &best_color_count);

//     fn backtrack(
//         graph: &Vec<FixedBitSet>,
//         order: &[usize],
//         idx: usize,
//         colors: &mut [Option<usize>],
//         max_color: usize,
//         best_coloring: &mut Vec<usize>,
//         best_color_count: &mut usize,
//     ) {
//         if idx == order.len() {
//             if max_color < *best_color_count {
//                 *best_color_count = max_color;
//                 for (i, &c) in colors.iter().enumerate() {
//                     best_coloring[i] = c.unwrap();
//                 }
//             }
//             return;
//         }

//         let v = order[idx];
//         let mut used = FixedBitSet::with_capacity(max_color + 1);
//         for u in graph[v].ones() {
//             if let Some(c) = colors[u] {
//                 used.insert(c);
//             }
//         }

//         for color in 0..=*best_color_count {
//             if !used.contains(color) {
//                 colors[v] = Some(color);
//                 let new_max = max_color.max(color + 1);
//                 if new_max <= *best_color_count {
//                     backtrack(
//                         graph,
//                         order,
//                         idx + 1,
//                         colors,
//                         new_max,
//                         best_coloring,
//                         best_color_count,
//                     );
//                 }
//                 colors[v] = None;
//             }
//         }
//     }

//     backtrack(
//         graph,
//         &order,
//         0,
//         &mut colors,
//         0,
//         &mut best_coloring,
//         &mut best_color_count,
//     );
//     // dbg!(&best_coloring, &best_color_count);

//     (best_color_count, best_coloring)
// }

/// Finds maximal cliques containing all the vertices in r, some of the
/// vertices in p, and none of the vertices in x.
fn bron_kerbosch_pivot(
    non_exclusive: &Vec<FixedBitSet>,
    r: FixedBitSet,
    mut p: FixedBitSet,
    mut x: FixedBitSet,
    n: usize,
    // ignore: &HashSet<usize>,
) -> Vec<FixedBitSet> {
    let mut cliques = Vec::with_capacity(1);
    if p.is_clear() {
        if x.is_clear() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    // println!("{}", p);
    let u = p
        .ones()
        .max_by_key(|&v| {
            let mut neighbours = 0;
            for i in 0..n {
                // if ignore.contains(&v) || ignore.contains(&i) {
                //     continue;
                // }
                if !non_exclusive[v].contains(i) && !non_exclusive[i].contains(v) {
                    neighbours += 1;
                }
                // if mx_contains_edge(mx, v, i) {
                //     neighbours += 1;
                // }
            }
            neighbours
        })
        .expect("there should be a vertex with max degree");
    let mut todo = p
        .ones()
        //skip neighbors of pivot
        .filter(|&v| {
            // if ignore.contains(&v) {
            //     return false;
            // }
            if u == v {
                return true;
            }
            !(!non_exclusive[u].contains(v) && !non_exclusive[v].contains(u))
        })
        .collect::<Vec<_>>();
    while let Some(v) = todo.pop() {
        let mut neighbors = FixedBitSet::from_iter(0..n);
        neighbors.difference_with(&non_exclusive[v]);
        // for ig in ignore {
        //     neighbors.remove(*ig);
        // }

        p.remove(v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let mut next_p = p.clone();
        next_p.intersect_with(&neighbors);

        let mut next_x = x.clone();
        next_x.intersect_with(&neighbors);

        cliques.extend(bron_kerbosch_pivot(
            non_exclusive,
            next_r,
            next_p,
            next_x,
            n,
        ));

        x.insert(v);
    }

    cliques
}

// fn non_exclusive(bit_sets: &Vec<FixedBitSet>, n: usize) -> Vec<FixedBitSet> {
//     let mut non_exclusive = vec![FixedBitSet::with_capacity(n); n];
//     for bit in 0..n {
//         for bit_set in bit_sets {
//             if bit_set.contains(bit) {
//                 non_exclusive[bit].union_with(bit_set);
//             }
//         }
//     }
//     non_exclusive
// }

pub fn cliques(non_exclusive: &Vec<FixedBitSet>, n: usize) -> (usize, Vec<usize>) {
    let mut cliques = Vec::new();
    let mut i = 0;
    let mut p = FixedBitSet::from_iter(0..n);
    while i < n {
        let r = FixedBitSet::with_capacity(n);
        let x = FixedBitSet::with_capacity(n);
        let c = bron_kerbosch_pivot(&non_exclusive, r, p.clone(), x, n);
        let c = c.into_iter().max_by_key(|c| c.count_ones(..)).unwrap();
        i += c.count_ones(..);
        for a in c.ones() {
            p.remove(a);
        }
        cliques.push(c.into_ones().collect::<Vec<_>>());
    }
    let mut coloring = vec![cliques.len(); n];
    for (color, clique) in cliques.iter().enumerate() {
        for &token_index in clique {
            coloring[token_index] = color;
        }
    }
    (cliques.len(), coloring)
}
