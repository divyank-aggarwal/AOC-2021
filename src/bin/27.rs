use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "27_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let init: Vec<&str> = file_contents.split("\n\n").collect::<Vec<&str>>();
    let mut str: String = init[0].chars().collect();
    let instructions: Vec<((char, char), char)> = init[1]
        .split("\n")
        .map(|x| {
            let strs = x.split(" -> ").collect::<Vec<&str>>();
            let res = strs[1].chars().collect::<Vec<char>>()[0];
            let test = strs[0].chars().collect::<Vec<char>>();
            ((test[0], test[1]), res)
        })
        .collect::<Vec<((char, char), char)>>();
    for i in 0..10 {
        let mut insert_pos: Vec<(usize, char)> = vec![];
        let mut ch_array = str.chars().collect::<Vec<char>>();
        let mut ctr = 0;
        for ch in 0..str.chars().collect::<Vec<char>>().len() - 1 {
            for instr in &instructions {
                if ch_array[ch] == instr.0 .0 && ch_array[ch + 1] == instr.0 .1 {
                    insert_pos.push((ch + 1 + ctr, instr.1));
                    ctr += 1;
                }
            }
        }
        for j in &insert_pos {
            ch_array.insert(j.0, j.1);
        }
        if insert_pos.len() != 0 {
            str = ch_array.into_iter().collect::<String>();
        }
    }
    let mut hash: HashMap<char, u64> = HashMap::new();
    for i in str.chars() {
        *hash.entry(i).or_insert(0) += 1;
    }
    let mut ans_arr: Vec<u64> = hash.iter().map(|x| *x.1).collect();
    ans_arr.sort();
    println!("{:?}", ans_arr);
}
