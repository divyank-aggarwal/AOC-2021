use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "13_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let mut nos: Vec<u32> = file_contents
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    nos.sort();
    println!("{:?}", nos);
    let size = nos.len();
    let mut min: u64 = 10000000000;
    for i in nos[0]..(nos[size - 1] + 1) {
        let mut curr: u64 = 0;
        for no in &nos {
            curr = curr + series_sum(no.abs_diff(i));
        }
        if curr < min {
            min = curr;
        }
    }
    println!("{}", min);
}

fn series_sum(no: u32) -> u64 {
    return (no as u64) * ((no as u64) + 1) / 2;
}
