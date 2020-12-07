use std::fs;

fn main() {

    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();

    println!("{}", find_and_count("shiny gold".to_string(), 1, &lines));
}

fn find_and_count(search: String, multiplier: i32, lines: &Vec<&str>) -> i32 {
    let mut count = 0;
    for line in lines {
        let bags: Vec<&str> = line.split(" bags contain").collect();
        if bags[0].contains(&search) {
            if bags[1].contains("no") {
                break;
            }
            let bags2: Vec<&str> = bags[1].split(",").collect();
            let mut tempcount = 0;
            for bag in bags2 {
                let elem: Vec<&str>  = bag.split_whitespace().collect();
                let mut name: String = elem[1].to_string();
                name.push_str(" ");
                name.push_str(&elem[2].to_string());

                tempcount += elem[0].parse::<i32>().unwrap();

                count +=  multiplier * find_and_count(name, elem[0].parse::<i32>().unwrap(), &lines);
            }
            count += multiplier * tempcount;
        }
    }
    return count;
}