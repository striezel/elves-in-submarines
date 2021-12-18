use std::path::PathBuf;
use basic::*;

fn main() {
    println!("Day 03: Binary diagnostic");

    let file_name = get_input_name();
    let lines = read_lines_from_file(PathBuf::from(&file_name));
    if lines.is_err()
    {
        eprintln!("Error: Could not read input from file {}!", file_name);
        return;
    }
    let lines = lines.unwrap();
    let mut width: Option<usize>  = None;
    let mut data: Vec<u32> = Vec::new();
    for l in lines
    {
        if let Ok(line) = l
        {
            if width.is_none()
            {
                width = Some(line.len());
                if width.unwrap() > 32
                {
                    eprintln!("Error: Cannot handle more than 32 binary digits per number!");
                    return;
                }
            }
            if line.len() != width.unwrap()
            {
                eprintln!("Error: Line length is not {} characters!", width.unwrap());
                return;
            }
            let current_number = u32::from_str_radix(&line, 2);
            if current_number.is_err()
            {
                eprintln!("Error: Could not parse input '{}' as binary number!", &line);
                return;
            }
            let current_number = current_number.unwrap();
            data.push(current_number);
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    let threshold = data.len() / 2 + 1;
    for i in 0..width.unwrap() {
        let mut count_of_ones = 0;
        for num in data.iter() {
            if num & (1 << i) != 0
            {
                count_of_ones += 1;
            }
        }
        if count_of_ones >= threshold
        {
            gamma.insert(0, '1');
            epsilon.insert(0, '0');
        }
        else
        {
            gamma.insert(0, '0');
            epsilon.insert(0, '1');
        }
    }
    let gamma_decimal = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_decimal = u32::from_str_radix(&epsilon, 2).unwrap();

    println!("Binary gamma rate is {}. That is {} in decimal.",
             gamma, gamma_decimal);
    println!("Binary epsilon rate is {}. That is {} in decimal.",
             epsilon, epsilon_decimal);
    println!("Therefore, the submarine's total power consumption is {} energy units.",
             gamma_decimal * epsilon_decimal);
}
