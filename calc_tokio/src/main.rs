mod get_top_k;
use calc_tokio::heap::MinHeap;
use get_top_k::get_top_k;
use std::{
    env::args,
    fs::File,
    io::{BufWriter, Write},
    process::exit,
};
use tokio::task;

#[tokio::main]
async fn main() {
    let num_of_threads = 16;
    let k = 100;

    let file_path = args().nth(1).expect("Please enter the file name");
    let file = File::open(&file_path).expect("Fail to read");
    let file_size: u64 = file.metadata().expect("Cannot get metadata").len();
    if file_size % 8 != 0 {
        panic!("File size is not multiple of 8, invalid f64 sequence");
    }
    let chunk_size = file_size / num_of_threads;
    drop(file);

    let mut handles = vec![];
    for i in 0..num_of_threads {
        let start = i * chunk_size;
        let end = if i == num_of_threads - 1 {
            file_size
        } else {
            (i + 1) * chunk_size
        };

        let file_path = file_path.clone();
        let handle = task::spawn_blocking(move || {
            println!("thread{i} start");
            let min_heap = get_top_k(k, &file_path, start, end);
            println!("thread{i} finished");
            min_heap
        });
        handles.push(handle);
    }

    let mut min_heap = handles.pop().unwrap().await.unwrap();
    if min_heap.len() != k {
        panic!("Get back to solve this");
    }

    for handle in handles {
        let received = handle.await.unwrap();
        for num in received.as_vec() {
            if *num > *min_heap.get_root().unwrap() {
                min_heap.pop();
                min_heap.insert(*num);
            }
        }
    }
    save_and_exit(&mut min_heap);
}

fn save_and_exit(min_heap: &mut MinHeap<f64>) {
    let file_path = "result";
    let file =
        File::create(file_path).unwrap_or_else(|_| panic!("Cannot open {file_path} for reading"));
    let mut file = BufWriter::new(file);

    let mut result = min_heap.as_vec().clone();
    result.sort_by(|a, b| b.partial_cmp(a).unwrap());
    for num in result {
        let bytes = num.to_le_bytes();
        file.write_all(&bytes)
            .unwrap_or_else(|_| panic!("Fail to write {num} when saving"));
    }
    file.flush().expect("Fail to flush");
    exit(0);
}
