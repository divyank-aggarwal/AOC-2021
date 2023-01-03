use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "5_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut arr: [i32; 12] = [0; 12];
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let instructions: Vec<&str> = file_contents.split("\n").collect();
    for val in &instructions {
        for (i, c) in val.chars().enumerate() {
            if c == '1' {
                arr[i] += 1;
            }
        }
    }
    let len = instructions.len();
    println!("{:?}", arr);
    let mut epsilon: String = String::from("");
    let mut gamma: String = String::from("");
    for val in arr {
        if val > (len / 2).try_into().expect("please pass") {
            epsilon.push_str("1");
            gamma.push_str("0");
        } else {
            epsilon.push_str("0");
            gamma.push_str("1");
        }
    }
    let epsilon_no = isize::from_str_radix(&epsilon, 2).expect("Please pass");
    let gamma_no = isize::from_str_radix(&gamma, 2).unwrap();
    println!("{}", epsilon_no * gamma_no);
}
