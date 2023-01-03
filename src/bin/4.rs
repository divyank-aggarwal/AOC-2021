use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "3_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut x: i64 = 0;
    let mut aim: i64 = 0;
    let mut y: i64 = 0;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let instructions: Vec<&str> = file_contents.split("\n").collect();
    for val in instructions {
        let val_pair: Vec<&str> = val.split_whitespace().collect();
        if val_pair[0].eq("forward") == true {
            x += val_pair[1].parse::<i64>().expect("Please pass");
            y += val_pair[1].parse::<i64>().expect("Please pass") * aim;
        } else if val_pair[0].eq("down") == true {
            aim += val_pair[1].parse::<i64>().expect("Please pass");
        } else {
            aim -= val_pair[1].parse::<i64>().expect("Please pass");
        }
    }
    println!("{}    {}", x, y);
    println!("{}", x * y);
}
