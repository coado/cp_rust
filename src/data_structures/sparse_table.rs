use num::Zero;
use std::f64;
use std::ops::AddAssign;

pub enum SparseTableType {
    MIN,
    MAX,
    SUM,
}

pub struct SparseTable<T>
where
    T: Ord + Zero + Copy + AddAssign,
{
    pub st: Vec<Vec<T>>,
    pub nums: Vec<T>,
    pub k: usize,
    pub st_type: SparseTableType,
}

impl<T> SparseTable<T>
where
    T: Ord + Zero + Copy + AddAssign,
{
    pub fn new(nums: Vec<T>, st_type: SparseTableType) -> Self {
        let k = f64::log2(nums.len() as f64) as usize;
        let mut st = vec![vec![T::zero(); nums.len()]; k + 1];

        st[0][..nums.len()].copy_from_slice(&nums[..]);

        for i in 1..=k {
            let mut j = 0;
            while j + (1 << i) <= nums.len() {
                st[i][j] = match st_type {
                    SparseTableType::MIN => st[i - 1][j].min(st[i - 1][j + (1 << (i - 1))]),
                    SparseTableType::MAX => st[i - 1][j].max(st[i - 1][j + (1 << (i - 1))]),
                    SparseTableType::SUM => st[i - 1][j] + st[i - 1][j + (1 << (i - 1))],
                };
                j += 1;
            }
        }

        Self {
            st,
            nums,
            k,
            st_type,
        }
    }

    pub fn sum_query(&self, l: usize, r: usize) -> T {
        let mut sum = T::zero();
        let mut j = l;
        for i in (0..=self.k).rev() {
            if j <= r && (1 << i) <= r - j + 1 {
                sum += self.st[i][j];
                j += 1 << i;
            }
        }

        sum
    }

    pub fn min_query(&self, l: usize, r: usize) -> T {
        let i = f64::log2((r - l + 1) as f64) as usize;
        self.st[i][l].min(self.st[i][r - ((1 << i) - 1)])
    }

    pub fn max_query(&self, l: usize, r: usize) -> T {
        let i = f64::log2((r - l + 1) as f64) as usize;
        self.st[i][l].max(self.st[i][r - ((1 << i) - 1)])
    }
}

#[cfg(test)]
mod test {
    use super::{SparseTable, SparseTableType};

    #[test]
    fn test_sparse_table_sum() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let st = SparseTable::new(nums, SparseTableType::SUM);
        let res = st.sum_query(0, 7);
        print!("Sum query result: {}", res);
    }

    #[test]
    fn test_sparse_table_min() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let st = SparseTable::new(nums, SparseTableType::MIN);
        let res = st.min_query(4, 7);
        print!("Min query result: {}", res);
    }

    #[test]
    fn test_sparse_table_max() {
        let nums = vec![1, 2, 20, 4, 5, 6, 7, 8];
        let st = SparseTable::new(nums, SparseTableType::MAX);
        let res = st.max_query(0, 7);
        print!("Max query result: {}", res);
    }
}
