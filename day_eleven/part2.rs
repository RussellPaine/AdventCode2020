use std::fs;
fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect(); 
    let mut grid = vec![vec![0i8; lines[0].len()]; lines.len()];
    for (i,line) in lines.iter().enumerate() {
        for (j,ch) in line.chars().enumerate() {
            match ch {
                'L' => {
                    grid[i][j] = 1;
                }
                '.' => {
                    grid[i][j] = 0;
                }
                _ =>  { println!("Uh-oh"); continue },
            }
        }
    }
    let mut mutgrid = grid;
    let mut done = true;
    while done {
        let imgrid = mutgrid.to_owned();
        for (i, line) in imgrid.iter().enumerate() {
            for (j, ch) in line.iter().enumerate() { 
                if imgrid[i][j] == 0 {
                    continue;
                }
                let mut seatcount = 0;
                for k in -1..2_i32 {
                    for l in -1..2_i32 { 
                        if k == 0 && l == 0 {
                            continue;
                        }
                        let mut count = 1;
                        loop {
                            let x = (i as i32 + (k * count)) as i32;
                            let y = (j as i32 + (l * count)) as i32;
                            count += 1;
                            if x < 0 || y < 0 || x > (imgrid.len() - 1) as i32 || y > (line.len() - 1) as i32 {
                                break;
                            }
                            if imgrid[x as usize][y as usize] == 1 {
                                break;
                            } 
                            if imgrid[x as usize][y as usize] == 3 {
                                seatcount += 1;
                                break;
                            }
                        }
                    }
                }
                if seatcount >= 5 {
                    mutgrid[i][j] = 1;
                }
                else if seatcount == 0 {
                    mutgrid[i][j] = 3;
                }
            }
        }
        if imgrid == mutgrid {
            let mut count = 0;
            for (i, line) in imgrid.iter().enumerate() {
                for (j, ch) in line.iter().enumerate() {  
                    //print!("{}", ch);
                    if imgrid[i][j] == 3 {
                        count += 1;
                    }
                }
                //println!();
            }
            println!("{}", count);
            done = false;
        }
    }
}