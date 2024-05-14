use std::cmp::min;

pub struct FenwickTree {
    n: usize,
    tree: Vec<i32>,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            tree: vec![0; n],
        }
    }

    pub fn add(&mut self, idx: usize, delta: i32) {
        let mut idx = idx;

        while idx < self.n {
            self.tree[idx] += delta;
            idx = idx | (idx + 1);
        }
    }

    pub fn sum(&self, r: i32) -> i32 {
        let mut r = r;
        let mut ret = 0;

        while r >= 0 {
            ret += self.tree[r as usize];
            r = (r & (r + 1)) - 1;
        }

        ret
    }

    pub fn range_sum(&self, l: i32, r: i32) -> i32 {
        self.sum(r) - self.sum(l - 1)
    }
}

impl From<Vec<i32>> for FenwickTree {
    fn from(nums: Vec<i32>) -> Self {
        let mut fenwick_tree = FenwickTree::new(nums.len());

        for i in 0..nums.len() {
            fenwick_tree.tree[i] += nums[i];
            let r = i | (i + 1);
            fenwick_tree.tree[r] += fenwick_tree.tree[i];
        }

        fenwick_tree
    }
}

pub struct MinFenwickTree {
    n: usize,
    tree: Vec<i32>,
}

impl MinFenwickTree {
    pub fn new(n: usize) -> Self {
        MinFenwickTree {
            n,
            tree: vec![i32::MAX; n],
        }
    }

    pub fn get_min(&self, r: i32) -> i32 {
        let mut r = r;
        let mut ret = i32::MAX;

        while r >= 0 {
            ret = min(ret, self.tree[r as usize]);
            r = (r & (r + 1)) - 1
        }

        ret
    }

    pub fn update(&mut self, idx: usize, val: i32) {
        let mut idx = idx;

        while idx < self.n {
            self.tree[idx] = min(self.tree[idx], val);
            idx = idx | (idx + 1);
        }
    }
}

pub struct FenwictTree2D {
    n: usize,
    m: usize,
    tree: Vec<Vec<i32>>,
}

impl FenwictTree2D {
    pub fn new(n: usize, m: usize) -> Self {
        FenwictTree2D {
            n,
            m,
            tree: vec![vec![0; n]; m],
        }
    }

    pub fn sum(&self, l: i32, r: i32) -> i32 {
        let mut l = l;
        let mut r = r;
        let mut ret = 0;

        while l >= 0 {
            while r >= 0 {
                ret += self.tree[l as usize][r as usize];
                r = (r & (r + 1)) - 1;
            }
            l = (l & (l + 1)) - 1;
        }

        ret
    }

    pub fn range_sum(&self, l1: i32, r1: i32, l2: i32, r2: i32) -> i32 {
        self.sum(l2, r2) - self.sum(l1 - 1, r1 - 1)
    }

    pub fn add(&mut self, x: usize, y: usize, val: i32) {
        let mut i = y;
        let mut j = x;

        while i < self.m {
            while j < self.n {
                self.tree[i][j] += val;
                j |= j + 1;
            }

            i |= i + 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::FenwickTree;
    use crate::utils::print_vector;

    #[test]
    fn test_fenwick_tree() {
        let nums = vec![1, 2, 3, 4, 5];

        let ft: FenwickTree = nums.into();
        print_vector(ft.tree);
    }
}
