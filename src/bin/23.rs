use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
fn main() {
    let filename = "23_input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let intstuctions: Vec<&str> = file_contents.split("\n").collect();
    let mut matrix: HashMap<&str, Vec<&str>> = HashMap::new();
    for instr in intstuctions {
        let vals = instr.split("-").collect::<Vec<&str>>();
        matrix.entry(vals[0]).or_default().push(vals[1]);
        matrix.entry(vals[1]).or_default().push(vals[0]);
    }
    println!("{}", find_paths("start", &matrix, vec![]))
}

fn find_paths<'a>(
    point: &'a str,
    matrix: &HashMap<&str, Vec<&str>>,
    mut visit: Vec<&'a str>,
) -> u32 {
    if point == &"end"[..] {
        return 1;
    }
    if point
        .chars()
        .filter(|x| x.is_ascii_uppercase())
        .collect::<Vec<char>>()
        .len()
        == 0
    {
        visit.push(point);
    }
    let nodes: Vec<&str> = match matrix.get(point) {
        Some(x) => x.clone(),
        None => vec![] as Vec<&str>,
    };
    let mut nodes_to_visit: Vec<&str> = vec![];
    for node in nodes {
        if !visit.contains(&node) {
            nodes_to_visit.push(node);
        }
    }
    if nodes_to_visit.len() == 0 {
        return 0;
    } else {
        let mut sum: u32 = 0;
        for node in nodes_to_visit {
            sum += find_paths(node, matrix, visit.clone());
        }
        return sum;
    }
}
