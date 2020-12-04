use std::fs;
extern crate regex;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n\n").collect();
    //let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valcount = 0;

    for pass in lines {
        let fields = pass.split_whitespace();
        let mut fieldCount = 0;

        for elem in fields {
            let keyVal: Vec<&str> = elem.split(":").collect();

            // if required.iter().any(|&j| j == keyVal[0]) {
            //     println!("{}", keyVal[0]);
            //     fieldCount += 1;
            // }else {
            //     continue;
            // }

            match keyVal[0] {
                "byr" => {
                    if keyVal[1].parse::<i32>().unwrap() >= 1920
                        && keyVal[1].parse::<i32>().unwrap() <= 2002
                    {
                        fieldCount += 1;
                    }
                }
                "iyr" => {
                    if keyVal[1].parse::<i32>().unwrap() >= 2010
                        && keyVal[1].parse::<i32>().unwrap() <= 2020
                    {
                        fieldCount += 1;
                    }
                }
                "eyr" => {
                    if keyVal[1].parse::<i32>().unwrap() >= 2020
                        && keyVal[1].parse::<i32>().unwrap() <= 2030
                    {
                        fieldCount += 1;
                    }
                }
                "hgt" => {
                    if keyVal[1].contains("cm") {
                        let number = keyVal[1].replace("cm", "").parse::<i32>().unwrap();
                        if number >= 150 && number <= 193 {
                            fieldCount += 1;
                        }
                    } else if keyVal[1].contains("in") {
                        let number = keyVal[1].replace("in", "").parse::<i32>().unwrap();
                        if number >= 59 && number <= 76 {
                            fieldCount += 1;
                        }
                    }
                }
                "hcl" => {
                    let re = Regex::new(r"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap();
                    if re.is_match(keyVal[1]) {
                        fieldCount += 1;
                    }
                }
                "ecl" => {
                    let validEyes = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    if validEyes.iter().any(|&j| j == keyVal[1]) {
                        fieldCount += 1;
                    }
                }
                "pid" => {
                    if keyVal[1].len() == 9 {
                        fieldCount += 1;
                    }
                }
                _ => continue,
            }
            //println!("{}", fieldCount);
            if fieldCount == 7 {
                valcount += 1;
            }
        }
    }
    println!("{}", valcount);
}
