pub fn hungarian_alg(a: Vec<Vec<i32>>, n: usize, m: usize) -> Vec<i32> {
    // potential of L
    let mut u = vec![0; n + 1];
    // potential of R
    let mut v = vec![0; m + 1];
    // matching
    let mut p = vec![0; m + 1];
    // where the mins are reached, so that we can reconstruct the augmenting path
    // for each column j, contains the number of previous column in the path
    let mut way = vec![0; m + 1];

    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0;
        // stores auxilary min for each column
        let mut minv = vec![std::i32::MAX; m + 1];
        let mut used = vec![false; m + 1];

        // runs until it finds the R j0 vertex that is not matched
        while p[j0] != 0 {
            used[j0] = true;
            // adjacent L vertex
            let i0 = p[j0];
            let mut delta = std::i32::MAX;
            let mut j1 = 0;

            for j in 1..=m {
                if used[j] {
                    continue;
                };

                let cur = a[i0 - 1][j - 1] - u[i0] - v[j];
                if cur < minv[j] {
                    minv[j] = cur;
                    way[j] = j0;
                }

                if minv[j] < delta {
                    delta = minv[j];
                    j1 = j;
                }
            }

            for j in 0..=m {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            j0 = j1;
        }

        while j0 != 0 {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
        }
    }

    let mut ans = vec![0; n + 1];
    for j in 1..=m {
        ans[p[j]] = j as i32;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::hungarian_alg;

    #[test]
    fn test_hangurian_alg() {
        let a: Vec<Vec<i32>> = [
            [108, 125, 150].to_vec(),
            [150, 135, 175].to_vec(),
            [122, 148, 250].to_vec(),
        ]
        .to_vec();
        let res = hungarian_alg(a, 3, 3);
        println!("{:?}", res);
    }
}
