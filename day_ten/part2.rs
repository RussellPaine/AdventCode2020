use std::fs;

fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut adapters: Vec<i64> = Vec::new();

    for adapter in lines {
        adapters.push(adapter.parse::<i64>().unwrap());
    }

    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();

    let mut c: Vec<i64> = Vec::new();
    c.push(1);
    let mut ans: i64 = 0;

    for i in 1..adapters.len() {
        ans = 0;
        for j in 0..i {
            if adapters[j] + 3 >= adapters[i] {
                println!("{}", c[j]);
                ans += c[j];
            }
        }
        c.push(ans);
    }

    println!("{}", c[c.len() -1]);
}