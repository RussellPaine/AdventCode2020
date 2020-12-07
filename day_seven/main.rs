use std::fs;

fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut valid_bags: Vec<&str> = Vec::new();

    for line in &lines {
        let bags: Vec<&str> = line.split(" bags contain").collect();
        if bags[1].contains("shiny gold") {
            valid_bags.push(bags[0]);
        }
    }

    let mut done = false;

    let mut valsize = valid_bags.len();

    while !done {
        
        for line in &lines {
            let bags: Vec<&str> = line.split(" bags contain").collect();
            if test(&valid_bags ,bags[1]) {
                if !valid_bags.iter().any(|&i| i==bags[0]) {
                    valid_bags.push(bags[0]);
                }
            }
            
        } 

        println!("{}",valsize);

        if valid_bags.len() == valsize{
            done = true;
        }
        else {
            valsize = valid_bags.len();
        }
    }
}

fn test(valid_bags: &[&str], bag: &str) -> bool { 
    for vbag in valid_bags {
        if bag.contains(vbag) {
            return true;
        }
    }
    return false;
}