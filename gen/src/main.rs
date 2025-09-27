use rand::random;
use std::{
    env::args,
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let file_path = args().nth(1).expect("Please enter the file name");
    println!("write into: {}", file_path);
    let file =
        File::create(&file_path).unwrap_or_else(|_| panic!("Cannot open {file_path} for writing"));
    let mut file = BufWriter::new(file);

    let size: usize = (20) * 1024 * 1024 * 1024; // GB
    let num_of_numbers = size / 8; // f64 takes 8 bytes

    for i in 0..num_of_numbers {
        let num: f64 = random();
        let bytes = num.to_le_bytes();
        file.write_all(&bytes)
            .unwrap_or_else(|_| panic!("Fail to write {:?} in {}th loop", bytes, i));
    }
    file.flush().expect("Fail to flush");
}
