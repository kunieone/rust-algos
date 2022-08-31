use crate::graph_iter::BFSIterator;
use crate::graph_iter::DFSIterator;
use crate::union_find::UnionFind;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::fmt::Debug;
// 带权图 -> &weight 1
type Edge = (usize, usize);
type Weight = usize;

pub struct NeighbourGenerator<'a, T> {
    g: &'a Graph<T>,
    record: BTreeSet<(Weight, Edge)>,
}

impl<'a, T: Debug + Ord> NeighbourGenerator<'a, T> {
    fn add(&mut self, v1: usize) {
        for v2 in self.g.neighbour(v1) {
            // Edge     左<右
            let edge_msg = if v1 < v2 {
                (self.g.matrix[v1][v2].unwrap(), (v1, v2))
            } else {
                (self.g.matrix[v1][v2].unwrap(), (v2, v1))
            };
            // 异或 1
            if self.record.contains(&edge_msg) {
                self.record.remove(&edge_msg);
            } else {
                self.record.insert(edge_msg);
            };
        }
    }
    pub fn get_min_edge(&self) -> Option<(Weight, Edge)> {
        match self.record.iter().min() {
            Some(&e) => Some(e),
            None => None,
        }
    }
}
#[derive(Debug)]

pub struct Graph<T> {
    pub matrix: Vec<Vec<Option<usize>>>,
    pub edge_list: BTreeMap<Edge, Weight>,
    pub nodes: BTreeMap<usize, Option<T>>,
}

impl<T: Debug + Ord> Graph<T> {
    pub fn neighbour(&self, i: usize) -> Vec<usize> {
        let mut result = vec![];
        for j in 1..=self.bound() {
            if self.matrix[i][j].is_some() {
                result.push(j);
            }
        }
        result
    }
    pub fn new() -> Self {
        Self {
            matrix: vec![],
            nodes: BTreeMap::new(),
            edge_list: BTreeMap::new(),
        }
    }

    pub fn get_sum_weight(&self) -> usize {
        let sum: usize = self.edge_list.iter().map(|(_, &x)| x).sum();
        sum
    }

    pub fn get_first(&self) -> Option<usize> {
        match self
            .nodes
            .iter()
            .map(|x| *x.0)
            .collect::<Vec<usize>>()
            .first()
        {
            Some(e) => Some(*e),
            None => None,
        }
    }
    pub fn prim(&self) -> Option<Graph<T>> {
        if self.is_empty() || !self.is_connected() {
            return None;
        }
        let mut v1 = self.get_first().unwrap();
        let mut mst: Graph<T> = Graph::new();
        let mut neighbours = self.neighbour_gen(); //  邻居节点生成器
        while mst.nodes.len() < self.nodes.len() {
            // 把生成树添加的节点遍历放入邻居生成器里，获得邻居边。
            neighbours.add(v1);
            let min_edge = neighbours.get_min_edge();
            if let Some((weight, (from, to))) = min_edge {
                mst.add_edge(from, to, weight);
                v1 = if from != v1 { from } else { to };
            }
        }
        Some(mst)
    }

    pub fn neighbour_gen(&self) -> NeighbourGenerator<T> {
        NeighbourGenerator {
            g: self,
            record: BTreeSet::new(),
        }
    }
    pub fn neighbours_edges(&self, v: Vec<usize>) -> Option<BTreeSet<(Weight, Edge)>> {
        if self.is_empty() || v.is_empty() {
            return None;
        }
        // 暂存边
        let mut result: BTreeSet<(Weight, Edge)> = BTreeSet::new();
        for v1 in v {
            for v2 in self.neighbour(v1) {
                if v1 < v2 {
                    let edge_message = (self.matrix[v1][v2].unwrap(), (v1, v2));
                    if result.contains(&edge_message) {
                        result.remove(&edge_message);
                    } else {
                        result.insert(edge_message);
                    }
                }
            }
        }
        Some(result)
    }
    pub fn rm_all_edges(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            let edge_list = self.edge_list.clone();
            for e in edge_list.keys() {
                self.rm_edge(e.0, e.1);
            }
            true
        }
    }

    pub fn kruskal(&self) -> Option<Graph<T>> {
        if self.is_empty() || !self.is_connected() {
            return None;
        }

        let mut tmp: BTreeSet<(Weight, Edge)> = BTreeSet::new();
        for (&(v1, v2), &weight) in self.edge_list.iter() {
            tmp.insert((weight, (v1, v2)));
        }
        let mut gra: Graph<T> = Graph::new();
        let mut uf = UnionFind::init(self.bound() + 1);
        for (weight, (v1, v2)) in tmp {
            let is_legal = uf.union(v1, v2);
            if is_legal {
                gra.add_edge(v1, v2, weight)
            }
        }
        Some(gra)
    }
    pub fn dfs_iter(&self) -> DFSIterator<T> {
        let stk = match self.get_first() {
            Some(e) => vec![e],
            None => vec![],
        };
        DFSIterator {
            graph: self,
            stk,
            visited: vec![false; self.bound() + 1],
        }
    }

    pub fn bfs_iter(&self) -> BFSIterator<T> {
        let que = match self.get_first() {
            Some(e) => VecDeque::from([e]),
            None => VecDeque::new(),
        };
        BFSIterator {
            graph: self,
            que,
            visited: vec![false; self.bound() + 1],
        }
    }
    // 删除节点的时候会用到，因为需要删除节点相连的边
    fn remove_relative_edge(&mut self, index: usize) {
        //  3   (1,2) /* (2,3) (3,4) */ (2,4)
        // BTreeMap
        for e in self.neighbour(index) {
            if e < index {
                self.edge_list.remove(&(e, index));
            } else {
                self.edge_list.remove(&(index, e));
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 0
    }

    pub fn add_node(&mut self, index: usize) {
        self.nodes.insert(index, None);
        self.fix_length(index);
    }

    pub fn add_node_with_value(&mut self, index: usize, value: T) {
        self.nodes.insert(index, Some(value));
        self.fix_length(index);
    }
    pub fn fix_length(&mut self, index: usize) -> bool {
        if self.matrix.len() > index {
            false
        } else {
            let target_len = (index as f32 * 1.3) as usize + 2;

            while self.matrix.len() < target_len {
                self.matrix.push(vec![]);
            }
            for i in 0..target_len {
                while self.matrix[i].len() < target_len {
                    self.matrix[i].push(None)
                }
            }
            true
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        if from > to {
            self.edge_list.insert((to, from), weight);
        } else {
            self.edge_list.insert((from, to), weight);
        }
        if !self.nodes.contains_key(&from) {
            // println!("加入{}", from);
            self.add_node(from);
        }
        if !self.nodes.contains_key(&to) {
            // println!("加入{}", to);
            self.add_node(to);
        }
        self.matrix[from][to] = Some(weight);
        self.matrix[to][from] = Some(weight);
    }

    pub fn print(&self) {
        println!("{}", self.debug());
        println!("{}", self.debug_edge_list());
        println!("总权值和:{}", self.get_sum_weight());
    }

    pub fn rm_edge_single(&mut self, from: usize, to: usize) -> bool {
        if from.max(to) > self.bound() {
            false
        } else {
            self.matrix[from][to] = None;
            true
        }
    }

    pub fn rm_edge(&mut self, from: usize, to: usize) -> bool {
        /* 删除边表内的边的逻辑 */
        if from > to {
            self.edge_list.remove(&(to, from));
        } else {
            self.edge_list.remove(&(from, to));
        }
        self.rm_edge_single(from, to) && self.rm_edge_single(to, from)
    }

    pub fn rm_node(&mut self, index: usize) -> bool {
        if !self.nodes.contains_key(&index) {
            false
        } else {
            // 删除
            self.remove_relative_edge(index);
            self.nodes.remove(&index);
            self.matrix[index].fill(None);
            for i in 1..=self.bound() {
                self.matrix[i][index] = None;
            }
            true
        }
    }

    pub fn debug(&self) -> String {
        let bound = self.bound();
        let mut paint = " *".to_string();
        (1..=bound).for_each(|x| paint.push_str(&format!("{:2} ", x)));
        paint.push_str("\n");
        for i in 1..=bound {
            paint.push_str(&format!("{:2}|", i));
            for j in 1..=bound {
                paint.push_str(&format!(
                    "{:2} ",
                    (match self.matrix[i][j] {
                        Some(e) => format!("{}", e),
                        None => String::from("."),
                    })
                ))
            }
            paint.push_str("\n");
        }
        paint
    }
    pub fn debug_edge_list(&self) -> String {
        format!("{:?}", self.edge_list)
    }
    pub fn bound(&self) -> usize {
        match self.nodes.iter().max() {
            Some((&e, _)) => e,
            None => 0,
        }
    }
    pub fn is_connected(&self) -> bool {
        // 1-3   2   ->l:1
        // 2
        // nodes.len = 3
        self.dfs_iter().collect::<Vec<usize>>().len() == self.nodes.len()
    }
}

#[test]
fn debug() {
    let mut g: Graph<String> = Graph::new();
    g.add_edge(1, 3, 1);
    g.add_edge(1, 4, 2);
    g.add_edge(4, 6, 2);
    g.add_edge(2, 5, 2);
    g.add_edge(2, 4, 3);
    g.add_edge(3, 4, 3);
    g.add_edge(1, 2, 6);
    g.add_edge(4, 5, 6);
    g.add_edge(3, 6, 7);
    g.print();
    for e in g.dfs_iter() {
        println!("{e}")
    }
    // {(1, 3): 1, (1, 4): 2, (2, 4): 3, (2, 5): 2, (4, 6): 2}
}
