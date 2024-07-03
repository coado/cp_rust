#[derive(Clone)]
enum VertexState {
    Unvisited,
    Visited,
    Explored,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CycleCheckResult {
    NoCycle,
    BackEdge,
    CrossEdge,
}

fn dfs(al: &[Vec<usize>], v_states: &mut Vec<VertexState>, u: usize) -> CycleCheckResult {
    v_states[u] = VertexState::Explored;

    for v in al[u].iter() {
        let res = match v_states[*v] {
            VertexState::Unvisited => dfs(al, v_states, *v),
            VertexState::Explored => CycleCheckResult::BackEdge,
            VertexState::Visited => CycleCheckResult::CrossEdge,
        };

        if res != CycleCheckResult::NoCycle {
            return res;
        }
    }

    v_states[u] = VertexState::Visited;
    CycleCheckResult::NoCycle
}

pub fn cycle_check(al: &[Vec<usize>]) -> CycleCheckResult {
    let mut v_states: Vec<VertexState> = vec![VertexState::Unvisited; al.len()];

    for i in 0..al.len() {
        let res: CycleCheckResult = match v_states[i] {
            VertexState::Unvisited => dfs(al, &mut v_states, i),
            _ => CycleCheckResult::NoCycle,
        };

        if res != CycleCheckResult::NoCycle {
            return res;
        }
    }

    CycleCheckResult::NoCycle
}

#[cfg(test)]
mod tests {
    use super::{cycle_check, CycleCheckResult};

    #[test]
    fn test_cycle_check() {
        let al = vec![vec![1], vec![2], vec![3], vec![1, 4], vec![]];
        let res = cycle_check(&al);

        assert_eq!(res, CycleCheckResult::BackEdge);

        let al = vec![vec![1], vec![2, 3], vec![3], vec![4], vec![]];
        let res = cycle_check(&al);

        assert_eq!(res, CycleCheckResult::CrossEdge);

        let al = vec![vec![1], vec![2], vec![3], vec![4], vec![]];
        let res = cycle_check(&al);

        assert_eq!(res, CycleCheckResult::NoCycle);
    }
}
