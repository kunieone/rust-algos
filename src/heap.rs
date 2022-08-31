enum HeapType {
    MaxHeap,
    MinHeap,
}
type Node<T> = (usize, T);
struct Heap<T> {
    datas: Vec<Node<T>>,
    heap_type: HeapType,
    len: usize,
}

impl<T: Ord + Copy> Heap<T> {
    pub fn new(heap_type: HeapType) -> Self {
        Self {
            datas: vec![],
            heap_type,
            len: 0,
        }
    }
    pub fn get_left(&self, nde: &Node<T>) -> Option<&Node<T>> {
        if nde.0 + 1 > self.len {
            None
        } else {
            Some(self.datas[nde.0 * 2 + 1])
        }
    }
    pub fn get_right(&self, nde: &Node<T>) -> Option<&Node<T>> {
        if nde.0 + 1 > self.len {
            None
        } else {
            Some(self.datas[nde.0 * 2 + 2])
        }
    }
    pub fn get_father(&self, nde: &Node<T>) -> Option<&Node<T>> {
        if nde.0 + 1 > self.len || nde.0 == 0 {
            None
        } else {
            Some(&self.datas[(nde.0 / 2) as usize])
        }
    }
    pub fn insert(&mut self, nde: Node<T>) {
        // 3 -> 0,1,2 max_idx:2
        let index = nde.0;
        self.datas.push(nde);
        self.len += 1;
        self.heapify(index);
    }

    pub fn swap(&mut self, key1: usize, key2: usize) -> bool {
        if key1.max(key2) + 1 > self.len {
            false
        } else {
            self.datas.swap(key1, key2);
            true
        }
    }
    pub fn heapify(&mut self, mut index: usize) -> bool {
        if index + 1 > self.len || index == 0 {
            return false;
        }
        let is_changed = false;
        let nde = &self.datas[index];
        let fa = &self.get_father(nde);
        match fa {
            Some(mut fa_node) => {}
            None => (),
        }

        is_changed
    }
}
