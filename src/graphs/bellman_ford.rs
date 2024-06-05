use anyhow::{bail, Result};
use num::{Bounded, Zero};

pub fn bellman_ford<T>(graph: Vec<Vec<Option<T>>>, src: usize, vertices: usize) -> Result<Vec<T>>
where
    T: Bounded + Copy + Zero + PartialEq + PartialOrd,
{
    let mut dist = vec![T::max_value(); vertices];
    dist[src] = T::zero();

    for _ in 1..vertices - 1 {
        for u in 0..vertices {
            for v in 0..vertices {
                if u == v {
                    continue;
                }

                match graph[u][v] {
                    None => continue,
                    Some(w) => {
                        if dist[u] != T::max_value() && dist[u] + w < dist[v] {
                            dist[v] = w + dist[u];
                        }
                    }
                }
            }
        }
    }

    for _ in 1..vertices - 1 {
        for u in 0..vertices {
            for v in 0..vertices {
                if u == v {
                    continue;
                }

                match graph[u][v] {
                    None => continue,
                    Some(w) => {
                        if dist[u] != T::max_value() && dist[u] + w < dist[v] {
                            bail!("Graph contains negative weight cycle");
                        }
                    }
                }
            }
        }
    }

    Ok(dist)
}

#[cfg(test)]
mod test {
    use super::bellman_ford;

    #[test]
    #[should_panic]
    fn test_bellman_ford() {
        let graph = vec![
            vec![None, Some(5), None, None, None, None],
            vec![None, None, Some(1), Some(2), None, None],
            vec![None, None, None, None, Some(1), None],
            vec![None, None, None, None, None, Some(2)],
            vec![None, None, None, Some(-1), None, None],
            vec![None, None, None, None, Some(-3), None],
        ];
        let _ = bellman_ford(graph, 0, 6).unwrap();

        // assert_eq!(res, vec![0, 5, 6, 7, 5, 2]);
    }
}
