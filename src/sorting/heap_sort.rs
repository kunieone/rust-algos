use crate::heaps::heap::{self, IHeap};

pub fn heap_sort<T: Ord + Copy>(arr: Vec<T>) -> Vec<T> {
    let mut h = heap::Heap::build_fast(arr, heap::HeapType::Min);
    let mut res = vec![];

    while let Some(e) = h.pop() {
        res.push(e);
    }

    res
}

#[test]

fn _test() {
    let test = vec![3, 5, 2, 1, 7, 6, 5, 9, 8, 0];
    let sorted = heap_sort(test);
    println!("{:?}", sorted);
    assert_eq!(vec![0, 1, 2, 3, 5, 5, 6, 7, 8, 9], sorted);
}
