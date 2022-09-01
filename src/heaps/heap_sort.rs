use crate::heaps::heap::Heap;
use crate::heaps::heap::HeapType;

pub fn heap_sort<T: Ord + Copy>(arr: Vec<T>) -> Vec<T> {
    let mut heap: Heap<usize, T> = Heap::new(HeapType::Min);
    heap.insert_from_arr(&arr[..]);
    let mut result = vec![];
    while let Some((priority, _)) = heap.extract() {
        result.push(priority);
    }
    result
}

#[test]
fn heap_sort_test() {
    let _vec = vec![2, 3, 6, 5, 4, 1, 9, 78, 6, 5, 7, 1];
    println!("{:?}", heap_sort(_vec));
}
