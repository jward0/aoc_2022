use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn part_1<R: Read>(io: R) -> Result<i32, Error> {

    let mut cycle: i32 = 0;
    let mut x_reg: i32 = 1;
    let mut summed_signal_strength: i32 = 0;

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = line.unwrap();
        if l[0..4].eq("noop") {
            cycle += 1;
            if cycle % 40 == 20 {
                summed_signal_strength += cycle * x_reg;
            }
        }
        if l[0..4].eq("addx") {
            let x: i32 = l[5..].parse::<i32>().unwrap();
            cycle += 1;
            if cycle % 40 == 20 {
                summed_signal_strength += cycle * x_reg;
            }
            cycle += 1;
            if cycle % 40 == 20 {
                summed_signal_strength += cycle * x_reg;
            }
            x_reg += x;
        }
        if cycle > 220 {
            break
        }
    }
    Ok(summed_signal_strength)
}


fn do_cycle(current_display: &mut [char; 40], cycle: &mut i32, x_reg: &i32) {
    *cycle += 1;
    if *x_reg <= *cycle % 40 && *cycle % 40 <= *x_reg + 2 {
        current_display[((*cycle-1) % 40) as usize] = '#';
    }
    else {
        current_display[((*cycle-1) % 40) as usize] = ' ';
    }
    if *cycle % 40 == 0 {
        let display_string: String = current_display.iter().collect();
        println!("{}", display_string);
    }
}


fn part_2<R: Read>(io: R) -> Result<i32, Error> {

    let mut current_display: [char; 40] = [' '; 40];
    let mut cycle: i32 = 0;
    let mut x_reg: i32 = 1;

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = line.unwrap();
        if l[0..4].eq("noop") {
            do_cycle(&mut current_display, &mut cycle, &x_reg);
        }
        if l[0..4].eq("addx") {
            let x: i32 = l[5..].parse::<i32>().unwrap();
            do_cycle(&mut current_display, &mut cycle, &x_reg);
            do_cycle(&mut current_display, &mut cycle, &x_reg);
            x_reg += x;
        }
    }
    Ok(0)
}


fn main() -> Result<(), Error> {

    println!("{:?}", part_1(File::open("input.txt").unwrap()));
    println!("{:?}", part_2(File::open("input.txt").unwrap()));
    Ok(())
}