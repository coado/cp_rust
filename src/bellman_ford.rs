use anyhow::{bail, Result};
use std;

pub fn bellman_ford(graph: Vec<Vec<Option<i32>>>, src: usize, vertexes: usize) -> Result<Vec<i32>> {
    let mut dist = vec![std::i32::MAX; vertexes];
    dist[src] = 0;

    for _ in 1..vertexes - 1 {
        for u in 0..vertexes {
            for v in 0..vertexes {
                if u == v {
                    continue;
                }

                match graph[u][v] {
                    None => continue,
                    Some(w) => {
                        if dist[u] != std::i32::MAX && dist[u] + w < dist[v] {
                            dist[v] = w + dist[u];
                        }
                    }
                }
            }
        }
    }

    for _ in 1..vertexes - 1 {
        for u in 0..vertexes {
            for v in 0..vertexes {
                if u == v {
                    continue;
                }

                match graph[u][v] {
                    None => continue,
                    Some(w) => {
                        if dist[u] != std::i32::MAX && dist[u] + w < dist[v] {
                            bail!("Graph contains negative weight cycle");
                        }
                    }
                }
            }
        }
    }

    Ok(dist)
}
