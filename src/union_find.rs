// 并查集 -> KrusKal

pub struct UnionFind {
    pub fa: Vec<usize>,
}

impl UnionFind {
    pub fn init(len: usize) -> Self {
        let mut fa = vec![0; len];
        for i in 0..len {
            fa[i] = i;
        }
        Self { fa }
    }
    pub fn find(&mut self, idx: usize) -> usize {
        if idx == self.fa[idx] {
            idx
        } else {
            // println!("向上查找");
            self.fa[idx] = self.find(self.fa[idx]);
            return self.fa[idx];
        }
    }
    pub fn union(&mut self, left: usize, right: usize) -> bool {
        let left_root = self.find(left);
        let right_root = self.find(right);
        if left_root == right_root {
            false
        } else {
            self.fa[left_root] = right_root;
            true
        }
    }
}
