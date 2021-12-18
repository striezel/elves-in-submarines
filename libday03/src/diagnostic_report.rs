use std::fs::File;
use std::io;

pub struct DiagnosticReport
{
  width: usize,
  data: Vec<u32>,
}

impl DiagnosticReport
{
  pub fn from_lines(lines: io::Result<io::Lines<io::BufReader<File>>>) -> DiagnosticReport
  {
    if lines.is_err()
    {
      panic!("Error: Could not read input from file!");
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
            panic!("Error: Cannot handle more than 32 binary digits per number!");
          }
        }
        if line.len() != width.unwrap()
        {
          panic!("Error: Line length is not {} characters!", width.unwrap());
        }
        let current_number = u32::from_str_radix(&line, 2);
        if current_number.is_err()
        {
          panic!("Error: Could not parse input '{}' as binary number!", &line);
        }
        let current_number = current_number.unwrap();
        data.push(current_number);
      }
    }

    if data.is_empty()
    {
      panic!("Error: There is no data in the diagnostic report!");
    }

    DiagnosticReport
    {
      width: width.unwrap(),
      data
    }
  }

  pub fn width(&self) -> usize
  {
    self.width
  }

  pub fn count_ones(&self, shifts_to_left: usize) -> usize
  {
    assert!(shifts_to_left < self.width);

    let mut count_of_ones = 0;
    for num in self.data.iter() {
      if num & (1 << &shifts_to_left) != 0
      {
        count_of_ones += 1;
      }
    }
    count_of_ones
  }

  pub fn majority_threshold(&self) -> usize
  {
    self.data.len() / 2 + 1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn count_ones() {
    let report = DiagnosticReport { width: 5, data: vec!(
      1, // 00001
      2, // 00010
      3, // 00011
      9, // 01001
      31 // 11111
    )};

    assert_eq!(4, report.count_ones(0));
    assert_eq!(3, report.count_ones(1));
    assert_eq!(1, report.count_ones(2));
    assert_eq!(2, report.count_ones(3));
    assert_eq!(1, report.count_ones(4));
  }

  #[test]
  fn majority_threshold() {
    let report = DiagnosticReport { width: 5, data: vec!(
      1, // 00001
      2, // 00010
      3, // 00011
      9, // 01001
      31 // 11111
    )};

    assert_eq!(3, report.majority_threshold());
  }
}
