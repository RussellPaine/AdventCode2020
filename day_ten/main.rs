use std::fs;
fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut adapters: Vec<i32> = Vec::new();
    for adapter in lines {
        adapters.push(adapter.parse::<i32>().unwrap());
    }
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();
    let mut onecount = 0;
    let mut threecount = 0;
    for i in 0..adapters.len() {
        if i == adapters.len() - 1 {
            break;
        }
        if adapters[i + 1] - adapters[i] == 1 {
            onecount += 1;
        }
        else if adapters[i + 1] - adapters[i] == 3 {
            threecount += 1;
        }
    }
    println!("{}", onecount  * threecount);
}