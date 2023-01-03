use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "17_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let array: Vec<Vec<char>> = file_contents
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let rows = array.len();
    let cols = array[0].len();
    let mut ans = 0;
    for i in 0..rows {
        for j in 0..cols {
            let mut cmp: Vec<char> = vec![];
            if j < cols - 1 {
                cmp.push(array[i][j + 1])
            }
            if j > 0 {
                cmp.push(array[i][j - 1])
            }
            if i < rows - 1 {
                cmp.push(array[i + 1][j])
            }
            if i > 0 {
                cmp.push(array[i - 1][j])
            }
            let mut flag = true;
            for val in cmp {
                if array[i][j] >= val {
                    flag = false;
                    break;
                }
            }
            if flag == true {
                ans = ans + 1 + array[i][j].to_digit(10).unwrap();
            }
        }
    }
    println!("{}", ans);
}
