use std::{collections::VecDeque, fmt::Debug};

use crate::graph_matrix::Graph;

pub struct DFSIterator<'a, T: Ord + Debug> {
    pub graph: &'a Graph<T>,
    pub stk: Vec<usize>,
    pub visited: Vec<bool>,
}

//  [ 0 1 2 3 4 ... ] -> pop
//                    <- push

//    dequeue<= [0 1 2 3 ]
//[ 0 1 2 3 ] <=enqueue
//  [ 1,2,3,4].shift    [2,3,4]
/*
   Vecdeque
*/

impl<'a, T: Ord + Debug> Iterator for DFSIterator<'a, T> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.graph.is_empty() {
            return None;
        } else {
            while !self.stk.is_empty() {
                let v = self.stk.pop().unwrap();
                if self.visited[v] {
                    continue;
                }
                self.visited[v] = true;
                for e in self.graph.neighbour(v) {
                    if !self.visited[e] {
                        self.stk.push(e)
                    }
                }
                return Some(v);
            }
            None
        }
    }
}

pub struct BFSIterator<'a, T: Ord + Debug> {
    pub graph: &'a Graph<T>,
    pub que: VecDeque<usize>,
    pub visited: Vec<bool>,
}

impl<'a, T: Ord + Debug> Iterator for BFSIterator<'a, T> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.graph.is_empty() {
            return None;
        } else {
            while !self.que.is_empty() {
                let v = self.que.pop_front().unwrap();
                if self.visited[v] {
                    continue;
                }
                self.visited[v] = true;
                for e in self.graph.neighbour(v) {
                    if !self.visited[e] {
                        self.que.push_back(e)
                    }
                }
                return Some(v);
            }
            None
        }
    }
}

#[test]
fn _test() {
    let v: VecDeque<usize> = VecDeque::new();
    // v.pop_front()
}
