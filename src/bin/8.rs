use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "7_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut callout = vec![];
    let mut file_contents = String::new();
    let mut boards: Vec<[[i32; 5]; 5]> = vec![];
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let instructions: Vec<&str> = file_contents.split("\n\n").collect();
    let mut flag = true;
    for (_, instr) in instructions.iter().enumerate() {
        if flag == true {
            let nos: Vec<&str> = instr.split(",").collect();
            callout = nos.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
            flag = false;
        } else {
            let rows: Vec<&str> = instr.split("\n").collect();
            let mut board: [[i32; 5]; 5] = [[0; 5]; 5];
            for (inner, row) in rows.iter().enumerate() {
                let mut temp: [i32; 5] = [0; 5];
                let nos: Vec<&str> = row.split_whitespace().collect();
                for (i, no) in nos.iter().enumerate() {
                    temp[i] = no.parse::<i32>().unwrap();
                }
                board[inner] = temp;
            }
            boards.push(board);
        }
    }
    let mut scores: Vec<i32> = vec![];
    for board in &boards {
        scores.push(calculate_score(board, &callout));
    }
    println!("{:?}", scores);
    let mut min = 0;
    let mut min_ind = 0;
    for (ind, val) in scores.iter().enumerate() {
        if *val < min {
            min = *val;
            min_ind = ind;
        }
    }
    println!("{}", calculate_ans(&boards[min_ind], &callout, min))
}

fn calculate_score(board: &[[i32; 5]; 5], callout: &Vec<i32>) -> i32 {
    let mut winner = i32::MAX;
    // row check
    for i in 0..5 {
        let mut count = 0;
        for (j, val) in callout.iter().enumerate() {
            if board[i].contains(val) {
                count += 1;
            }
            if count == 5 {
                if winner > (j as i32) {
                    winner = j as i32;
                }
                break;
            }
        }
    }

    // col check
    for i in 0..5 {
        let mut count = 0;
        for (j, val) in callout.iter().enumerate() {
            if [
                board[0][i],
                board[1][i],
                board[2][i],
                board[3][i],
                board[4][i],
            ]
            .contains(val)
            {
                count += 1;
            }
            if count == 5 {
                if winner > (j as i32) {
                    winner = j as i32;
                }
                break;
            }
        }
    }

    winner
}

fn calculate_ans(board: &[[i32; 5]; 5], callout: &Vec<i32>, min: i32) -> i32 {
    let val = callout[min as usize];
    let r = (min + 1) as usize;
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !callout[0..r].to_vec().contains(&board[i][j]) {
                sum += board[i][j];
            }
        }
    }
    sum * val
}
