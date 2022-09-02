#[derive(Debug, PartialEq)]
pub enum HeapType {
    Min,
    Max,
}

pub trait IHeap<T: Ord + Copy> {
    fn left(index: usize) -> usize;
    fn new(heap_type: HeapType) -> Self;
    fn build(vec: Vec<T>, heap_type: HeapType) -> Self; /* O(nlogn) */
    fn build_fast(vec: Vec<T>, heap_type: HeapType) -> Self; /* O(n) */
    fn get_last_parent(&self) -> Option<usize>;
    fn right(index: usize) -> usize;
    fn get_most(&self, index: usize) -> Option<usize>;
    fn is_leaf(&self, index: usize) -> bool;
    fn fa(index: usize) -> Option<usize>;
    fn cmp(&self, k1: usize, k2: usize) -> bool;
    fn is_empty(&self) -> bool;
    fn insert(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn sink_down(&mut self, index: usize);
    fn swim_up(&mut self, index: usize);
}
#[derive(Debug, PartialEq)]
pub struct Heap<T> {
    pub arr: Vec<T>,
    pub heap_type: HeapType,
}

impl<T: Ord + Copy> IHeap<T> for Heap<T> {
    fn left(index: usize) -> usize {
        index * 2 + 1
    }
    fn new(heap_type: HeapType) -> Self {
        Self {
            arr: vec![],
            heap_type,
        }
    }

    fn build(vec: Vec<T>, heap_type: HeapType) -> Self {
        let mut h = Heap::new(heap_type);
        for i in 0..vec.len() {
            let value = vec[i];
            h.insert(value)
        }
        h
    }

    /*
             1
           12  5
         7 3  4
    */
    fn build_fast(vec: Vec<T>, heap_type: HeapType) -> Self {
        let mut h = Heap::new(heap_type);
        h.arr = vec;
        match h.get_last_parent() {
            None => h,
            Some(fa) => {
                for i in (0..=fa).rev() {
                    // println!("递减下沉,{i}");
                    h.sink_down(i);
                }
                h
            }
        }
    }

    fn get_last_parent(&self) -> Option<usize> {
        if self.arr.len() <= 1 {
            None
            /*
              空堆          12
                          /
                         1
            */
        } else {
            let most_ = Self::fa(self.arr.len() - 1);
            // println!("most_:{:?}", most_);
            most_
        }
    }

    fn right(index: usize) -> usize {
        index * 2 + 2
    }

    fn get_most(&self, index: usize) -> Option<usize> {
        if self.is_leaf(index) {
            // println!("是leaf ");
            None
        } else if self.arr.get(Self::right(index)).is_none() {
            Some(Self::left(index))
        } else {
            match self.cmp(Self::left(index), Self::right(index)) {
                true => Some(Self::right(index)),
                false => Some(Self::left(index)),
            }
        }
    }
    fn is_leaf(&self, index: usize) -> bool {
        self.arr.get(Self::left(index)).is_none()
    }

    fn fa(index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / 2)
        }
    }

    fn cmp(&self, k1: usize, k2: usize) -> bool {
        match self.heap_type {
            HeapType::Min => self.arr[k1] > self.arr[k2],
            HeapType::Max => self.arr[k1] < self.arr[k2],
        }
    }
    fn is_empty(&self) -> bool {
        self.arr.len() == 0
    }
    fn insert(&mut self, value: T) {
        self.arr.push(value);
        let last_index = self.arr.len() - 1;
        self.swim_up(last_index)
    }
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else if self.arr.len() == 1 {
            Some(self.arr.pop().unwrap())
        } else {
            let most = self.arr[0];
            self.arr[0] = self.arr.pop().unwrap();
            self.sink_down(0);
            Some(most)
        }
    }
    fn sink_down(&mut self, index: usize) {
        println!("{index}");
        match self.get_most(index) {
            Some(most_index) => {
                if self.cmp(index, most_index) {
                    self.arr.swap(index, most_index);
                    self.sink_down(most_index)
                }
            }
            None => (),
        }
    }

    fn swim_up(&mut self, index: usize) {
        match Self::fa(index) {
            Some(fa) => {
                if self.cmp(fa, index) {
                    self.arr.swap(fa, index);
                    self.swim_up(fa)
                }
            }
            None => (),
        }
    }
}

#[test]
fn heap_test() {
    // let mut h = Heap::build_fast(vec![1, 12, 5, 7, 3, 4], HeapType::Max);
    let h1 = Heap::build_fast(vec![1, 12, 5, 7, 3, 4], HeapType::Max);
    let h2 = Heap::build(vec![1, 12, 5, 7, 3, 4], HeapType::Max);
    println!("{}", h1 == h2);
    println!("{:?}", h1);
    println!("{:?}", h2);
    // dbg!(&h);
    // /*
    //      12
    //   7     5
    //  1 3  4
    // */
    // h.pop();
    // dbg!(&h);

    // h.pop();
    // dbg!(&h);

    // h.pop();
    // dbg!(&h);
    //    4
    //  3   1

    // arr: [
    //     4,
    //     3,
    //     1,
    // ],
}
