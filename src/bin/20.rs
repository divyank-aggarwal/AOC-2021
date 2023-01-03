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
    let mut values: HashMap<char, u64> = HashMap::new();
    let closer = HashMap::from([(')', '('), ('}', '{'), (']', '['), ('>', '<')]);
    let mut opener: HashMap<char, char> = HashMap::new();
    for (k, v) in &closer {
        opener.insert(*v, *k);
    }
    values.insert(')', 1);
    values.insert(']', 2);
    values.insert('}', 3);
    values.insert('>', 4);
    let mut ans: Vec<u64> = vec![];
    for instr in array {
        let runes: Vec<char> = instr.chars().collect();
        let mut fulfil: Vec<char> = vec![];
        let mut flag = true;
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
                                flag = false;
                                break;
                            }
                        } else {
                            flag = false;
                            break;
                        }
                    }
                }
            }
        }
        if fulfil.len() > 0 && flag == true {
            let mut score: u64 = 0;
            let closing: Vec<&char> = fulfil
                .iter()
                .rev()
                .map(|x| opener.get(x).unwrap())
                .collect();
            for cl in closing {
                score = score * 5 + values.get(cl).unwrap();
            }
            ans.push(score);
        }
    }
    ans.sort();
    println!("{:?}", ans);
    println!("{}", ans[ans.len() / 2])
}
