use crate::sliding_window::SlidingWindow;

/// Manages sliding windows.
pub struct Slider
{
  data: Vec<SlidingWindow>,
  increases: usize
}

impl Slider {
  /// Creates a new empty Slider.
  pub fn new() -> Slider
  {
    Slider { data: [].to_vec(), increases: 0 }
  }

  /// Pushes a new data value onto the slider.
  pub fn data(&mut self, value: i32)
  {
    // Push new sliding window.
    self.data.push(SlidingWindow::new());
    // Push new data value to all non-full windows.
    for window in self.data.iter_mut()
    {
      if !window.full()
      {
        window.push_back(&value);
      }
    }
    // Check whether there is an increase.
    if self.data.len() > 1 && self.data[0].full() && self.data[1].full()
    {
      if self.data[0].sum() < self.data[1].sum()
      {
        self.increases += 1;
      }
      // Remove first sliding window, because it's not needed anymore.
      self.data.remove(0);
    }
  }

  /// Gets the number of increases.
  pub fn all_increases(&self) -> usize
  {
    self.increases
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn example()
  {
    let mut slider = Slider::new();

    /* Data from example:
       199  A
       200  A B
       208  A B C
       210    B C D
       200      C D E
       207        D E F
       240          E F G
       269            F G H
       260              G H
       263                H
     */

    slider.data(199);
    slider.data(200);
    slider.data(208);
    slider.data(210);
    assert_eq!(1, slider.all_increases());
    slider.data(200);
    slider.data(207);
    slider.data(240);
    slider.data(269);
    slider.data(260);
    slider.data(263);
    assert_eq!(5, slider.all_increases());
  }
}