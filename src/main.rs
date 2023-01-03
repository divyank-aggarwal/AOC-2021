use std::fs::File;
use std::io::Read;

fn main() {
    let filename = "1_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let numbers: Vec<u32> = file_contents
        .split("\n")
        .map(|s: &str| s.parse::<u32>().expect("could not parse into u32"))
        .collect();
    println!("{}", numbers.len());
}
