use cp_rust::min_stack::Stack;

#[test]
fn test_min_stack() {
    let mut stack = Stack::new();
    stack.add(1);
    stack.add(2);
    stack.add(3);
    stack.add(4);
    stack.add(5);
    stack.add(6);
    stack.add(7);
    stack.add(8);
    let min = stack.get_min().unwrap();
    assert_eq!(*min, 1);
    stack.pop();
    stack.pop();
    stack.pop();
    let min = stack.get_min().unwrap();
    assert_eq!(*min, 1);
    stack.pop();
    stack.pop();
    stack.pop();
    stack.pop();
    let min = stack.get_min().unwrap();
    assert_eq!(*min, 1);
    stack.pop();
    let min = stack.get_min();
    assert_eq!(min, None);
}

#[test]
fn test_min_stack2() {
    let mut stack = Stack::new();
    stack.add(8);
    stack.add(7);
    stack.add(6);
    let min = stack.get_min().unwrap();
    assert_eq!(*min, 6);
    stack.pop();
    let min = stack.get_min().unwrap();
    assert_eq!(*min, 7);
    stack.pop();
    let min = stack.get_min().unwrap();
    assert_eq!(*min, 8);
}
