use crate::heaps::heap::{Heap, HeapType, Unit};
use std::fmt::{Debug, Display};

pub trait PQueue<T, P> {
    fn enque(&mut self, unit: Unit<T, P>);
    fn deque(&mut self) -> Option<Unit<T, P>>;
}

#[derive(Debug)]
pub struct PriorityQue<T: Debug, P: Ord + Display + Copy> {
    heap: Heap<T, P>,
}

impl<T: Debug + Copy, P: Ord + Copy + Display> PriorityQue<T, P> {
    pub fn new() -> Self {
        Self {
            heap: Heap::new(HeapType::Max),
        }
    }
}

impl<T: Debug + Copy, P: Display + Ord + Copy> PQueue<T, P> for PriorityQue<T, P> {
    fn enque(&mut self, unit: Unit<T, P>) {
        self.heap.insert(unit)
        // self.heap
    }

    fn deque(&mut self) -> Option<Unit<T, P>> {
        self.heap.extract()
    }
}

#[test]
fn priorty_queue_test() {
    let mut tasks = PriorityQue::new();
    tasks.enque((1, Some("吃早饭")));
    tasks.enque((35, Some("看书")));
    tasks.enque((101, Some("打游戏")));
    tasks.enque((90, Some("开会")));
    tasks.enque((112, Some("看电影")));
    tasks.enque((1519, Some("洗漱")));
    println!("{:?}", tasks.heap.size());
    println!("{:?}", tasks.deque());
    println!("{:?}", tasks.deque());
    println!("{:?}", tasks.deque());
    println!("{:?}", tasks.deque());
    println!("{:?}", tasks.deque());
    println!("{:?}", tasks.deque());
}
