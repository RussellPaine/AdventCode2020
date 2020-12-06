use std::fs;

fn main() {
    let data = fs::read_to_string("input.in").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();

    let mut highID = 0;

    let mut IDs: Vec<i32> = Vec::new();
    for line in lines {
        let mut rowrange:[f32; 2] = [0.0, 127.0];
        let mut colrange:[f32; 2] = [0.0, 7.0];

        for elem in line.chars() {
            if elem == 'F' {
                rowrange[1] = ((rowrange[0] + rowrange[1]) / 2.0).floor();
            }
            else if elem == 'B' {
                rowrange[0] = ((rowrange[0] + rowrange[1]) / 2.0).ceil();
            }

            if elem == 'L' {
                colrange[1] = ((colrange[0] + colrange[1]) / 2.0).floor();
            }
            else if elem == 'R' {
                colrange[0] = ((colrange[0] + colrange[1]) / 2.0).ceil();
            }
        }
        if (rowrange[0] as i32 * 8) as i32 + colrange[0] as i32 > highID {
            highID = (rowrange[0] as i32 * 8) as i32 + colrange[0] as i32;
        }
        IDs.push((rowrange[0] as i32 * 8) as i32 + colrange[0] as i32);
    }

    IDs.sort();

    for (i ,elem) in IDs.iter().enumerate() {
        if IDs.len() - 1 == i { continue; }
        if IDs[i+1] - IDs[i] > 1 {
            println!("{}-{}", IDs[i+1], IDs[i] )
        }
    }
}
