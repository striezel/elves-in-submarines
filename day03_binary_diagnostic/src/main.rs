use std::path::PathBuf;
use basic::*;
use libday03::DiagnosticReport;

fn main() {
    println!("Day 03: Binary diagnostic");

    let file_name = get_input_name();
    let lines = read_lines_from_file(PathBuf::from(&file_name));
    if lines.is_err()
    {
        eprintln!("Error: Could not read input from file {}!", file_name);
        return;
    }
    let report = DiagnosticReport::from_lines(lines);

    let mut gamma = String::new();
    let mut epsilon = String::new();
    let threshold = report.majority_threshold();
    for i in 0..report.width() {
        let count_of_ones = report.count_ones(i);
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
