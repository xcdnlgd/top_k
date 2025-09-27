use calc_tokio::heap::MinHeap;

#[test]
fn test_min_heap() {
    let mut min_heap: MinHeap<f64> = MinHeap::new();
    min_heap.insert(3.0);
    min_heap.insert(6.0);
    min_heap.insert(7.0);
    min_heap.insert(2.0);
    min_heap.insert(1.0);
    min_heap.insert(4.0);
    min_heap.insert(5.0);
    assert_eq!(Some(&1.0), min_heap.get_root());
    println!("{:?}", min_heap);
    assert_eq!(Some(1.0), min_heap.pop());
    assert_eq!(Some(2.0), min_heap.pop());
    assert_eq!(Some(3.0), min_heap.pop());
    assert_eq!(Some(4.0), min_heap.pop());
    assert_eq!(Some(5.0), min_heap.pop());
    assert_eq!(Some(6.0), min_heap.pop());
    assert_eq!(Some(7.0), min_heap.pop());
    assert_eq!(None, min_heap.pop());
}
