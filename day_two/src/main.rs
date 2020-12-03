use std::fs;

fn main() {
    let data = fs::read_to_string("list.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut count = 0;

    for line in lines {
        let lower: usize = line
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .next()
            .unwrap()
            .split("-")
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let upper: usize = line
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .next()
            .unwrap()
            .split("-")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let con: char = line
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<char>()
            .unwrap();

        let passwd: String = line
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .parse::<String>()
            .unwrap();

        //let c = passwd.matches(con).count();

        //if c <= upper && c >= lower {
        //    count += 1;
        //}

        if passwd.chars().nth(lower - 1).unwrap() == con {
            if passwd.chars().nth(upper - 1).unwrap() != con {
                count += 1;
            }
        }
        if passwd.chars().nth(upper - 1).unwrap() == con {
            if passwd.chars().nth(lower - 1).unwrap() != con {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
