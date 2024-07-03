use anyhow::Result;

#[derive(Debug, Clone, Default)]
pub struct Kosarajus {
    visited: Vec<bool>,
    order: Vec<usize>,
}

impl Kosarajus {
    pub fn new() -> Self {
        Self {
            visited: vec![],
            order: vec![],
        }
    }

    pub fn kosarajus(&mut self, al: &[Vec<usize>]) -> Result<Vec<Vec<usize>>> {
        let n = al.len();

        let mut al_rev = vec![vec![]; n];
        for (u, item) in al.iter().enumerate() {
            for &v in item {
                al_rev[v].push(u);
            }
        }

        self.visited = vec![false; n];
        self.order = vec![];

        for u in 0..n {
            if !self.visited[u] {
                self.dfs(u, al);
            }
        }

        self.visited.fill(false);

        let mut components = vec![];

        for u in self.order.clone().iter().rev() {
            if !self.visited[*u] {
                let mut component = vec![];
                self.dfs_rev(*u, &al_rev, &mut component);
                components.push(component);
            }
        }

        Ok(components)
    }

    fn dfs(&mut self, u: usize, al: &[Vec<usize>]) {
        self.visited[u] = true;
        for &v in &al[u] {
            if !self.visited[v] {
                self.dfs(v, al);
            }
        }

        self.order.push(u);
    }

    fn dfs_rev(&mut self, u: usize, al: &[Vec<usize>], components: &mut Vec<usize>) {
        self.visited[u] = true;
        for &v in &al[u] {
            if !self.visited[v] {
                self.dfs_rev(v, al, components);
            }
        }

        components.push(u);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tarjans {
    visited: Vec<bool>,
    stack: Vec<usize>,
    low: Vec<usize>,
    num: Vec<usize>,
    counter: usize,
}

impl Tarjans {
    pub fn new() -> Self {
        Self {
            visited: vec![],
            stack: vec![],
            low: vec![],
            num: vec![],
            counter: 0,
        }
    }

    pub fn tarjans(&mut self, al: &[Vec<usize>]) -> Result<Vec<Vec<usize>>> {
        let n = al.len();

        self.visited = vec![false; n];
        self.stack = vec![];
        self.low = vec![0; n];
        self.num = vec![0; n];
        self.counter = 0;

        let mut components = vec![];

        for u in 0..n {
            if !self.visited[u] {
                self.dfs(al, u, &mut components);
            }
        }

        Ok(components)
    }

    fn dfs(&mut self, al: &[Vec<usize>], u: usize, components: &mut Vec<Vec<usize>>) {
        self.visited[u] = true;
        self.low[u] = self.counter;
        self.num[u] = self.counter;
        self.counter += 1;

        self.stack.push(u);

        for v in al[u].iter() {
            if !self.visited[*v] {
                self.dfs(al, *v, components);
            } else {
                self.low[u] = self.low[0].min(self.low[*v]);
            }
        }

        if self.low[u] == self.num[u] {
            let mut component = vec![];
            loop {
                let v = self.stack.pop().unwrap();
                component.push(v);

                if u == v {
                    break;
                }
            }

            components.push(component);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Kosarajus;

    #[test]
    fn test_kosarajus() {
        let al = vec![vec![1], vec![2], vec![0], vec![4], vec![3]];
        let mut kosarajus = Kosarajus::new();
        let res = kosarajus.kosarajus(al.as_slice()).unwrap();
        assert_eq!(2, res.len());

        let al = vec![
            vec![1],
            vec![3],
            vec![1],
            vec![2, 4],
            vec![5],
            vec![7],
            vec![4],
            vec![6],
        ];

        let res = kosarajus.kosarajus(al.as_slice()).unwrap();
        assert_eq!(res.len(), 3);
    }
}
