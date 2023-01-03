use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;

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
        .ok()
        .expect("failed to read!");
    let arr: Vec<Vec<u32>> = file_contents
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let rows = arr.len();
    let cols = arr[0].len();
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
            if min_arr[i][j] > curr.min + arr[i][j] {
                queue.push(Node {
                    i,
                    j,
                    min: curr.min + arr[i][j],
                });
                min_arr[i][j] = curr.min + arr[i][j];
            }
        }
    }
    println!("{}", ans);
}
