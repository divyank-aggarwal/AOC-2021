use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "15_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let instr: Vec<Vec<&str>> = file_contents
        .split("\n")
        .map(|x| x.split(" | ").collect())
        .collect();
}
