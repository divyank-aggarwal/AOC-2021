use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;
use std::vec;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    i: usize,
    j: usize,
    min: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.min.cmp(&self.min)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let filename = "29_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("failed to read!");
    let arr: Vec<Vec<u32>> = file_contents
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut rows = arr.len();
    let mut cols = arr[0].len();
    let mut bigger_arr = vec![vec![0; cols * 5]; rows * 5];
    for i in 0..5 * rows {
        for j in 0..5 * cols {
            let x = i % rows;
            let y = j % cols;
            let add = (i / rows + j / cols) as u32;
            let val = if arr[x][y] + add >= 10 {
                (arr[x][y] + 1 + add) % 10
            } else {
                arr[x][y] + add
            };
            bigger_arr[i][j] = val;
        }
    }
    rows = rows * 5;
    cols = cols * 5;
    let mut min_arr: Vec<Vec<u32>> = vec![vec![60000; cols]; rows];
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let ans;
    queue.push(Node { i: 0, j: 0, min: 0 });
    min_arr[0][0] = 0;
    loop {
        let curr = queue.pop().unwrap();
        let mut check_arr: Vec<(usize, usize)> = vec![];
        if curr.i == rows - 1 && curr.j == cols - 1 {
            ans = curr.min;
            break;
        }
        if curr.i > 0 {
            check_arr.push((curr.i - 1, curr.j));
        }
        if curr.i < rows - 1 {
            check_arr.push((curr.i + 1, curr.j));
        }
        if curr.j > 0 {
            check_arr.push((curr.i, curr.j - 1));
        }
        if curr.j < cols - 1 {
            check_arr.push((curr.i, curr.j + 1));
        }

        for (i, j) in check_arr {
            if min_arr[i][j] > curr.min + bigger_arr[i][j] {
                queue.push(Node {
                    i,
                    j,
                    min: curr.min + bigger_arr[i][j],
                });
                min_arr[i][j] = curr.min + bigger_arr[i][j];
            }
        }
    }
    println!("{}", ans);
}
