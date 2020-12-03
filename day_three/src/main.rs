use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", tobog(1, 1));
    println!("{}", tobog(3, 1));
    println!("{}", tobog(5, 1));
    println!("{}", tobog(7, 1));
    //println!("{}", tobog(1, 2));
}

fn tobog(m_across: usize, m_down: i32) -> i32 {
    let mut f = BufReader::new(File::open("map.txt").unwrap());
    let mut f2 = BufReader::new(File::open("map.txt").unwrap());
    let mut num_line = String::new();
    let mut num_line2 = String::new();

    f.read_line(&mut num_line).unwrap();
    f2.read_line(&mut num_line2).unwrap();

    let n = num_line2.len() - 1;

    let mut trees = 0;
    let mut across = m_across + 1;
    let mut down = m_down;

    for (i, line) in f.lines().enumerate() {
        // if i == 0 {
        //     continue;
        // }

        // if i == 0 {
        //     down = m_down;
        // }else{
        //     down -= 1;
        //    continus 
        // }

        for (j, number) in line.unwrap().chars().enumerate() {
            if j == (across - 1) {
                across += m_across;
                if number == '#' {
                    trees += 1;
                }
                if across > n {
                    across -= n
                }
                break;
            }
        }
    }

    trees
}