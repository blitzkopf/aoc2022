use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;

#[derive(PartialEq,Debug)]
struct Assignment {
  elf1:Range<u8>,
  elf2:Range<u8>
}
impl Assignment {
  pub fn from_str (input:&str) -> Option<Assignment> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    match RE.captures(input) {
      Some(cap) => {
        let r1 = Range { 
          start: cap.get(1).unwrap().as_str().parse::<u8>().unwrap(),
          end: cap.get(2).unwrap().as_str().parse::<u8>().unwrap(),
        };
        let r2 = Range { 
          start: cap.get(3).unwrap().as_str().parse::<u8>().unwrap(),
          end: cap.get(4).unwrap().as_str().parse::<u8>().unwrap(),
        };
        Some(Assignment {elf1:r1,elf2:r2})

      },
      None => {None}
    }
  }
  pub fn overlap(&self) -> Option<Range<u8>> { 
    let start = std::cmp::max(self.elf1.start,self.elf2.start);
    let end  = std::cmp::min(self.elf1.end,self.elf2.end);
    if  start <= end {
      Some(Range {start:start , end:end})
    } else {
      None
    }
  }
  pub fn is_contained(&self) -> bool { 
    match self.overlap() {
      None=> false,
      Some(ol) => { 
        if ol == self.elf1 || ol == self.elf2 { 
          true
        } else {
          false
        }
      } 
    }
  }
}
pub struct Tasklist {
  assignments:Vec<Assignment>
}

impl Tasklist {
  pub fn new(input:&str) -> Tasklist {
    let mut assmts=Vec::new();
    for line in input.split("\n") {
      match Assignment::from_str(line) {
        Some(ass) => assmts.push(ass),
        None => {}
      }
    }
    Tasklist { assignments: assmts }
  }
  pub fn count_contains(&self) -> u32 {
    let mut res=0;
    for ass in self.assignments.iter() {
      if ass.is_contained() {
         res += 1 
      } 
     }
    res
  }
  pub fn count_overlaps(&self) -> u32 {
    let mut res=0;
    for ass in self.assignments.iter() {
      match ass.overlap() {
        Some(ol) =>         res += 1 ,
        None => {}
      } 
     }
    res
  }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_read_assignment () {
      let ass = Assignment::from_str("2-4,6-8").unwrap();
      assert_eq!(ass.elf1, Range{start:2,end:4});
      assert_eq!(ass.elf2, Range{start:6,end:8});

      assert_eq!(ass.overlap(), None);
    }
    #[test]
    fn test_overlaps () {
      /*5-7,7-9
2-8,3-7 */
      let ass = Assignment::from_str("5-7,7-9").unwrap();
      assert_eq!(ass.overlap(), Some(7..7));
      let ass = Assignment::from_str("2-8,3-7").unwrap();
      assert_eq!(ass.overlap(), Some(3..7));
    }
    #[test]
    fn test_is_contained () {
      /*5-7,7-9
2-8,3-7 */
      let ass = Assignment::from_str("5-7,7-9").unwrap();
      assert_eq!(ass.is_contained(), false);
      let ass = Assignment::from_str("2-8,3-7").unwrap();
      assert_eq!(ass.is_contained(), true);
    }
    #[test]
    fn test_tasklist () {
      let tl = Tasklist::new("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
      assert_eq!(tl.assignments[0],Assignment{elf1:2..4,elf2:6..8});
      assert_eq!(tl.count_contains(),2);
      assert_eq!(tl.count_overlaps(),4);

    }
}