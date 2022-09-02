/*
Rust算法补完计划 #6
内容: 实现选择排序 --引出--> 堆 ->实现最大/最小堆 -->完成堆排序 --> 实现优先队列
*/

pub fn select_sort<T: Ord>(mut arr: Vec<T>) -> Vec<T> {
    let length = arr.len();
    for i in 0..length {
        let mut min = i;
        for j in (i + 1)..length {
            if arr[min] > arr[j] {
                min = j;
            }
        }
        arr.swap(min, i);
    }
    arr
}

#[test]
fn select_sort_test() {
    let test_arr = vec![5, 6, 3, 7, 2, 9, 1, 0, 8, 4];
    println!("{:?}", select_sort(test_arr))
}
