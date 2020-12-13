use std::fs;
fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();
    
    let mut dir = 90;

    let mut up = 0;
    let mut side = 0;

    let mut mup = 1;
    let mut mside = 10;

    for line in lines {
        match line.chars().next().unwrap() {
            'N' => {
                mup += line[1..].parse::<i32>().unwrap();
            }
            'S' => {
                mup += -line[1..].parse::<i32>().unwrap();
            }
            'E' => {
                mside += line[1..].parse::<i32>().unwrap();
            }
            'W' => {
                mside += -line[1..].parse::<i32>().unwrap();
            }
            'L' => {
                let mut temp = line[1..].parse::<i32>().unwrap();
                temp = 360 - temp;
                while temp > 0 {
                    temp -=90;
                    let tmup = duplicate(mup);
                    let tmside = duplicate(mside);
                    mup = -tmside;
                    mside = tmup;
                }
            }
            'R' => {
                let mut temp = line[1..].parse::<i32>().unwrap();
                println!("{}-{}", mup, mside);
                while temp > 0 {
                    temp-=90;
                    let tmup = duplicate(mup);
                    let tmside = duplicate(mside);
                    mup = -tmside;
                    mside = tmup;
                }
                println!("{}-{}", mup, mside);
            }
            'F' => {
                up += mup * line[1..].parse::<i32>().unwrap();
                side += mside * line[1..].parse::<i32>().unwrap();
            }
            _ =>  { println!("Uh-oh"); continue },
        }
    }
    if up < 0 {
        up *= -1;
    }
    if side < 0 {
        side *= -1;
    }
    println!("{}-{}", side, up);
    println!("{}", (up + side));

}

fn duplicate<T>(x: T) -> T { x }