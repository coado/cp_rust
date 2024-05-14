use cp_rust::sparse_table::{get_sparse_table, SparseTableType};

#[test]
fn test_sparse_table_sum() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let st = get_sparse_table(nums, SparseTableType::SUM);
    let res = st.sum_query(0, 7);
    print!("Sum query result: {}", res);
}

#[test]
fn test_sparse_table_min() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let st = get_sparse_table(nums, SparseTableType::MIN);
    let res = st.min_query(4, 7);
    print!("Min query result: {}", res);
}

#[test]
fn test_sparse_table_max() {
    let nums = vec![1, 2, 20, 4, 5, 6, 7, 8];
    let st = get_sparse_table(nums, SparseTableType::MAX);
    let res = st.max_query(0, 7);
    print!("Max query result: {}", res);
}
