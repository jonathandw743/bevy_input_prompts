use std::{collections::HashMap, fs::read_dir, path::Path};

use bimap::BiMap;
use itertools::Itertools;
use petgraph::{Graph, algo::toposort};

fn main() {
    let p =
        Path::new("assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default");
    let mut g = Graph::<String, ()>::new();
    let mut map = HashMap::new();
    for e in read_dir(p).unwrap() {
        let mut v = Vec::new();
        for p in e
            .unwrap()
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .split("_")
        {
            let s = p.to_string();
            let ni = map.entry(s).or_insert_with(|| g.add_node(p.to_string()));
            v.push(*ni);
        }
        for i in 0..(v.len() - 1) {
            g.add_edge(v[i], v[i + 1], ());
        }
    }
    let sort = toposort(&g, None).unwrap();
    for x in sort {
        dbg!(g.node_weight(x).unwrap());
    }
}

// fn rec_dir<P: AsRef<Path>>(p: P) {
//     let p = p.as_ref();
//     if p.is_file() {
//         dbg!(p.file_stem().unwrap().to_str().unwrap().split("_").collect::<Vec<_>>());
//     }
//     if p.is_dir() {
//         for e in read_dir(p).unwrap() {
//             rec_dir(e.unwrap().path());
//         }
//     }
// }
