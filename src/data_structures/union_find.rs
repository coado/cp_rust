#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    set_size: Vec<usize>,
    num_sets: usize,
}

impl UnionFind {
    pub fn new() -> Self {
        UnionFind {
            parent: Vec::new(),
            rank: Vec::new(),
            set_size: Vec::new(),
            num_sets: 0,
        }
    }

    pub fn with_capacity(n: usize) -> Self {
        UnionFind {
            parent: vec![0; n].iter().enumerate().map(|(i, _)| i).collect(),
            rank: vec![0; n],
            set_size: vec![1; n],
            num_sets: n,
        }
    }

    pub fn find_set(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        self.parent[i] = self.find_set(self.parent[i]);
        self.parent[i]
    }

    pub fn is_same_set(&mut self, i: usize, j: usize) -> bool {
        self.find_set(i) == self.find_set(j)
    }

    pub fn union_set(&mut self, i: usize, j: usize) -> bool {
        if self.is_same_set(i, j) {
            return false;
        }

        let mut x = self.find_set(i);
        let mut y = self.find_set(j);

        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[x] = y;
        self.set_size[y] += self.set_size[x];

        if self.rank[x] == self.rank[y] {
            self.rank[y] += 1;
        }

        self.num_sets -= 1;
        true
    }

    pub fn size_of_set(&mut self, i: usize) -> usize {
        let x = self.find_set(i);
        self.set_size[x]
    }

    pub fn num_sets(&self) -> usize {
        self.num_sets
    }

    pub fn set_size(&self) -> usize {
        self.set_size.len()
    }
}

impl Default for UnionFind {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::with_capacity(5);

        assert_eq!(uf.num_sets(), 5);
        assert_eq!(uf.size_of_set(0), 1);
        assert_eq!(uf.size_of_set(1), 1);
        assert_eq!(uf.size_of_set(2), 1);
        assert_eq!(uf.size_of_set(3), 1);
        assert_eq!(uf.size_of_set(4), 1);

        assert_eq!(uf.union_set(0, 1), true);
        assert_eq!(uf.num_sets(), 4);
        assert_eq!(uf.size_of_set(0), 2);
        assert_eq!(uf.size_of_set(1), 2);
        assert_eq!(uf.size_of_set(2), 1);
        assert_eq!(uf.size_of_set(3), 1);
        assert_eq!(uf.size_of_set(4), 1);

        assert_eq!(uf.union_set(2, 3), true);
        assert_eq!(uf.num_sets(), 3);
        assert_eq!(uf.size_of_set(0), 2);
        assert_eq!(uf.size_of_set(1), 2);
        assert_eq!(uf.size_of_set(2), 2);
        assert_eq!(uf.size_of_set(3), 2);
        assert_eq!(uf.size_of_set(4), 1);

        assert_eq!(uf.union_set(0, 2), true);
        assert_eq!(uf.num_sets(), 2);
        assert_eq!(uf.size_of_set(0), 4);
        assert_eq!(uf.size_of_set(1), 4);
        assert_eq!(uf.size_of_set(2), 4);
        assert_eq!(uf.size_of_set(3), 4);
        assert_eq!(uf.size_of_set(4), 1);

        assert_eq!(uf.union_set(0, 4), true);
        assert_eq!(uf.num_sets(), 1);
        assert_eq!(uf.size_of_set(0), 5);
        assert_eq!(uf.size_of_set(1), 5);
        assert_eq!(uf.size_of_set(2), 5);
        assert_eq!(uf.size_of_set(3), 5);
        assert_eq!(uf.size_of_set(4), 5);
    }
}
