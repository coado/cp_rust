use cp_rust::topological_sort;

#[test]
fn test_topological_sort() {
    let graph = vec![vec![], vec![], vec![3], vec![1], vec![0, 1], vec![2, 0]];
    let res = topological_sort::topological_sort(graph).unwrap();

    println!("{:?}", res);

    assert_eq!(res, vec![5, 4, 2, 3, 1, 0]);
}
