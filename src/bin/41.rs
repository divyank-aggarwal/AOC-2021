use std::cmp::min;
use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "41_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let arr = file_contents
        .split("\n")
        .map(|x| {
            x.split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect::<Vec<u32>>();
    let mut first = arr[0];
    let mut first_score = 0;
    let mut second = arr[1];
    let mut second_score = 0;
    let mut flag = true;
    let mut roll = 1;
    let ans;
    loop {
        if first_score >= 1000 || second_score >= 1000 {
            ans = min(first_score, second_score) * (roll - 1);
            break;
        }
        if flag {
            first += roller(roll, 100) + roller(roll + 1, 100) + roller(roll + 2, 100);
            first_score += roller(first, 10);
        } else {
            second += roller(roll, 100) + roller(roll + 1, 100) + roller(roll + 2, 100);
            second_score += roller(second, 10);
        }
        flag = !flag;
        roll += 3;
    }
    println!("{}", ans);
}

fn roller(no: u32, base: u32) -> u32 {
    if no % base == 0 {
        base
    } else {
        no % base
    }
}
