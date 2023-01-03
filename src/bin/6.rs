use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "5_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let mut instructions: Vec<&str> = file_contents.split("\n").collect();
    let mut gamma_instr = instructions.clone();
    let mut ctr: usize = 0;
    while instructions.len() > 1 {
        let len = instructions.len();
        println!("{}", len);
        let mut sum = 0;
        for val in &instructions {
            if val.chars().nth(ctr).unwrap() == '1' {
                sum += 1;
            }
        }
        let found = if sum * 2 >= len { '1' } else { '0' };
        instructions = instructions
            .into_iter()
            .filter(|x| x.chars().nth(ctr).unwrap() == found)
            .collect();
        ctr += 1;
    }
    let epsilon = instructions[0];
    ctr = 0;
    while gamma_instr.len() > 1 {
        let len = gamma_instr.len();
        println!("{}", len);
        let mut sum = 0;
        for val in &gamma_instr {
            if val.chars().nth(ctr).unwrap() == '1' {
                sum += 1;
            }
        }
        let found = if sum * 2 < len { '1' } else { '0' };
        gamma_instr = gamma_instr
            .into_iter()
            .filter(|x| x.chars().nth(ctr).unwrap() == found)
            .collect();
        ctr += 1;
    }
    let gamma = gamma_instr[0];

    let epsilon_no = isize::from_str_radix(&epsilon, 2).expect("Please pass");
    let gamma_no = isize::from_str_radix(&gamma, 2).unwrap();
    println!("{}", epsilon_no * gamma_no);
}
