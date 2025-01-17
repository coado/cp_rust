use anyhow::{bail, Result};
use num::{Bounded, Zero};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, VecDeque};
use std::ops::{AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Default, Copy)]
pub struct Edge<T> {
    pub to: usize,
    pub capacity: T,
    pub flow: T,
}

impl<T> Edge<T>
where
    T: Zero,
{
    pub fn new(to: usize, capacity: T) -> Self {
        Edge {
            to,
            capacity,
            flow: T::zero(),
        }
    }
}

pub trait AddEdge {
    fn add_edge(&mut self, from: usize, to: usize, capacity: i32) -> Result<()>;
}

/// Edmonds-Karp algorithm for finding maximum flow in a graph
/// that runs in O(VE^2) time complexity.
/// Uses BFS to find augmenting paths (shortest path from source to sink)
#[derive(Debug, Clone, Default)]
pub struct EdmondsKarpMaxFlow<T> {
    al: Vec<Vec<usize>>,
    edges: Vec<Edge<T>>,
    parent: Vec<Option<usize>>,
    n: usize,
    lt: HashMap<String, usize>,
}

impl<T> EdmondsKarpMaxFlow<T>
where
    T: Zero + Copy + AddAssign + Sub<Output = T> + SubAssign + PartialEq + Bounded + Ord,
{
    pub fn new(n: usize) -> Self {
        EdmondsKarpMaxFlow {
            al: vec![vec![]; n],
            edges: vec![],
            parent: vec![None; n],
            n,
            lt: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, capacity: T) -> Result<()> {
        assert_vertices_in_bounds(from, to, self.n)?;

        let id = self.edges.len();
        self.al[from].push(id);
        self.edges.push(Edge::new(to, capacity));
        self.lt.insert(format!("{}-{}", from, to), id);

        self.al[to].push(id + 1);
        self.edges.push(Edge::new(from, T::zero()));
        self.lt.insert(format!("{}-{}", to, from), id + 1);

        Ok(())
    }

    pub fn maxflow(&mut self, source: usize, sink: usize) -> Result<T> {
        assert_vertices_in_bounds(source, sink, self.n)?;

        self.reset();
        let mut flow = T::zero();

        loop {
            let new_flow = self.bfs(source, sink);
            if new_flow == T::zero() {
                break;
            }
            flow += new_flow;
            let mut cur = sink;
            while cur != source {
                if let Some(prev) = self.parent[cur] {
                    let edge = self.edges[self.lt[&format!("{}-{}", prev, cur)]].borrow_mut();
                    edge.flow += new_flow;
                    let edge = self.edges[self.lt[&format!("{}-{}", cur, prev)]].borrow_mut();
                    edge.flow -= new_flow;
                    cur = prev;
                } else {
                    bail!("No path from source to sink");
                }
            }
        }

        Ok(flow)
    }

    fn bfs(&mut self, source: usize, sink: usize) -> T {
        self.parent.fill(None);
        self.parent[source] = Some(source);
        let mut deque: VecDeque<(usize, T)> = VecDeque::new();
        deque.push_back((source, T::max_value()));

        while !deque.is_empty() {
            let (v, flow) = deque.pop_front().unwrap();

            for u in self.al[v].iter() {
                let edge = self.edges[*u].borrow();
                if self.parent[edge.to].is_none() && edge.capacity - edge.flow > T::zero() {
                    self.parent[edge.to] = Some(v);
                    let new_flow = std::cmp::min(flow, edge.capacity - edge.flow);
                    if edge.to == sink {
                        return new_flow;
                    }

                    deque.push_back((edge.to, new_flow));
                }
            }
        }

        T::zero()
    }

    fn reset(&mut self) {
        self.parent.fill(None);
        self.edges.iter_mut().for_each(|edge| {
            edge.flow = T::zero();
        });
    }
}

#[derive(Debug, Clone, Default)]
pub struct DinicMaxFlow<T> {
    dist: Vec<i32>,
    last: Vec<usize>,
    source: usize,
    sink: usize,
    al: Vec<Vec<usize>>,
    num_edges: usize,
    edges: Vec<Edge<T>>,
    vertices: usize,
}

impl<T> DinicMaxFlow<T>
where
    T: Zero + Clone + Copy + AddAssign + Sub<Output = T> + SubAssign + PartialEq + Bounded + Ord,
{
    pub fn new(source: usize, sink: usize, vertices: usize) -> Result<Self> {
        assert_vertices_in_bounds(source, sink, vertices)?;

        Ok(DinicMaxFlow {
            dist: vec![-1; vertices],
            last: vec![0; vertices],
            source,
            sink,
            al: vec![vec![]; vertices],
            num_edges: 0,
            edges: vec![],
            vertices,
        })
    }

    pub fn add_edge(&mut self, from: usize, to: usize, capacity: T) -> Result<()> {
        assert_vertices_in_bounds(from, to, self.vertices)?;

        self.al[from].push(self.num_edges);
        self.edges.push(Edge::new(to, capacity));
        self.num_edges += 1;
        self.al[to].push(self.num_edges);
        self.edges.push(Edge::new(from, T::zero()));
        self.num_edges += 1;

        Ok(())
    }

    pub fn update_source_and_sink(&mut self, source: usize, sink: usize) -> Result<()> {
        assert_vertices_in_bounds(source, sink, self.vertices)?;

        self.source = source;
        self.sink = sink;

        Ok(())
    }

    fn bfs(&mut self) -> bool {
        self.dist.fill(-1);
        self.dist[self.source] = 0;
        let mut deque = VecDeque::new();
        deque.push_back(self.source);

        while !deque.is_empty() {
            let u = deque.pop_front().unwrap();

            if u == self.sink {
                break;
            }

            for v in self.al[u].iter() {
                let edge = &self.edges[*v];
                if self.dist[edge.to] == -1 && edge.flow < edge.capacity {
                    self.dist[edge.to] = self.dist[u] + 1;
                    deque.push_back(edge.to);
                }
            }
        }

        self.dist[self.sink] != -1
    }

    fn dfs(&mut self, u: usize, f: T) -> T {
        if u == self.sink || f == T::zero() {
            return f;
        }

        for e in self.last[u]..self.al[u].len() {
            let edge_id = self.al[u][e];
            let edge = &self.edges[edge_id];

            if self.dist[edge.to] != self.dist[u] + 1 || edge.flow == edge.capacity {
                continue;
            }

            let df = self.dfs(edge.to, std::cmp::min(f, edge.capacity - edge.flow));

            if df > T::zero() {
                self.last[u] = e;
                self.edges[edge_id].flow += df;
                self.edges[edge_id ^ 1].flow -= df;
                return df;
            }
        }

        self.last[u] = self.al[u].len();
        T::zero()
    }

    pub fn maxflow(&mut self) -> T {
        self.reset();
        let mut flow = T::zero();

        while self.bfs() {
            self.last.fill(0);
            loop {
                let df = self.dfs(self.source, T::max_value());
                if df == T::zero() {
                    break;
                }
                flow += df;
            }
        }

        flow
    }

    fn reset(&mut self) {
        self.dist.fill(0);
        self.last.fill(0);
        self.edges.iter_mut().for_each(|edge| {
            edge.flow = T::zero();
        });
    }
}

fn assert_vertices_in_bounds(from: usize, to: usize, n: usize) -> Result<()> {
    if from >= n || to >= n {
        bail!("Provided vertices are out of bounds");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{DinicMaxFlow, EdmondsKarpMaxFlow};

    #[test]
    fn test_edmonds_karp_maxflow() {
        let mut edmonds_karp = EdmondsKarpMaxFlow::new(5);
        edmonds_karp.add_edge(0, 1, 3).unwrap();
        edmonds_karp.add_edge(0, 2, 2).unwrap();
        edmonds_karp.add_edge(1, 2, 2).unwrap();
        edmonds_karp.add_edge(1, 3, 3).unwrap();
        edmonds_karp.add_edge(2, 4, 2).unwrap();
        edmonds_karp.add_edge(3, 4, 2).unwrap();

        let res = edmonds_karp.maxflow(0, 4).unwrap();

        assert_eq!(res, 4);

        let mut edmonds_karp = EdmondsKarpMaxFlow::new(6);
        edmonds_karp.add_edge(0, 1, 7).unwrap();
        edmonds_karp.add_edge(0, 4, 4).unwrap();
        edmonds_karp.add_edge(1, 2, 5).unwrap();
        edmonds_karp.add_edge(1, 3, 3).unwrap();
        edmonds_karp.add_edge(2, 5, 8).unwrap();
        edmonds_karp.add_edge(3, 2, 3).unwrap();
        edmonds_karp.add_edge(3, 5, 5).unwrap();
        edmonds_karp.add_edge(4, 1, 3).unwrap();
        edmonds_karp.add_edge(4, 3, 2).unwrap();

        let res = edmonds_karp.maxflow(0, 5).unwrap();
        assert_eq!(res, 10);

        let res = edmonds_karp.maxflow(1, 5).unwrap();
        assert_eq!(res, 8);

        let res = edmonds_karp.maxflow(1, 2).unwrap();
        assert_eq!(res, 8);

        let res = edmonds_karp.maxflow(4, 3).unwrap();
        assert_eq!(res, 5);

        let mut edmonds_karp = EdmondsKarpMaxFlow::new(4);
        edmonds_karp.add_edge(0, 1, 20).unwrap();
        edmonds_karp.add_edge(0, 2, 10).unwrap();
        edmonds_karp.add_edge(1, 2, 5).unwrap();
        edmonds_karp.add_edge(1, 3, 10).unwrap();
        edmonds_karp.add_edge(2, 3, 20).unwrap();

        let res = edmonds_karp.maxflow(0, 3);
        assert_eq!(res.unwrap(), 25);
    }

    #[test]
    fn test_dinic_maxflow() {
        let mut dinic = DinicMaxFlow::new(0, 4, 5).unwrap();
        dinic.add_edge(0, 1, 3).unwrap();
        dinic.add_edge(0, 2, 2).unwrap();
        dinic.add_edge(1, 2, 2).unwrap();
        dinic.add_edge(1, 3, 3).unwrap();
        dinic.add_edge(2, 4, 2).unwrap();
        dinic.add_edge(3, 4, 2).unwrap();

        let res = dinic.maxflow();

        assert_eq!(res, 4);

        let mut dinic = DinicMaxFlow::new(0, 5, 6).unwrap();
        dinic.add_edge(0, 1, 7).unwrap();
        dinic.add_edge(0, 4, 4).unwrap();
        dinic.add_edge(1, 2, 5).unwrap();
        dinic.add_edge(1, 3, 3).unwrap();
        dinic.add_edge(2, 5, 8).unwrap();
        dinic.add_edge(3, 2, 3).unwrap();
        dinic.add_edge(3, 5, 5).unwrap();
        dinic.add_edge(4, 1, 3).unwrap();
        dinic.add_edge(4, 3, 2).unwrap();

        let res = dinic.maxflow();
        assert_eq!(res, 10);

        dinic.update_source_and_sink(1, 5).unwrap();
        let res = dinic.maxflow();
        assert_eq!(res, 8);

        dinic.update_source_and_sink(1, 2).unwrap();
        let res = dinic.maxflow();
        assert_eq!(res, 8);

        dinic.update_source_and_sink(4, 3).unwrap();
        let res = dinic.maxflow();
        assert_eq!(res, 5);

        let mut dinic = DinicMaxFlow::new(0, 3, 4).unwrap();
        dinic.add_edge(0, 1, 20).unwrap();
        dinic.add_edge(0, 2, 10).unwrap();
        dinic.add_edge(1, 2, 5).unwrap();
        dinic.add_edge(1, 3, 10).unwrap();
        dinic.add_edge(2, 3, 20).unwrap();

        let res = dinic.maxflow();
        assert_eq!(res, 25);
    }
}
