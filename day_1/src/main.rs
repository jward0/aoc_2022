use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn speedrun<R: Read>(io: R) -> () {
    let br = BufReader::new(io);
    let mut best: [i64; 3] = [0, 0, 0];
    let mut temp: i64 = 0;
    for line in br.lines() {
        let l = line.unwrap();
        if l.is_empty() {
            if temp > best[2] {
                best[2] = temp;
            }
            if temp > best[1] {
                best[2] = best[1];
                best[1] = temp;
            }
            if temp > best[0] {
                best[1] = best[0];
                best[0] = temp;
            }
            temp = 0;
        }
        else {
            temp += l.parse::<i64>().unwrap();
        }
    }
    println!("{}", best[0]);
    println!("{}", best.into_iter().sum::<i64>());
}


fn sorted_inputs_from_file<R: Read>(io: R) -> Result<Vec<Vec<i64>>, Error> {
    let br = BufReader::new(io);
    let mut output_vec = vec![];
    let new_empty_vec: Vec<i64> = vec![];
    output_vec.push(new_empty_vec);
    for line in br.lines() {
        let line_contents = line?;
        if line_contents.is_empty() {
            let new_empty_vec: Vec<i64> = vec![];
            output_vec.push(new_empty_vec);
        }
        else {
            output_vec.last_mut().unwrap().push(line_contents.parse::<i64>().unwrap());
        }
    }
    Ok(output_vec)
}


fn part_1(input:&Vec<Vec<i64>>) -> Result<i64, Error> {
    let summed_inputs: Vec<i64> = input.into_iter().map(|v| v.into_iter().sum()).collect();
    let max_value: i64 = *summed_inputs.iter().max().unwrap();
    Ok(max_value)
} 


fn part_2(input:&Vec<Vec<i64>>) -> Result<i64, Error> {
    let mut summed_inputs: Vec<i64> = input.into_iter().map(|v| v.into_iter().sum()).collect();
    summed_inputs.sort();
    summed_inputs = summed_inputs.into_iter().rev().collect();
    Ok(summed_inputs[0] + summed_inputs[1] + summed_inputs[2])
}


fn main() -> Result<(), Error> {

    speedrun(File::open("input.txt")?);

    let input = sorted_inputs_from_file(File::open("input.txt")?)?;
    println!("{}", part_1(&input)?);
    println!("{}", part_2(&input)?);


    Ok(())
}
