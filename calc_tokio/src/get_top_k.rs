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

    for _ in 0..num_of_numbers {
        if file.read_exact(&mut buffer).await.is_err() {
            break;
        }
        let num = f64::from_le_bytes(buffer);
        if min_heap.len() < k {
            min_heap.insert(num);
        }
        else if num > *min_heap.get_root().unwrap() {
            min_heap.pop();
            min_heap.insert(num);
        }
    }
    min_heap
}
