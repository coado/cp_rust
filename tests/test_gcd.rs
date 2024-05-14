use cp_rust::gcd;

#[test]
fn test_gcd() {
    let result = gcd::gcd(8, 12);
    assert_eq!(result, 4);
}
