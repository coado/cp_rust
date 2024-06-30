#[derive(Clone, PartialEq, Debug, Default)]
pub struct ArticulationPointAndBridge {
    dfs_num: Vec<Option<usize>>,
    dfs_low: Vec<Option<usize>>,
    articulation_vertex: Vec<bool>,
    dfs_parent: Vec<Option<usize>>,
    dfs_counter: usize,
    dfs_root: usize,
    root_children: usize,
}

impl ArticulationPointAndBridge {
    pub fn new() -> Self {
        Self {
            dfs_num: vec![None; 0],
            dfs_low: vec![None; 0],
            articulation_vertex: vec![false; 0],
            dfs_parent: vec![None; 0],
            dfs_counter: 0,
            dfs_root: 0,
            root_children: 0,
        }
    }

    pub fn articulation_point_and_bridge(&mut self, al: &[Vec<usize>]) -> Vec<usize> {
        self.dfs_num = vec![None; al.len()];
        self.dfs_low = vec![None; al.len()];
        self.articulation_vertex = vec![false; al.len()];
        self.dfs_parent = vec![None; al.len()];

        for u in 0..al.len() {
            if self.dfs_num[u].is_none() {
                self.dfs_root = u;
                self.root_children = 0;
                self.dfs(al, u);
                self.articulation_vertex[u] = self.root_children > 1;
            }
        }

        let mut res = Vec::new();
        for i in 0..self.articulation_vertex.len() {
            if self.articulation_vertex[i] {
                res.push(i);
            }
        }

        res
    }

    fn dfs(&mut self, al: &[Vec<usize>], u: usize) {
        self.dfs_num[u] = Some(self.dfs_counter);
        self.dfs_counter += 1;
        self.dfs_low[u] = self.dfs_num[u];

        for v in al[u].iter() {
            if self.dfs_num[*v].is_none() {
                self.dfs_parent[*v] = Some(u);
                if u == self.dfs_root {
                    self.root_children += 1;
                }

                self.dfs(al, *v);

                if self.dfs_low[*v].unwrap() >= self.dfs_num[u].unwrap() {
                    self.articulation_vertex[u] = true;
                }

                self.dfs_low[u] = Some(self.dfs_low[u].unwrap().min(self.dfs_low[*v].unwrap()));
            } else if self.dfs_parent[u] != Some(*v) {
                let cur_low = self.dfs_low[u].unwrap();
                let v_low = self.dfs_num[*v].unwrap();
                self.dfs_low[u] = Some(cur_low.min(v_low));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArticulationPointAndBridge;

    #[test]
    fn test_articulation_point_and_bridge() {
        let al = vec![vec![1, 2], vec![0, 2], vec![0, 1, 3], vec![2, 4], vec![3]];
        let mut apb = ArticulationPointAndBridge::new();
        let res = apb.articulation_point_and_bridge(&al);

        assert_eq!(res, vec![2, 3]);
    }
}
