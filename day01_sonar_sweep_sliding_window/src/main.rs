use basic::*;
use std::path::PathBuf;
use crate::slider::Slider;

mod sliding_window;
mod slider;

fn main() {
    println!("Day 01: Sonar sweep with sliding windows - count depth increases");

    let file_name = get_input_name();
    let lines = read_lines_from_file(PathBuf::from(&file_name));
    if lines.is_err()
    {
        eprintln!("Error: Could not read input from file {}!", file_name);
        return;
    }
    let lines = lines.unwrap();
    let mut slider = Slider::new();
    for line in lines.flatten()
    {
        let current_number = line.parse::<i32>();
        if current_number.is_err()
        {
            eprintln!("Error: Could not parse input '{}' as number!", &line);
            return;
        }
        let current_number = current_number.unwrap();
        slider.data(current_number);
    }

    println!("Depth values of sliding windows in {} have increased {} time(s).",
             &file_name, slider.all_increases())
}
