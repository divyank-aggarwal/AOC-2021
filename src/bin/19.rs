use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "19_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let array: Vec<&str> = file_contents.split("\n").collect();
    let mut values: HashMap<char, u32> = HashMap::new();
    let closer = HashMap::from([(')', '('), ('}', '{'), (']', '['), ('>', '<')]);
    values.insert(')', 3);
    values.insert(']', 57);
    values.insert('}', 1197);
    values.insert('>', 25137);
    let mut ans = 0;
    for instr in array {
        let runes: Vec<char> = instr.chars().collect();
        let mut fulfil: Vec<char> = vec![];
        let mut flag = false;
        let mut found = '(';
        for i in 0..runes.len() {
            match runes[i] {
                '(' | '{' | '[' | '<' => {
                    fulfil.push(runes[i]);
                }
                x => {
                    if let Some(y) = closer.get(&x) {
                        if fulfil.len() > 0 {
                            if fulfil[fulfil.len() - 1] == *y {
                                fulfil.pop();
                            } else {
                                flag = true;
                                found = x;
                                break;
                            }
                        } else {
                            flag = true;
                            found = x;
                            break;
                        }
                    }
                }
            }
        }
        if flag == true {
            ans += *values.get(&found).unwrap();
        }
    }
    println!("{}", ans);
}
