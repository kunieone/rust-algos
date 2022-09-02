pub fn merge<T: Ord + Copy>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    let mut i = 0; //left
    let mut j = 0; //right
    let mut result = vec![];
    // 按照从小到大的顺序push，直到某一边数组push完。
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    // 剩余有序数组中的元素push到result
    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}

pub fn merget_sort<T: Ord + Copy>(arr: Vec<T>) -> Vec<T> {
    if arr.len() == 1 {
        return arr;
    }
    let mid = arr.len() / 2;
    let left = arr[0..mid].to_vec();
    let right = arr[mid..].to_vec();
    merge(&merget_sort(left), &merget_sort(right))
}

#[test]
fn heap_srot_test() {
    let test = vec![3, 5, 2, 1, 7, 6, 5, 9, 8, 0];
    let sorted = merget_sort(test);
    println!("{:?}", sorted);
    assert_eq!(vec![0, 1, 2, 3, 5, 5, 6, 7, 8, 9], sorted);
}
