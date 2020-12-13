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
                if mutgrid[i][j] == 0 {
                    continue;
                }

                let mut seatcount = 0;
                
                for k in -1..2_i32 {
                    if i == 0 && k == -1 {
                        continue;
                    }
                    if i == imgrid.len() - 1 && k == 1 {
                        continue;
                    }
                    for l in -1..2_i32 {
                        if j == 0 && l == -1 {
                            continue;
                        }
                        if j == line.len() - 1 && l == 1 {
                            continue;
                        }
                        let x: usize = (i as i32 + k) as usize;
                        let y: usize = (j as i32 + l) as usize;
                        if x == i && y == j{
                            continue;
                        }
                        if imgrid[x][y] == 3 {
                            seatcount += 1;
                        }
                    }
                }
                if seatcount >= 4 {
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
                    if imgrid[i][j] == 3 {
                        count += 1;
                    }
                }
                println!();
            }
            println!("{}", count);
            done = false;
        }
        else {
            println!("Failed");
        }

    }
}