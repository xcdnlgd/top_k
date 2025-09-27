use std::io::SeekFrom;

use calc_tokio::heap::MinHeap;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt, BufReader},
};

pub async fn get_top_k(k: usize, file_path: &str, start: u64, end: u64) -> MinHeap<f64> {
    let mut file = File::open(file_path)
        .await
        .unwrap_or_else(|_| panic!("Cannot open {file_path} for reading"));
    file.seek(SeekFrom::Start(start))
        .await
        .unwrap_or_else(|_| panic!("Fail to seek to {start}"));
    let mut file = BufReader::new(file);
    let mut min_heap: MinHeap<f64> = MinHeap::new();
    let mut buffer = [0u8; 8];
    let num_of_numbers = (end - start) / 8;
    println!("num: {num_of_numbers}");

    while min_heap.len() < k {
        if file.read_exact(&mut buffer).await.is_err() {
            return min_heap;
        }
        let num = f64::from_le_bytes(buffer);
        min_heap.insert(num);
    } // read first k numbers

    let mut count: u64 = k as u64;

    // while count < num_of_numbers {
    //     file.read_exact(&mut buffer).expect(&format!("Wrong calculation lead to reading failure, count={count}"));
    while file.read_exact(&mut buffer).await.is_ok() {
        count += 1;
        let num = f64::from_le_bytes(buffer);
        if num > *min_heap.get_root().unwrap() {
            min_heap.pop();
            min_heap.insert(num);
        }
        if count == num_of_numbers {
            break;
        }
    } // process rest numbers
    min_heap
}
