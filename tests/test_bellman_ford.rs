use cp_rust::bellman_ford;

#[test]
#[should_panic]
fn test_bellman_ford() {
    let graph = vec![
        vec![None, Some(5), None, None, None, None],
        vec![None, None, Some(1), Some(2), None, None],
        vec![None, None, None, None, Some(1), None],
        vec![None, None, None, None, None, Some(2)],
        vec![None, None, None, Some(-1), None, None],
        vec![None, None, None, None, Some(-3), None],
    ];
    let _ = bellman_ford::bellman_ford(graph, 0, 6).unwrap();

    // assert_eq!(res, vec![0, 5, 6, 7, 5, 2]);
}
