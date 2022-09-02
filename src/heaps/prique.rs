use super::heap::{Heap, HeapType, IHeap};

trait IPriorityQue<T: Ord + Copy> {
    fn enque(&mut self, item: T);
    fn deque(&mut self) -> Option<T>;
}

pub struct PriorityQue<T: Ord + Copy> {
    heap: Heap<T>,
    length: usize,
}

impl<T: Ord + Copy> PriorityQue<T> {
    pub fn new() -> Self {
        Self {
            heap: Heap::new(HeapType::Max),
            length: 0,
        }
    }
}

impl<T: Ord + Copy> IPriorityQue<T> for PriorityQue<T> {
    fn enque(&mut self, priority: T) {
        self.length += 1;
        self.heap.insert(priority);
    }

    fn deque(&mut self) -> Option<T> {
        self.heap.pop()
    }
}

#[test]
fn pq_test() {
    let mut pq = PriorityQue::new();
    pq.enque(12);
    pq.enque(2);
    pq.enque(5);
    pq.enque(4);
    pq.enque(6);
    println!("{:?}", pq.deque());
    println!("{:?}", pq.deque());
    println!("{:?}", pq.deque());
    println!("{:?}", pq.deque());
    
}
