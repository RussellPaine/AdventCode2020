use std::fs;

fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut testIndexes: Vec<usize> = Vec::new();

    for i in 0..lines.len() {
        let instruct: Vec<&str> = lines[i].split_whitespace().collect();
        match instruct[0] {
            "nop" => {
                testIndexes.push(i);
                continue;
            }
            "jmp" => {
                testIndexes.push(i);
                continue;
            }
            _ => continue,
        }
    }

    for i in testIndexes {
        let (worked, count) = testfix(&lines, i);
        if worked {
            println!("{}", count);
            break;
        }
    }

}


fn testfix(lines: &Vec<&str>, changeIndex: usize) -> (bool,i32) { 
    let mut count = 0;
    let mut ain: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    let mut worked = false;

    while i <= lines.len() as i32 {
        if i == lines.len() as i32 {
            worked = true;
            break;
        }

        let instruct: Vec<&str> = lines[i as usize].split_whitespace().collect();
        
        if !ain.iter().any(|&j| j==i) {
            ain.push(i);
        }else {
            break;
        }

        match instruct[0] {
            "nop" => {
                if i == changeIndex as i32{
                    i = i + instruct[1].parse::<i32>().unwrap();
                }else {
                    i += 1;
                }
                
                continue;
            }
            "acc" => {
                i += 1;
                count += instruct[1].parse::<i32>().unwrap();
                continue;
            }
            "jmp" => {
                if i == changeIndex as i32 {
                    i += 1;
                }else {
                    i = i + instruct[1].parse::<i32>().unwrap();
                }
                continue;
            }
            _ => continue,
        }
    }

    return (worked, count);
}