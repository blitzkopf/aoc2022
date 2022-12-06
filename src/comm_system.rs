use std::collections::VecDeque;

pub fn find_marker(input:&str,len:usize) -> Option<usize> {
  let mut last_four:VecDeque<char> = VecDeque::new();
  for (i,c) in input.chars().enumerate() {
    println!("{}:{}  in  {:?}",i,c,last_four);

    match last_four
      .iter()
      .enumerate()
      .find(|a|  *a.1==c ) {
    
      Some(v) => { 
        //last_four.clear();
        //last_four.pop_front();
        last_four.drain(0..v.0+1);
        last_four.push_back(c);
      },
      None =>  {
        if last_four.len()< len-1 {
          last_four.push_back(c);
        } else {
          return Some(i+1)
        }
       }
    }

  }
  None
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reading () {
    assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb",4),Some(7));
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz",4),Some(5));
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg",4),Some(6));
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4),Some(10));
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4),Some(11));
  }
  #[test]
  fn test_reading2 () {
    assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14),Some(19));
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz",14),Some(23));
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg",14),Some(23));
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",14),Some(29));
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",14),Some(26));
  }
}