use anyhow::{Context, Result};

pub fn dfs(graph: &Vec<Vec<i32>>, v: usize, visited: &mut Vec<bool>, stack: &mut Vec<i32>) {
    visited[v] = true;

    for el in graph[v].iter() {
        let i = *el as usize;
        if !visited[i] {
            dfs(graph, i, visited, stack);
        }
    }

    stack.push(v as i32);
}

pub fn topological_sort(graph: Vec<Vec<i32>>) -> Result<Vec<i32>> {
    let v = graph.len();
    let mut visited = vec![false; v];

    let mut stack: Vec<i32> = Vec::new();

    for i in 0..v {
        if visited[i] {
            continue;
        }

        dfs(&graph, i, &mut visited, &mut stack);
    }

    let mut res = Vec::new();
    while !stack.is_empty() {
        let el = stack.pop().context("stack is empty")?;
        res.push(el);
    }

    Ok(res)
}
