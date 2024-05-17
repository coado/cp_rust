use anyhow::{Context, Result};

fn dfs(graph: &Vec<Vec<i32>>, v: usize, visited: &mut Vec<bool>, stack: &mut Vec<i32>) {
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

#[cfg(test)]
mod test {
    use super::topological_sort;

    #[test]
    fn test_topological_sort() {
        let graph = vec![vec![], vec![], vec![3], vec![1], vec![0, 1], vec![2, 0]];
        let res = topological_sort(graph).unwrap();

        println!("{:?}", res);

        assert_eq!(res, vec![5, 4, 2, 3, 1, 0]);
    }
}
