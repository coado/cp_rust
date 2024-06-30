use num::{Bounded, Zero};
use std::cmp::min;
use std::ops::{AddAssign, Sub};

#[derive(Debug, Clone, Default)]
pub struct FenwickTree<T>
where
    T: Copy + AddAssign + Sub<Output = T> + Zero,
{
    n: usize,
    tree: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Copy + AddAssign + Sub<Output = T> + Zero,
{
    pub fn new(n: usize, default: T) -> Self {
        FenwickTree {
            n,
            tree: vec![default; n],
        }
    }

    pub fn add(&mut self, idx: usize, delta: T) {
        let mut idx = idx;

        while idx < self.n {
            self.tree[idx] += delta;
            idx = idx | (idx + 1);
        }
    }

    pub fn sum(&self, r: i64) -> T {
        let mut r = r;
        let mut ret = T::zero();

        while r >= 0 {
            ret += self.tree[r as usize];
            r = (r & (r + 1)) - 1;
        }

        ret
    }

    pub fn range_sum(&self, l: i64, r: i64) -> T {
        self.sum(r) - self.sum(l - 1)
    }
}

impl From<Vec<i32>> for FenwickTree<i32> {
    fn from(nums: Vec<i32>) -> Self {
        let mut fenwick_tree = FenwickTree::new(nums.len(), 0);

        for (i, val) in nums.iter().enumerate() {
            fenwick_tree.tree[i] += val;
            let r = i | (i + 1);
            fenwick_tree.tree[r] += fenwick_tree.tree[i];
        }

        fenwick_tree
    }
}

impl From<Vec<i64>> for FenwickTree<i64> {
    fn from(nums: Vec<i64>) -> Self {
        let mut fenwick_tree = FenwickTree::new(nums.len(), 0);

        for (i, val) in nums.iter().enumerate() {
            fenwick_tree.tree[i] += val;
            let r = i | (i + 1);
            fenwick_tree.tree[r] += fenwick_tree.tree[i];
        }

        fenwick_tree
    }
}

impl From<Vec<i16>> for FenwickTree<i16> {
    fn from(nums: Vec<i16>) -> Self {
        let mut fenwick_tree = FenwickTree::new(nums.len(), 0);

        for (i, val) in nums.iter().enumerate() {
            fenwick_tree.tree[i] += val;
            let r = i | (i + 1);
            fenwick_tree.tree[r] += fenwick_tree.tree[i];
        }

        fenwick_tree
    }
}

impl From<Vec<i8>> for FenwickTree<i8> {
    fn from(nums: Vec<i8>) -> Self {
        let mut fenwick_tree = FenwickTree::new(nums.len(), 0);

        for (i, val) in nums.iter().enumerate() {
            fenwick_tree.tree[i] += val;
            let r = i | (i + 1);
            fenwick_tree.tree[r] += fenwick_tree.tree[i];
        }

        fenwick_tree
    }
}

#[derive(Debug, Clone, Default)]
pub struct MinFenwickTree<T>
where
    T: Copy + AddAssign + Sub<Output = T> + Zero + Bounded + Ord,
{
    n: usize,
    tree: Vec<T>,
}

impl<T> MinFenwickTree<T>
where
    T: Copy + AddAssign + Sub<Output = T> + Zero + Bounded + Ord,
{
    pub fn new(n: usize) -> Self {
        MinFenwickTree {
            n,
            tree: vec![T::max_value(); n],
        }
    }

    pub fn get_min(&self, r: i32) -> T {
        let mut r = r;
        let mut ret = T::max_value();

        while r >= 0 {
            ret = min(ret, self.tree[r as usize]);
            r = (r & (r + 1)) - 1
        }

        ret
    }

    pub fn update(&mut self, idx: usize, val: T) {
        let mut idx = idx;

        while idx < self.n {
            self.tree[idx] = min(self.tree[idx], val);
            idx = idx | (idx + 1);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FenwickTree2D<T>
where
    T: Copy + AddAssign + Sub<Output = T> + Zero,
{
    n: usize,
    m: usize,
    tree: Vec<Vec<T>>,
}

impl<T> FenwickTree2D<T>
where
    T: Copy + AddAssign + Sub<Output = T> + Zero,
{
    pub fn new(n: usize, m: usize) -> Self {
        FenwickTree2D {
            n,
            m,
            tree: vec![vec![T::zero(); n]; m],
        }
    }

    pub fn sum(&self, y: i32, x: i32) -> T {
        let mut y = y;
        let mut res = T::zero();

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

    pub fn range_sum(&self, y1: i32, x1: i32, y2: i32, x2: i32) -> T {
        self.sum(y2, x2) - self.sum(y1 - 1, x2) - self.sum(y2, x1 - 1) + self.sum(y1 - 1, x1 - 1)
    }

    pub fn add(&mut self, x: usize, y: usize, val: T) {
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

        let mut fenwick_tree = FenwickTree::new(n, 0);
        let mut sum = 0;

        for i in 0..n {
            nums[i] = rng.gen_range(0..1000);
            fenwick_tree.add(i, nums[i]);

            sum += nums[i];
            assert_eq!(fenwick_tree.sum(i as i64), sum);
        }
    }

    #[test]
    fn test_fenwick_tree_ranges() {
        let n = 1000;
        let mut rng = thread_rng();
        let mut prefix_sum: Vec<i32> = vec![0; n];

        let mut fenwick_tree = FenwickTree::new(n, 0);

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

            let sum = fenwick_tree.range_sum(l as i64, r as i64);
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
            assert_eq!(sum, expected);
        }
    }
}
