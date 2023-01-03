use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "25_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let init: Vec<&str> = file_contents.split("\n\n").collect::<Vec<&str>>();
    let pairs: Vec<(u32, u32)> = init[0]
        .split("\n")
        .map(|x| {
            let nos = x
                .split(",")
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (nos[0], nos[1])
        })
        .collect::<Vec<(u32, u32)>>();
    let first_instr = init[1]
        .split("\n")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()[2..]
        .to_vec()[0]
        .chars()
        .collect::<Vec<char>>()[0];
    let mut rows = 0;
    let mut cols = 0;
    for (x, y) in &pairs {
        if *x > cols {
            cols = *x + 1;
        }
        if *y > rows {
            rows = *y + 1;
        }
    }
    println!("{}   {}", rows, cols);
    let mut dot_map = vec![vec![false; cols as usize]; rows as usize];
    for (x, y) in &pairs {
        dot_map[*y as usize][*x as usize] = true;
    }
    let mut ans = 0;
    if first_instr == 'x' {
        do_fold(&mut dot_map, false, rows as usize, cols as usize);
        cols = cols / 2;
        ans = calculate_ans(&mut dot_map, rows as usize, cols as usize);
    } else {
        do_fold(&mut dot_map, true, rows as usize, cols as usize);
        rows = rows / 2;
        ans = calculate_ans(&mut dot_map, rows as usize, cols as usize);
    }
    println!("{}", ans);
}

fn do_fold(dot_map: &mut Vec<Vec<bool>>, is_row: bool, rows: usize, cols: usize) {
    let fold_line: usize = if is_row { rows / 2 } else { cols / 2 };
    if is_row {
        for i in 0..fold_line {
            for j in 0..cols {
                dot_map[i][j] = dot_map[rows - i - 1][j] || dot_map[i][j]
            }
        }
    } else {
        for i in 0..rows {
            for j in 0..fold_line {
                dot_map[i][j] = dot_map[i][j] || dot_map[i][cols - j - 1]
            }
        }
    }
}

fn calculate_ans(dot_map: &mut Vec<Vec<bool>>, rows: usize, cols: usize) -> u32 {
    let mut ans = 0;
    for i in 0..rows {
        for j in 0..cols {
            if dot_map[i][j] {
                ans += 1;
            }
        }
    }
    ans
}
