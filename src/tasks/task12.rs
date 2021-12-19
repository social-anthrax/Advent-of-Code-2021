use std::collections::HashMap;

use crate::task_handler::get_task;

type Graph = HashMap<String, Vec<String>>;
pub fn tasks() {
    let mut adjacency_list = HashMap::new();

    get_task(12).lines().for_each(|line| {
        let (a, b) = line.split_once('-').expect("Failed to split");
        adjacency_list
            .entry(a.to_string())
            .or_insert_with(Vec::new)
            .push(b.to_string());

        adjacency_list
            .entry(b.to_string())
            .or_insert_with(Vec::new)
            .push(a.to_string());
    });

    let mut visited: HashMap<String, bool> = adjacency_list
        .keys()
        .cloned()
        .zip(std::iter::repeat(false))
        .collect();

    let start = std::time::Instant::now();

    println!(
        "Task12.1 {}",
        count_paths(&adjacency_list, "start", "end", &mut visited)
    );

    println!("done in {}us!", start.elapsed().as_micros());

    let mut visited: HashMap<String, bool> = adjacency_list
        .keys()
        .cloned()
        .zip(std::iter::repeat(false))
        .collect();

    let start = std::time::Instant::now();

    println!(
        "Task12.2 {}. Done in {}us!",
        count_paths_double(&adjacency_list, "start", "end", &mut visited, None),
        start.elapsed().as_micros()
    );
}

fn count_paths_double(
    graph: &Graph,
    from: &str,
    to: &str,
    visited: &mut HashMap<String, bool>,
    double_used: Option<&str>,
) -> usize {
    if from == to {
        if let Some(used) = double_used {
            return if visited[used] { 1 } else { 0 };
        }
        return 1;
    }

    let mut counter = 0;

    // If `from` is not a large cave, mark it as visited
    if from.to_lowercase() == *from {
        *visited.get_mut(from).unwrap() = true;
    }

    for child in graph.get(from).unwrap().clone() {
        if !visited[&child] {
            counter += count_paths_double(graph, &child, to, visited, double_used);
        }
    }

    // after we have found all the paths from a small cave, mark it as not visited so other paths can use it again.

    if from.to_uppercase() != *from {
        *visited.get_mut(from).unwrap() = false;
    }

    // only count the paths if we use the node twice.
    if double_used.is_none() && from.to_uppercase() != from && !["start", "end"].contains(&from) {
        for child in graph[from].clone().iter() {
            if !visited[child] {
                counter += count_paths_double(graph, child, to, visited, Some(from))
            }
        }
    }

    counter
}

fn count_paths(graph: &Graph, from: &str, to: &str, visited: &mut HashMap<String, bool>) -> usize {
    if from == to {
        return 1;
    }

    let mut counter = 0;

    // If `from` is not a large cave, mark it as visited
    if from.to_uppercase() != *from {
        *visited.get_mut(from).unwrap() = true;
    }

    for child in graph.get(from).unwrap().clone() {
        if !visited.get(&child).unwrap() {
            counter += count_paths(graph, &child, to, visited);
        }
    }

    // after we have found all the paths from a small cave, mark it as not visited so other paths can use it again.
    if from.to_uppercase() != *from {
        *visited.get_mut(from).unwrap() = false;
    }

    counter
}
