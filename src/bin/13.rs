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
    let sum: u32 = nos.iter().sum();
    println!("{}", sum);
    let mut sum_arr: Vec<u32> = vec![0, nos[0]];
    let size = nos.len();
    for (i, val) in nos.iter().enumerate() {
        if i == 0 {
            continue;
        } else {
            sum_arr.push(sum_arr[i] + *val);
        }
    }
    let mut n = 0;
    let mut min: u32 = 100000000;
    for i in nos[0]..(nos[size - 1] + 1) {
        while nos[n] < i {
            n += 1;
        }
        let calc: u32 = (2 * (n as u32) * i) + sum - ((size as u32) * i) - (2 * sum_arr[n]);
        println!("{} {} {}", i, calc, n);
        if calc < min {
            min = calc;
        }
    }
    println!("{}", min);
}
