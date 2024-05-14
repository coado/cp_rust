use std::f64;

pub enum SparseTableType {
    MIN,
    MAX,
    SUM,
}

pub struct SparseTable {
    pub st: Vec<Vec<i32>>,
    pub nums: Vec<i32>,
    pub k: usize,
    pub st_type: SparseTableType,
}

impl SparseTable {
    pub fn new(nums: Vec<i32>, st_type: SparseTableType) -> Self {
        let k = f64::log2(nums.len() as f64) as usize;
        let mut st = vec![vec![0; nums.len()]; k + 1];

        for i in 0..nums.len() {
            st[0][i] = nums[i];
        }

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

    pub fn sum_query(&self, l: usize, r: usize) -> i32 {
        let mut sum = 0;
        let mut j = l;
        for i in (0..=self.k).rev() {
            if j <= r && (1 << i) <= r - j + 1 {
                sum += self.st[i][j];
                j += 1 << i;
            }
        }

        sum
    }

    pub fn min_query(&self, l: usize, r: usize) -> i32 {
        let i = f64::log2((r - l + 1) as f64) as usize;
        self.st[i][l].min(self.st[i][r - ((1 << i) - 1)])
    }

    pub fn max_query(&self, l: usize, r: usize) -> i32 {
        let i = f64::log2((r - l + 1) as f64) as usize;
        self.st[i][l].max(self.st[i][r - ((1 << i) - 1)])
    }
}

pub fn get_sparse_table(nums: Vec<i32>, st_type: SparseTableType) -> SparseTable {
    SparseTable::new(nums, st_type)
}
