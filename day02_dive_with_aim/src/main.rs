use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

fn read_lines_from_file(p: PathBuf) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(p)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Day 02: Diving");

    let args: Vec<String> = std::env::args().collect();
    let file_name = if args.len() > 1
    {
        &args[1]
    }
    else
    {
        "input.txt"
    };
    let lines = read_lines_from_file(PathBuf::from(file_name));
    if lines.is_err()
    {
        eprintln!("Error: Could not read input from file {}!", file_name);
        return;
    }
    let lines = lines.unwrap();
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
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
            let number = i32::from_str_radix(&parts[1], 10);
            if number.is_err()
            {
                eprintln!("Error: Could not parse input '{}' as number!", &parts[1]);
                return;
            }
            let number = number.unwrap();
            match parts[0]
            {
                "forward" => {
                    horizontal += number;
                    depth += &aim * number;
                },
                "down" => aim += number,
                "up" => aim -= number,
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
    println!("Aim factor is {} units.", &aim);
}
