use basic::*;
use std::path::PathBuf;

fn main() {
    println!("Day 02: Diving");

    let file_name = get_input_name();
    let lines = read_lines_from_file(PathBuf::from(&file_name));
    if lines.is_err()
    {
        eprintln!("Error: Could not read input from file {}!", file_name);
        return;
    }
    let lines = lines.unwrap();
    let mut depth = 0;
    let mut horizontal = 0;
    for l in lines
    {
        if let Ok(line) = l
        {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2
            {
                eprintln!("Error: Invalid input '{}' encountered!", &line);
                return;
            }
            let number = parts[1].parse::<i32>();
            if number.is_err()
            {
                eprintln!("Error: Could not parse input '{}' as number!", &parts[1]);
                return;
            }
            let number = number.unwrap();
            match parts[0]
            {
                "forward" => horizontal += number,
                "down" => depth += number,
                "up" => depth -= number,
                _ => {
                    eprintln!("Error: '{}' is not a valid movement direction!", &parts[0]);
                    return;
                }
            }
        }
    }

    println!("Horizontal position is {} navigational units.", &horizontal);
    println!("Depth is {} navigational units.", &depth);
    println!("The product of both is {} navigational square units.", depth * horizontal);
}
