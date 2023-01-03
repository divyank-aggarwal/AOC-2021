use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
fn main() {
    let filename = "21_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let arr: Mutex<Vec<Vec<char>>> = Mutex::new(
        file_contents
            .split("\n")
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );
    let rows: usize = arr.lock().unwrap().len();
    let cols: usize = arr.lock().unwrap()[0].len();
    let days = 100;
    let mut ans: u32 = 0;
    for _ in 0..days {
        ans += treat(&arr, rows, cols);
    }
    println!("{}", ans);
}

fn treat(arr: &Mutex<Vec<Vec<char>>>, rows: usize, cols: usize) -> u32 {
    let visit = Mutex::new(vec![vec![false; cols]; rows]);
    let mut flash: Vec<(usize, usize)> = vec![];
    {
        let mut arr = arr.lock().unwrap();
        for i in 0..rows {
            for j in 0..cols {
                if arr[i][j] == '9' {
                    arr[i][j] = 'X';
                    flash.push((i, j));
                } else {
                    arr[i][j] = char::from_digit(arr[i][j].to_digit(10).unwrap() + 1, 10).unwrap();
                }
            }
        }
    }
    let mut ans: u32 = 0;
    for val in flash {
        ans += do_flash(arr, &visit, val.0, val.1, rows, cols);
    }

    let mut arr = arr.lock().unwrap();
    for i in 0..rows {
        for j in 0..cols {
            if arr[i][j] == 'X' {
                arr[i][j] = '0';
            }
        }
    }
    ans
}

fn do_flash(
    arr: &Mutex<Vec<Vec<char>>>,
    visit: &Mutex<Vec<Vec<bool>>>,
    x: usize,
    y: usize,
    rows: usize,
    cols: usize,
) -> u32 {
    let mut flash: Vec<(usize, usize)> = vec![];
    if visit.lock().unwrap()[x][y] == true {
        return 0;
    } else {
        visit.lock().unwrap()[x][y] = true;
        let mut arr = arr.lock().unwrap();
        let x = x as isize;
        let y = y as isize;
        for i in x - 1..x + 2 {
            for j in y - 1..y + 2 {
                if i >= 0 && i < (rows as isize) && j >= 0 && j < (cols as isize) {
                    if arr[i as usize][j as usize] == 'X' {
                        continue;
                    } else if arr[i as usize][j as usize] == '9' {
                        arr[i as usize][j as usize] = 'X';
                        flash.push((i as usize, j as usize));
                    } else {
                        arr[i as usize][j as usize] = char::from_digit(
                            arr[i as usize][j as usize].to_digit(10).unwrap() + 1,
                            10,
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
    let mut ans: u32 = 1;
    for val in flash {
        if visit.lock().unwrap()[val.0][val.1] == true {
            continue;
        } else {
            ans += do_flash(arr, visit, val.0, val.1, rows, cols)
        }
    }
    ans
}
