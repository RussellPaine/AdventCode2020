use std::fs;

fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n\n").collect();
    let mut c = 0;

    for line in lines {
        // let mut y: Vec<char> = Vec::new();

        // for elem in line.chars() {
        //     if elem == '\n' {
        //         continue;
        //     }
        //     if !y.contains(&elem){
        //         y.push(elem);
        //     }
        // }
        // c = c + y.len();

        let people: Vec<&str> = line.split_whitespace().collect();
        let alpha = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
        let mut y = 0;
        
        'outer: for a in alpha.iter() {
            for peep in &people {

                if !peep.contains(a) {
                    continue 'outer;
                }
            }
            print!("{}",a);
            y += 1;
        }
        c = c + y;
        print!("{}",c);
        println!();
    }
    println!("{}", c);
}
