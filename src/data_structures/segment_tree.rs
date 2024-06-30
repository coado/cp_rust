use std::cmp::{max, min};

/// Segment Tree
/// O(log n) time complexity for range queries and updates.
/// The segment tree is a binary tree that stores the range of values in an array.
/// The root of the tree stores the range of values from 0 to n-1

pub struct SegmentTree<T: PartialEq + Copy> {
    n: usize,
    nums: Vec<T>,
    st: Vec<Option<T>>,
    lazy: Vec<Option<T>>,
    cnq: Box<dyn Fn(T, T) -> T>,
}

impl<T: PartialEq + Copy> SegmentTree<T> {
    #[inline]
    fn l(p: usize) -> usize {
        p << 1
    }

    #[inline]
    fn r(p: usize) -> usize {
        (p << 1) + 1
    }

    fn conquer(&self, a: Option<T>, b: Option<T>) -> Option<T> {
        match (a, b) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => Some((self.cnq)(x, y)),
        }
    }

    pub fn new(nums: Vec<T>, cnq: Box<dyn Fn(T, T) -> T>) -> Self {
        let n = nums.len();

        SegmentTree {
            n,
            nums,
            st: vec![None; 4 * n],
            lazy: vec![None; 4 * n],
            cnq,
        }
    }

    pub fn rmq(&mut self, i: usize, j: usize) -> Option<T> {
        self.rmq_internal(1, 0, self.n - 1, i, j)
    }

    pub fn build(&mut self) {
        self.build_internal(1, 0, self.n - 1);
    }

    pub fn update(&mut self, i: usize, j: usize, val: T) {
        self.update_internal(1, 0, self.n - 1, i, j, val);
    }

    fn update_internal(&mut self, p: usize, left: usize, right: usize, i: usize, j: usize, val: T) {
        self.propagate(p, left, right);
        if i > j {
            return;
        }

        if left >= i && right <= j {
            self.lazy[p] = Some(val);
            self.propagate(p, left, right);
        } else {
            let m = (right - left) / 2 + left;
            self.update_internal(Self::l(p), left, m, i, min(m, j), val);
            self.update_internal(Self::r(p), m + 1, right, max(i, m + 1), j, val);
            let l_subtree = if Option::is_some(&self.lazy[Self::l(p)]) {
                self.lazy[Self::l(p)].unwrap()
            } else {
                self.st[Self::l(p)].unwrap()
            };

            let r_subtree = if Option::is_some(&self.lazy[Self::r(p)]) {
                self.lazy[Self::r(p)].unwrap()
            } else {
                self.st[Self::r(p)].unwrap()
            };

            self.st[p] = if (self.cnq)(r_subtree, l_subtree) == l_subtree {
                self.st[Self::l(p)]
            } else {
                self.st[Self::r(p)]
            }
        }
    }

    fn build_internal(&mut self, p: usize, left: usize, right: usize) {
        if left == right {
            self.st[p] = Some(self.nums[left]);
        } else {
            let m = (right - left) / 2 + left;
            self.build_internal(Self::l(p), left, m);
            self.build_internal(Self::r(p), m + 1, right);
            self.st[p] = self.conquer(self.st[Self::l(p)], self.st[Self::r(p)]);
        }
    }

    fn propagate(&mut self, p: usize, left: usize, right: usize) {
        if Option::is_some(&self.lazy[p]) {
            self.st[p] = self.lazy[p];
            if left != right {
                self.lazy[Self::l(p)] = self.lazy[p];
                self.lazy[Self::r(p)] = self.lazy[p];
            } else {
                self.nums[left] = self.lazy[p].unwrap_or_else(|| self.nums[left]);
            }
        }
    }

    fn rmq_internal(
        &mut self,
        p: usize,
        left: usize,
        right: usize,
        i: usize,
        j: usize,
    ) -> Option<T> {
        self.propagate(p, left, right);
        if i > j {
            return None;
        }

        if (left >= i) && (right <= j) {
            return self.st[p];
        }

        let m: usize = (right - left) / 2 + left;
        let res_left = self.rmq_internal(Self::l(p), left, m, i, min(m, j));
        let res_right = self.rmq_internal(Self::r(p), m + 1, right, max(i, m + 1), j);
        self.conquer(res_left, res_right)
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTree;
    use rand::prelude::*;
    use std::cmp::min;

    #[test]
    fn test_sparse_table_min() {
        let nums = [18, 17, 13, 19, 15, 11, 20, 99];
        let cnq = |a, b| min(a, b);
        let mut st = SegmentTree::new(nums.to_vec(), Box::new(cnq));
        st.build();

        let res = st.rmq(1, 3);
        assert_eq!(res, Some(13));

        let res = st.rmq(4, 7);
        assert_eq!(res, Some(11));

        let res = st.rmq(3, 4);
        assert_eq!(res, Some(15));
    }

    #[test]
    fn test_sparse_table_max() {
        let nums = [18, 17, 13, 19, 15, 11, 20, 99];
        let cnq = |a, b| std::cmp::max(a, b);
        let mut st = SegmentTree::new(nums.to_vec(), Box::new(cnq));
        st.build();

        let res = st.rmq(1, 3);
        assert_eq!(res, Some(19));

        let res = st.rmq(4, 7);
        assert_eq!(res, Some(99));

        let res = st.rmq(3, 4);
        assert_eq!(res, Some(19));
    }

    #[test]
    fn test_sparse_table_updates_min() {
        let nums = [18, 17, 13, 19, 15, 11, 20, 99];
        let cnq = |a, b| min(a, b);
        let mut st = SegmentTree::new(nums.to_vec(), Box::new(cnq));
        st.build();
        st.update(5, 5, 40);

        let res = st.rmq(1, 3);
        assert_eq!(res, Some(13));

        let res = st.rmq(4, 7);
        assert_eq!(res, Some(15));

        let res = st.rmq(3, 4);
        assert_eq!(res, Some(15));
    }

    #[test]
    fn test_sparse_table_updates_max() {
        let nums = [18, 17, 13, 19, 15, 11, 20, 99];
        let cnq = |a, b| std::cmp::max(a, b);
        let mut st = SegmentTree::new(nums.to_vec(), Box::new(cnq));
        st.build();
        st.update(5, 5, 120);

        let res = st.rmq(1, 3);
        assert_eq!(res, Some(19));

        let res = st.rmq(4, 7);
        assert_eq!(res, Some(120));

        let res = st.rmq(3, 4);
        assert_eq!(res, Some(19));
    }

    #[test]
    fn test_sparse_table_random() {
        let n = 1000;

        let mut nums: Vec<i32> = vec![0; n];
        let mut rng = rand::thread_rng();

        for i in 0..n {
            nums[i] = rng.gen_range(0..1000);
        }

        let cnq = |a, b| min(a, b);
        let mut st = SegmentTree::new(nums.to_vec(), Box::new(cnq));
        st.build();

        for _ in 0..10_000 {
            let v1 = rng.gen_range(1..n);
            let v2 = rng.gen_range(1..n);

            let l = min(v1, v2);
            let r = min(v1, v2);

            let res = st.rmq(l, r);
            let expected = nums[l..=r].iter().min().copied();
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_sparse_table_strings() {
        let nums = ["aaa", "be", "cc", "fdddd", "ed", "fa", "gasdasd", "h"];
        let cnq = |a, b| min(a, b);
        let mut st = SegmentTree::new(nums.to_vec(), Box::new(cnq));
        st.build();

        let res = st.rmq(1, 3);
        assert_eq!(res, Some("be"));

        let res = st.rmq(4, 7);
        assert_eq!(res, Some("ed"));

        let res = st.rmq(3, 4);
        assert_eq!(res, Some("ed"));
    }
}
