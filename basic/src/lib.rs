use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

/**
 * Gets the name of the input file from the command line arguments.
 * Returns "input.txt", if no arguments are given.
 */
pub fn get_input_name() -> String
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1
    {
        args[1].to_owned()
    } else {
        String::from("input.txt")
    }
}

pub fn read_lines_from_file(p: PathBuf) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(p)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_input_name_default()
    {
        assert_eq!("input.txt", get_input_name());
    }

    #[test]
    fn read_lines_from_file_ok()
    {
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        let result = read_lines_from_file(PathBuf::from(cargo_dir.to_string() + "/input.txt"));
        assert!(result.is_ok());
        let result = result.unwrap();
        let mut lines = Vec::new();
        for element in result
        {
            if let Ok(line) = element
            {
                lines.push(line);
            }
        }

        assert_eq!(5, lines.len());
        assert_eq!(vec!{ "abc", "123", "foo", "over 9000", "The End."}, lines);
    }

    #[test]
    fn read_lines_from_file_fail()
    {
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        let result = read_lines_from_file(PathBuf::from(cargo_dir.to_string() + "/this_fails.txt"));
        assert!(result.is_err());
    }
}
