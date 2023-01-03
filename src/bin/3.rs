use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "3_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut x = 0;
    let mut y = 0;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let instructions: Vec<&str> = file_contents.split("\n").collect();
    for val in instructions {
        let val_pair: Vec<&str> = val.split_whitespace().collect();
        if val_pair[0].eq("forward") == true {
            x += val_pair[1].parse::<i32>().expect("Please pass");
        } else if val_pair[0].eq("down") == true {
            y += val_pair[1].parse::<i32>().expect("Please pass");
        } else {
            y -= val_pair[1].parse::<i32>().expect("Please pass");
        }
    }
    println!("{}    {}", x, y);
    println!("{}", x * y);
}
