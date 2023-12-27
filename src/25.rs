use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use itertools::Itertools;

fn main() {
    let graph = {
        let mut graph = Vec::new();
        let mut names = Vec::new();
        let mut indices = HashMap::new();
        fn resolve(
            graph: &mut Vec<Vec<usize>>,
            names: &mut Vec<String>,
            indices: &mut HashMap<String, usize>,
            name: &str,
        ) -> usize {
            match indices.get(name) {
                Some(&i) => i,
                None => {
                    let i = names.len();
                    indices.insert(String::from(name), i);
                    names.push(String::from(name));
                    graph.push(Vec::new());
                    i
                }
            }
        }
        for res in stdin().lines() {
            let line = res.unwrap();
            let (from, to) = line.split_once(": ").unwrap();
            let from = resolve(&mut graph, &mut names, &mut indices, from);
            for to in to.split_ascii_whitespace() {
                let to = resolve(&mut graph, &mut names, &mut indices, to);
                graph[from].push(to);
                graph[to].push(from);
            }
        }
        graph
    };

    fn cut_pairs(graph: &Vec<Vec<usize>>) -> Vec<((usize, usize), (usize, usize))> {
        #[derive(Clone, Copy)]
        struct CutPath(usize, usize, usize, usize);

        #[derive(Clone, Copy)]
        struct VertexData {
            nd: u32,
            low: (u32, usize),
            second_low: (u32, usize),
            to_low: usize,
            parent: usize,
            dfs: u32,
        }

        fn find_cut_pairs(
            graph: &[Vec<usize>],
            data: &mut [Option<VertexData>],
            stacks: &mut [Vec<CutPath>],
            dfs: &mut u32,
            cut_pairs: &mut Vec<((usize, usize), (usize, usize))>,
            v: usize,
            parent: usize,
        ) {
            data[v] = Some(VertexData {
                nd: 1,
                low: (*dfs, v),
                second_low: (*dfs, v),
                to_low: usize::MAX,
                parent,
                dfs: *dfs,
            });
            *dfs += 1;
            for &w in &graph[v] {
                match data[w] {
                    None => {
                        find_cut_pairs(graph, data, stacks, dfs, cut_pairs, w, v);
                        if stacks[w].last().is_some_and(|&CutPath(_, _, _, q)| w == q) {
                            let CutPath(x, y, p, _) = stacks[w].pop().unwrap();
                            cut_pairs.push(((x, y), (v, w)));
                            if v != p {
                                stacks[w].push(CutPath(x, y, p, v));
                            }
                        }
                        let w_data = data[w].unwrap();
                        let v_data = &mut data[v].as_mut().unwrap();
                        v_data.nd += w_data.nd;
                        if w_data.low.0 < v_data.low.0 {
                            v_data.second_low = v_data.low;
                            v_data.low = w_data.low;
                            stacks[v] = stacks[w].clone();
                            v_data.to_low = w;
                        } else if w_data.low.0 < v_data.second_low.0 {
                            v_data.second_low = w_data.low;
                            stacks[w].clear();
                        }
                    }
                    Some(w_data) => {
                        let v_data = &mut data[v].as_mut().unwrap();
                        if v_data.parent != w && w_data.dfs < v_data.dfs {
                            if w_data.dfs <= v_data.low.0 {
                                v_data.second_low = v_data.low;
                                v_data.low = (w_data.dfs, w);
                                stacks[v].clear();
                                v_data.to_low = w;
                            } else if w_data.dfs < v_data.second_low.0 {
                                v_data.second_low = (w_data.dfs, w);
                            }
                        }
                    }
                }
            }
            let v_data = data[v].unwrap();
            match stacks[v].last() {
                None => {
                    if v_data.second_low.0 > v_data.low.0 {
                        stacks[v].push(CutPath(v, v_data.to_low, v_data.low.1, v_data.second_low.1))
                    }
                }
                Some(&CutPath(_, _, _, q)) => {
                    if v_data.second_low.0 > data[q].unwrap().dfs {
                        stacks[v].push(CutPath(v, v_data.to_low, q, v_data.second_low.1))
                    } else {
                        while stacks[v].last().is_some_and(|&CutPath(_, _, p, _)| {
                            v_data.second_low.0 <= data[p].unwrap().dfs
                        }) {
                            stacks[v].pop();
                        }
                        if stacks[v].last().is_some_and(|&CutPath(_, _, _, q)| {
                            v_data.second_low.0 < data[q].unwrap().dfs
                        }) {
                            let CutPath(x, y, p, _) = stacks[v].pop().unwrap();
                            stacks[v].push(CutPath(x, y, p, v_data.second_low.1))
                        }
                    }
                }
            }
            for &u in &graph[v] {
                let u_data = data[u].unwrap();
                if u_data.parent != v && v_data.dfs < u_data.dfs {
                    while stacks[v].last().is_some_and(|&CutPath(x, y, _, _)| {
                        let y_data = data[y].as_ref().unwrap();
                        y_data.parent == x
                            && y_data.dfs <= u_data.dfs
                            && u_data.dfs <= y_data.dfs + y_data.nd - 1
                    }) {
                        stacks[v].pop();
                    }
                }
            }
        }
        let mut data = graph.iter().map(|_| None).collect_vec();
        let mut stacks = graph.iter().map(|_| Vec::new()).collect_vec();
        let mut cut_pairs = Vec::new();
        find_cut_pairs(
            &graph,
            &mut data[..],
            &mut stacks,
            &mut 0,
            &mut cut_pairs,
            0,
            0,
        );
        cut_pairs
    }

    fn connected_components(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        fn dfs(
            graph: &Vec<Vec<usize>>,
            remaining: &mut HashSet<usize>,
            component: &mut Vec<usize>,
            v: usize,
        ) {
            if remaining.remove(&v) {
                component.push(v);
                for &u in &graph[v] {
                    dfs(graph, remaining, component, u);
                }
            }
        }

        let mut remaining: HashSet<usize> = (0..graph.len()).collect();
        let mut components = Vec::new();
        while let Some(&v) = remaining.iter().next() {
            let mut component = Vec::new();
            dfs(graph, &mut remaining, &mut component, v);
            components.push(component);
        }
        components
    }

    let part1 = graph
        .iter()
        .enumerate()
        .flat_map(|(i, js)| js.iter().filter_map(move |&j| (i < j).then_some((i, j))))
        .find_map(|(v1, v2)| {
            let mut graph = graph.clone();
            graph[v1].retain(|&i| i != v2);
            graph[v2].retain(|&i| i != v1);

            for ((v1, v2), (v3, v4)) in cut_pairs(&graph) {
                graph[v1].retain(|&i| i != v2);
                graph[v2].retain(|&i| i != v1);
                graph[v3].retain(|&i| i != v4);
                graph[v4].retain(|&i| i != v3);
            }

            match &connected_components(&graph)[..] {
                [a, b] => Some(a.len() * b.len()),
                _ => None,
            }
        })
        .unwrap();
    println!("{}", part1);
}
