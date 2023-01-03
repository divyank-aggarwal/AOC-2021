use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::Mutex;
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
    let mut ans: Vec<u32> = vec![];
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
                let visit = Rc::new(Mutex::new(vec![vec![false; cols]; rows]));
                ans.push(traverse(i, j, rows, cols, &array, Rc::clone(&visit)));
            }
        }
    }
    ans.sort();
    ans.reverse();
    let res = ans[0] * ans[1] * ans[2];
    println!("{}", res);
}

fn traverse(
    i: usize,
    j: usize,
    rows: usize,
    cols: usize,
    arr: &Vec<Vec<char>>,
    visit: Rc<Mutex<Vec<Vec<bool>>>>,
) -> u32 {
    let mut cmp: Vec<(usize, usize)> = vec![];
    {
        let mut visit = visit.lock().unwrap();
        if visit[i][j] == true {
            return 0;
        }
        visit[i][j] = true;
        if arr[i][j] == '9' {
            return 0;
        }
        if j < cols - 1 {
            if visit[i][j + 1] == false {
                cmp.push((i, j + 1));
            }
        }
        if j > 0 {
            if visit[i][j - 1] == false {
                cmp.push((i, j - 1));
            }
        }
        if i < rows - 1 {
            if visit[i + 1][j] == false {
                cmp.push((i + 1, j));
            }
        }
        if i > 0 {
            if visit[i - 1][j] == false {
                cmp.push((i - 1, j));
            }
        }
    }
    let mut ans: u32 = 1;
    for val in cmp {
        ans += traverse(val.0, val.1, rows, cols, arr, Rc::clone(&visit));
    }
    ans
}
