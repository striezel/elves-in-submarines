#[derive(Clone)]
pub struct SlidingWindow
{
  data: [Option<i32>; 3]
}

impl SlidingWindow {
  /**
   * Creates a new SlidingWindow with no data.
   */
  pub fn new() -> SlidingWindow
  {
    SlidingWindow { data: [None; 3] }
  }

  /**
   * Gets the sum of the data elements.
   */
  pub fn sum(&self) -> i32
  {
    let mut sum = 0;
    for element in self.data.iter()
    {
      if let Some(value) = element
      {
        sum += value;
      }
    }
    sum
  }

  /**
   * Pushes a new element to the back of the window.
   *
   * @param data   the new element to push
   * @return       Returns true, if the new element could be pushed.
   *               Returns false, if the element was full.
   */
  pub fn push_back(&mut self, data: &i32) -> bool
  {
    for item in self.data.iter().enumerate()
    {
      let (idx, value) = item;
      if let None = value
      {
        self.data[idx] = Some(*data);
        return true;
      }
    }
    false
  }

  /**
   * Checks whether the window is full
   */
  pub fn full(&self) -> bool
  {
    self.data[0].is_some() && self.data[1].is_some() && self.data[2].is_some()
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn sw_push()
  {
    let mut window = SlidingWindow::new();
    assert!(window.push_back(&1));
    assert!(window.push_back(&3));
    assert!(window.push_back(&5));
    assert!(!window.push_back(&7));
    assert!(!window.push_back(&8));
  }

  #[test]
  fn sw_sum()
  {
    let mut window = SlidingWindow::new();
    assert_eq!(0, window.sum());
    window.push_back(&1);
    assert_eq!(1, window.sum());
    window.push_back(&2);
    assert_eq!(3, window.sum());
    window.push_back(&3);
    assert_eq!(6, window.sum());
  }

  #[test]
  fn sw_full()
  {
    let mut window = SlidingWindow::new();
    assert!(!window.full());
    window.push_back(&1);
    assert!(!window.full());
    window.push_back(&2);
    assert!(!window.full());
    window.push_back(&3);
    assert!(window.full());
  }
}