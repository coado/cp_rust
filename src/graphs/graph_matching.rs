use anyhow::{bail, Context, Result};
use std::collections::{HashMap, VecDeque};

pub fn is_bipartite(al: &Vec<&Vec<usize>>) -> Result<bool> {
    let mut is_correct = true;
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut color = vec![std::usize::MAX; al.len()];
    color[0] = 0;
    deque.push_back(0);

    while !deque.is_empty() && is_correct {
        let u: usize = deque.pop_front().context("The deque is empty")?;
        for v in al[u] {
            if color[*v] == std::usize::MAX {
                color[*v] = 1 - color[u];
                deque.push_front(*v);
            } else if color[*v] == color[u] {
                is_correct = false;
                break;
            }
        }
    }

    Ok(is_correct)
}

/// Kuhn's algorithm for finding Maximum Cardinality Bipartite Matching (MCBM) that runs in O(VE) time complexity.
/// Uses Berge's lemma of augmentic paths
///
/// # Arguments
///
/// * al - adjacency list
/// * u - vector of left side vertecies ids
/// * v - vector of right side vertecies ids
///
/// ```
/// # let al: Vec<Vec<usize>> = vec![vec![2, 3], vec![2], vec![0, 1], vec![0]];
/// # let u = vec![0, 1];
/// # let v = vec![2, 3];
/// # let res = kuhn_algorithm(&al.iter().collect(), u, v).unwrap();
/// ```
pub fn kuhn_algorithm(
    al: &Vec<&Vec<usize>>,
    u: Vec<usize>,
    v: Vec<usize>,
) -> Result<HashMap<usize, i32>> {
    if al.len() != u.len() + v.len() {
        bail!("");
    }

    let mut mt: HashMap<usize, i32> = HashMap::new();
    let mut visited: HashMap<usize, bool> = HashMap::new();

    for i in v {
        mt.insert(i, -1);
    }

    fn dfs(
        u: usize,
        al: &Vec<&Vec<usize>>,
        mt: &mut HashMap<usize, i32>,
        visited: &mut HashMap<usize, bool>,
    ) -> bool {
        if visited.get(&u).is_some_and(|x| *x == true) || visited.get(&u).is_none() {
            return false;
        }

        visited.insert(u, true);

        for v in al[u] {
            if let Some(matched) = mt.get(&(*v as usize)) {
                if *matched == -1 || dfs(*matched as usize, al, mt, visited) {
                    mt.insert(*v as usize, u as i32);
                    return true;
                }
            }
        }

        false
    }

    for cu in &u {
        u.iter().for_each(|el| {
            visited.insert(*el, false);
        });
        dfs(*cu, al, &mut mt, &mut visited);
    }

    Ok(mt)
}

#[cfg(test)]
mod tests {
    use super::{is_bipartite, kuhn_algorithm};

    #[test]
    fn test_graph_matching_should_succeed() {
        let al: Vec<Vec<usize>> = vec![
            vec![3, 4, 5],
            vec![3, 4, 5],
            vec![3, 4, 5],
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![0, 1, 2],
        ];

        let res = is_bipartite(&al.iter().collect()).unwrap();
        assert_eq!(res, true);
    }

    #[test]
    fn test_graph_matching_should_not_succeed() {
        let al: Vec<Vec<usize>> = vec![
            vec![1, 4, 5],
            vec![3, 4, 5],
            vec![3, 4, 5],
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![0, 1, 2],
        ];

        let res = is_bipartite(&al.iter().collect()).unwrap();
        assert_eq!(res, false);
    }

    #[test]
    fn test_kuhn_algorithm_min_example() {
        let al: Vec<Vec<usize>> = vec![vec![2, 3], vec![2], vec![0, 1], vec![0]];

        let u = vec![0, 1];
        let v = vec![2, 3];

        let res = kuhn_algorithm(&al.iter().collect(), u, v).unwrap();

        let vertex2_matching = res.get(&2);
        let vertex3_matching = res.get(&3);

        assert_eq!(vertex2_matching, Some(&1));
        assert_eq!(vertex3_matching, Some(&0));
    }

    #[test]
    fn test_kuhn_algorithm() {
        let al: Vec<Vec<usize>> = vec![
            vec![3, 4, 5],
            vec![3, 4, 5],
            vec![3, 4, 5],
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![0, 1, 2],
        ];

        let u = vec![0, 1, 2];
        let v = vec![3, 4, 5];

        let res = kuhn_algorithm(&al.iter().collect(), u, v);

        println!("{:?}", res);
    }

    #[test]
    fn test_kuhn_algorithm2() {
        let al: Vec<Vec<usize>> = vec![
            vec![4, 5, 6],
            vec![4],
            vec![6, 7],
            vec![5],
            vec![0, 1],
            vec![0, 3],
            vec![0, 2],
            vec![2],
        ];

        let u = vec![0, 1, 2, 3];
        let v = vec![4, 5, 6, 7];

        let res = kuhn_algorithm(&al.iter().collect(), u, v).unwrap();

        let vertex4_matching = res.get(&4);
        let vertex5_matching = res.get(&5);
        let vertex6_matching = res.get(&6);
        let vertex7_matching = res.get(&7);

        assert_eq!(vertex4_matching, Some(&1));
        assert_eq!(vertex5_matching, Some(&3));
        assert_eq!(vertex6_matching, Some(&0));
        assert_eq!(vertex7_matching, Some(&2));

        println!("{:?}", res);
    }
}
