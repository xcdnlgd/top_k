use calc_mmap::heap::MinHeap;

pub fn get_top_k(k: usize, slice: &[f64]) -> MinHeap<f64> {
    let mut min_heap: MinHeap<f64> = MinHeap::new();
    for &num in slice {
        if min_heap.len() < k {
            min_heap.insert(num);
        } else if num > *min_heap.get_root().unwrap() {
            min_heap.pop();
            min_heap.insert(num);
        }
    }
    min_heap
}

