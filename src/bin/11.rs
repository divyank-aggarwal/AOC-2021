use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "11_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let mut nos: Vec<u32> = file_contents
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let days = 80;
    let mut count = nos.len();
    for _ in 0..days {
        let mut cycle_ct = 0;
        for x in nos.iter_mut() {
            if *x == 0 {
                count = count + 1;
                cycle_ct += 1;
                *x = 6
            } else {
                *x = *x - 1;
            }
        }
        for _ in 0..cycle_ct {
            nos.push(8);
        }
    }
    println!("{}", count);
    // let mut map: HashMap<u32, u32> = HashMap::new();
    // for no in nos {
    //     *map.entry(no).or_insert(0) += 1;
    // }
    // let days: u32 = 18;
    // let period: u32 = 7;
    // let first_period: u32 = 9;
    // let mut ans: u32 = 0;
    // for (key, val) in map {
    //     println!("{} {}", key, val);
    //     ans += val * find_hatchlings(key, days, period, first_period);
    // }
    // println!("{}", ans);
}

// fn find_hatchlings(init: u32, days: u32, period: u32, first_period: u32) -> u32 {
//     let mut ans: u32 = 0;
//     if init as i32 - days as i32 >= 0 {
//         return 0;
//     }
//     ans += (days - init - 1) / first_period + 1 + u32::pow(days - init - 1, 2) / period;
//     let n: u32 = (days - init - 1) / first_period;
//     ans -= (1 + n) * (days - init - 1) / 2;
//     ans
// }
