use basic::*;
use std::path::PathBuf;

fn main() {
    println!("Day 01: Sonar sweep - count depth increases");

    let file_name = get_input_name();
    let lines = read_lines_from_file(PathBuf::from(&file_name));
    if lines.is_err()
    {
        eprintln!("Error: Could not read input from file {}!", file_name);
        return;
    }
    let lines = lines.unwrap();
    let mut previous: Option<i32>  = None;
    let mut increase_count: usize = 0;
    for l in lines
    {
        if let Ok(line) = l
        {
            let current_number = i32::from_str_radix(&line, 10);
            if current_number.is_err()
            {
                eprintln!("Error: Could not parse input '{}' as number!", &line);
                return;
            }
            let current_number = current_number.unwrap();
            if let Some(prev) = previous
            {
                if prev < current_number
                {
                    increase_count += 1;
                }
            }
            previous = Some(current_number);
        }
    }

    println!("Depth values in {} have increased {} time(s).", &file_name, increase_count)
}
