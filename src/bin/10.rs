use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "9_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let instructions: Vec<&str> = file_contents.split("\n").collect();
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut ans: u32 = 0;
    for instr in &instructions {
        let points: Vec<&str> = instr.split(" -> ").collect();
        let init: Vec<&str> = points[0].split(",").collect();
        let end: Vec<&str> = points[1].split(",").collect();
        let x_init = init[0].parse::<i32>().unwrap();
        let y_init = init[1].parse::<i32>().unwrap();
        let x_end = end[0].parse::<i32>().unwrap();
        let y_end = end[1].parse::<i32>().unwrap();
        if x_init != x_end && y_init != y_end {
            println!("{}", instr);
            for (i, _) in (min(y_init, y_end)..(max(y_init, y_end) + 1)).enumerate() {
                let x_flag: i32 = if x_init < x_end { 1 } else { -1 };
                let y_flag: i32 = if y_init < y_end { 1 } else { -1 };
                *map.entry((x_init + (i as i32) * x_flag, y_init + (i as i32) * y_flag))
                    .or_insert(0) += 1;
                if *map
                    .get(&(x_init + (i as i32) * x_flag, y_init + (i as i32) * y_flag))
                    .unwrap()
                    == 2_u32
                {
                    ans += 1;
                }
            }
        } else if x_init == x_end {
            for i in min(y_init, y_end)..(max(y_init, y_end) + 1) {
                *map.entry((x_init, i)).or_insert(0) += 1;
                if *map.get(&(x_init, i)).unwrap() == 2_u32 {
                    ans += 1;
                }
            }
        } else if y_init == y_end {
            for i in min(x_init, x_end)..(max(x_init, x_end) + 1) {
                *map.entry((i, y_init)).or_insert(0) += 1;
                if *map.get(&(i, y_init)).unwrap() == 2_u32 {
                    ans += 1;
                }
            }
        }
        println!("{}", ans)
    }
    println!("{}", ans);
}
