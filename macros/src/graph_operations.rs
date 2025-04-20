use std::collections::{BinaryHeap, HashSet};

use fixedbitset::FixedBitSet;

/// Compute degeneracy ordering using a simple greedy peeling algorithm
pub fn degeneracy_ordering(graph: &Vec<FixedBitSet>, n: usize) -> Vec<usize> {
    let mut degrees: Vec<usize> = graph.iter().map(|neighbors| neighbors.count_ones(..)).collect();
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
            for u in graph[v].ones() {
                if !removed.contains(u) {
                    degrees[u] -= 1;
                }
            }
        }
    }

    order.reverse(); // For coloring, reverse it (lowest-degree last)
    order
}

pub fn greedy_coloring(graph: &Vec<FixedBitSet>, order: &[usize], n: usize) -> (usize, Vec<usize>) {
    let mut coloring = vec![usize::MAX; n];

    for &v in order {
        let mut used = FixedBitSet::with_capacity(n);
        for u in graph[v].ones() {
            if coloring[u] != usize::MAX {
                used.insert(coloring[u]);
            }
        }

        // Find the first available color
        let color = (0..n).find(|c| !used.contains(*c)).expect("color not available");
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

/// Exact graph coloring using backtracking
pub fn color_graph(graph: &Vec<FixedBitSet>, n: usize) -> (usize, Vec<usize>) {
    let order = degeneracy_ordering(graph, n);
    let mut colors = vec![None; n];
    let (initial_upper_bound, initial_coloring) = greedy_coloring(graph, &order, n);
    //let (initial_upper_bound, initial_coloring) = dsatur(graph);
    let mut best_coloring = initial_coloring.clone();
    let mut best_color_count = initial_upper_bound;
    // dbg!(&best_coloring, &best_color_count);

    fn backtrack(
        graph: &Vec<FixedBitSet>,
        order: &[usize],
        idx: usize,
        colors: &mut [Option<usize>],
        max_color: usize,
        best_coloring: &mut Vec<usize>,
        best_color_count: &mut usize,
    ) {
        if idx == order.len() {
            if max_color < *best_color_count {
                *best_color_count = max_color;
                for (i, &c) in colors.iter().enumerate() {
                    best_coloring[i] = c.unwrap();
                }
            }
            return;
        }

        let v = order[idx];
        let mut used = FixedBitSet::with_capacity(max_color + 1);
        for u in graph[v].ones() {
            if let Some(c) = colors[u] {
                used.insert(c);
            }
        }

        for color in 0..=*best_color_count {
            if !used.contains(color) {
                colors[v] = Some(color);
                let new_max = max_color.max(color + 1);
                if new_max <= *best_color_count {
                    backtrack(
                        graph,
                        order,
                        idx + 1,
                        colors,
                        new_max,
                        best_coloring,
                        best_color_count,
                    );
                }
                colors[v] = None;
            }
        }
    }

    backtrack(
        graph,
        &order,
        0,
        &mut colors,
        0,
        &mut best_coloring,
        &mut best_color_count,
    );
    // dbg!(&best_coloring, &best_color_count);

    (best_color_count, best_coloring)
}
