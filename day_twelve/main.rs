use std::fs;
fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();
    
    let mut dir = 90;
    let mut up = 0;
    let mut side = 0;

  
    let mut mside = 10;
    let mut mup = 1;

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
                dir -= line[1..].parse::<i32>().unwrap();
                if dir >= 360 {
                    dir -= 360;
                }
                if dir < 0 {
                    dir += 360;
                }
            }
            'R' => {
                dir += line[1..].parse::<i32>().unwrap();
                if dir >= 360 {
                    dir -= 360;
                }
                if dir < 0 {
                    dir += 360;
                }
            }
            'F' => {
                match dir {
                    0 => {
                        up *= line[1..].parse::<i32>().unwrap();
                    }
                    90 => {
                        side *= line[1..].parse::<i32>().unwrap();
                    }
                    180 => {
                        up -= line[1..].parse::<i32>().unwrap();
                    }
                    270 => {
                        side -= line[1..].parse::<i32>().unwrap();
                    }
                    _ =>  { println!("Uh-oh2"); continue },
                }
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
    println!("{}", (up + side));

}