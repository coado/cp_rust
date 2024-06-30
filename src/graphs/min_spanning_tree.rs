use std::{collections::BinaryHeap, default};

use crate::data_structures::union_find::UnionFind;
use anyhow::{bail, Result};

#[derive(Clone, Copy, Debug, Default)]
pub struct Edge {
    from: usize,
    to: usize,
    weight: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Kruskal {
    edges: Vec<Edge>,
    num_nodes: usize,
}

impl Kruskal {
    pub fn new(num_nodes: usize) -> Self {
        Kruskal {
            edges: Vec::new(),
            num_nodes,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32) -> Result<()> {
        if self.num_nodes <= from || self.num_nodes <= to {
            bail!("Invalid node index");
        }

        self.edges.push(Edge { from, to, weight });
        Ok(())
    }

    pub fn min_spanning_tree(&self) -> Result<(i32, Vec<Edge>)> {
        let mut uf = UnionFind::with_capacity(self.num_nodes);
        let mut edges = self.edges.clone();
        edges.sort_by_key(|x| x.weight);

        let mut mst = Vec::new();
        let mut mst_cost = 0;

        for edge in edges {
            if uf.union_set(edge.from, edge.to) {
                mst.push(edge);
                mst_cost += edge.weight;
                if mst.len() == self.num_nodes - 1 {
                    break;
                }
            }
        }

        Ok((mst_cost, mst))
    }
}

pub struct Prim {
    num_nodes: usize,
    adj_list: Vec<Vec<(usize, i32)>>,
}

impl Prim {
    pub fn new(num_nodes: usize) -> Self {
        Prim {
            num_nodes,
            adj_list: vec![Vec::new(); num_nodes],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32) -> Result<()> {
        if self.num_nodes <= from || self.num_nodes <= to {
            bail!("Invalid node index");
        }

        self.adj_list[from].push((to, weight));
        self.adj_list[to].push((from, weight));
        Ok(())
    }

    pub fn min_spanning_tree(&self) -> Result<(i32, Vec<Edge>)> {
        let mut visited = vec![false; self.num_nodes];
        let mut mst = Vec::new();
        let mut mst_cost = 0;

        let mut pq: BinaryHeap<(i32, usize, usize)> = std::collections::BinaryHeap::new();
        pq.push((0, 0, 0));

        while let Some((weight, from, to)) = pq.pop() {
            if visited[to] {
                continue;
            }

            visited[to] = true;
            mst_cost += weight;
            if to != 0 {
                mst.push(Edge { from, to, weight });
            }

            for &(next, weight) in &self.adj_list[to] {
                if !visited[next] {
                    pq.push((weight, to, next));
                }
            }
        }

        Ok((mst_cost, mst))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal() {
        let mut kruskal = Kruskal::new(5);

        kruskal.add_edge(0, 1, 1).unwrap();
        kruskal.add_edge(0, 2, 2).unwrap();
        kruskal.add_edge(0, 3, 3).unwrap();
        kruskal.add_edge(0, 4, 4).unwrap();
        kruskal.add_edge(1, 2, 5).unwrap();
        kruskal.add_edge(1, 3, 6).unwrap();
        kruskal.add_edge(1, 4, 7).unwrap();
        kruskal.add_edge(2, 3, 8).unwrap();
        kruskal.add_edge(2, 4, 9).unwrap();
        kruskal.add_edge(3, 4, 10).unwrap();

        let (cost, mst) = kruskal.min_spanning_tree().unwrap();

        assert_eq!(cost, 16);
        assert_eq!(mst.len(), 4);
    }
}
