
pub fn bipow_rec(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let res = bipow_rec(a, b / 2);

    if b % 2 == 1 {
        return res * res * 2;
    } else {
        return res * res;
    }
}

pub fn bipow(mut a: i64, mut b: i64) -> i64 {
    let mut res = 1;

    while b > 0 {
        if b & 1 == 1 {
            res *= a;
        }

        a = a * a;
        b >>= 1;
    }

    res
}
