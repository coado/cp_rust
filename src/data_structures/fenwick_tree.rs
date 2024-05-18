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

        for (i, val) in nums.iter().enumerate() {
            fenwick_tree.tree[i] += val;
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

pub struct FenwickTree2D {
    n: usize,
    m: usize,
    tree: Vec<Vec<i32>>,
}

impl FenwickTree2D {
    pub fn new(n: usize, m: usize) -> Self {
        FenwickTree2D {
            n,
            m,
            tree: vec![vec![0; n]; m],
        }
    }

    pub fn sum(&self, y: i32, x: i32) -> i32 {
        let mut y = y;
        let mut res = 0;

        while y >= 0 {
            let mut x = x;
            while x >= 0 {
                res += self.tree[y as usize][x as usize];
                x = (x & (x + 1)) - 1;
            }
            y = (y & (y + 1)) - 1;
        }

        res
    }

    pub fn range_sum(&self, y1: i32, x1: i32, y2: i32, x2: i32) -> i32 {
        self.sum(y2, x2) - self.sum(y1 - 1, x2) - self.sum(y2, x1 - 1)
    }

    pub fn add(&mut self, x: usize, y: usize, val: i32) {
        let mut i = y;

        while i < self.m {
            let mut j = x;
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
    use std::cmp::min;

    use super::{FenwickTree, FenwickTree2D, MinFenwickTree};
    use rand::prelude::*;

    #[test]
    fn test_fenwick_tree() {
        let n = 1000;
        let mut rng = thread_rng();
        let mut nums: Vec<i32> = vec![0; n];

        let mut fenwick_tree = FenwickTree::new(n);
        let mut sum = 0;

        for i in 0..n {
            nums[i] = rng.gen_range(0..1000);
            fenwick_tree.add(i, nums[i]);

            sum += nums[i];
            assert_eq!(fenwick_tree.sum(i as i32), sum);
        }
    }

    #[test]
    fn test_fenwick_tree_ranges() {
        let n = 1000;
        let mut rng = thread_rng();
        let mut prefix_sum: Vec<i32> = vec![0; n];

        let mut fenwick_tree = FenwickTree::new(n);

        for i in 1..n {
            let val = rng.gen_range(0..1000);
            prefix_sum[i] = prefix_sum[i - 1] + val;
            fenwick_tree.add(i, val);
        }

        for _ in 0..1000 {
            let v1 = rng.gen_range(1..n);
            let v2 = rng.gen_range(1..n);

            let l = v1.min(v2);
            let r = v1.max(v2);

            let sum = fenwick_tree.range_sum(l as i32, r as i32);
            let expected = prefix_sum[r] - prefix_sum[l - 1];

            assert_eq!(sum, expected);
        }
    }

    #[test]
    fn test_min_fenwick_tree() {
        let n = 1000;
        let mut rng = thread_rng();
        let mut nums: Vec<i32> = vec![0; n];

        let mut min_fenwick_tree = MinFenwickTree::new(n);

        for i in 0..n {
            nums[i] = rng.gen_range(0..1000);
            min_fenwick_tree.update(i, nums[i]);
        }

        let mut min_val = i32::MAX;
        for i in 0..n {
            min_val = min(nums[i], min_val);
            assert_eq!(min_fenwick_tree.get_min(i as i32), min_val);
        }
    }

    #[test]
    fn test_fenwick_tree_2_d() {
        let n = 100;
        let m = 100;
        let mut rng = thread_rng();
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; n]; m];

        let mut fenwick_tree_2d = FenwickTree2D::new(n, m);

        for i in 0..m {
            for j in 0..n {
                matrix[i][j] = rng.gen_range(0..10);
                fenwick_tree_2d.add(j, i, matrix[i][j]);
            }
        }

        for i in 0..m {
            for j in 0..n {
                let sum = fenwick_tree_2d.sum(i as i32, j as i32);
                let expected = matrix[0..=i]
                    .iter()
                    .map(|row| row[0..=j].iter().sum::<i32>())
                    .sum::<i32>();

                assert_eq!(sum, expected);
            }
        }
    }

    #[test]
    fn test_fenwick_tree_2_d_ranges() {
        let n = 5;
        let m = 5;
        let mut rng = thread_rng();
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; n]; m];

        let mut fenwick_tree_2d = FenwickTree2D::new(n, m);

        for i in 0..m {
            for j in 0..n {
                matrix[i][j] = rng.gen_range(0..10);
                fenwick_tree_2d.add(j, i, matrix[i][j]);
            }
        }

        for _ in 0..1000 {
            let xx1 = rng.gen_range(0..n);
            let yy1 = rng.gen_range(0..m);
            let xx2 = rng.gen_range(0..n);
            let yy2 = rng.gen_range(0..m);

            let x1 = xx1.min(xx2);
            let y1 = yy1.min(yy2);
            let x2 = xx1.max(xx2);
            let y2 = yy1.max(yy2);

            let sum = fenwick_tree_2d.range_sum(y1 as i32, x1 as i32, y2 as i32, x2 as i32);
            let expected = matrix[y1..=y2]
                .iter()
                .map(|row| row[x1..=x2].iter().sum::<i32>())
                .sum::<i32>();
            println!("sum: {}, expected: {}", sum, expected);
            assert_eq!(sum, expected);
        }
    }
}
