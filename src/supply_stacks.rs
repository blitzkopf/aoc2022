use std::{collections::HashMap, cmp::min};
use itertools::Itertools;  // itertools = "0.8"


use lazy_static::lazy_static;

#[derive(Debug,Clone)]
struct Stack (Vec<char>);
use regex::Regex;

impl Stack {
  pub fn new () -> Stack {
    Stack (Vec::new())
  }
  pub fn add_bottom(&mut self, element:char) {
    self.0.insert(0, element)
  }

  pub fn pop(&mut self,count:u32) -> Vec<char> {
    let mut res = Vec::new();
    for i in 0..count {
      match self.0.pop() {
        Some(c) => res.insert(0,c),// res.push(c),
        None => {}
      }
    }
    res
  }
  pub fn push(&mut self,elements:Vec<char> ) {
    for elem in elements {
      self.0.push(elem);
    }
  }
}
#[derive(Debug,PartialEq)]
struct Move {
  count:u32,
  from:usize,
  to:usize
}

#[derive(Debug)]
pub struct Crane {
  stacks:Vec<Stack>,
  script:Vec<Move>
}

impl Crane {
  pub fn new<'a>(input:&str) -> Crane {
    lazy_static! {
      /* 
[D]    
[N] [C]    
[Z] [M] [P]
 1   2   3  
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
*/
      //static ref STACKS_RE: Regex = Regex::new(r"(([\w]|\s{3})\s)+").unwrap();
      static ref STACKS_RE: Regex = Regex::new(r"\[(\w)\]").unwrap();
      static ref MOVE_RE: Regex = Regex::new(r"move\s+(\d+)\s+from\s+(\d+)\s+to\s(\d+)").unwrap();

    }
    let mut  stacks:HashMap<u8, Stack>=HashMap::new();
    let mut script:Vec<Move> = Vec::new();


    for line in input.split("\n") {
      if STACKS_RE.is_match(line) {
        let mut  sn = 0;

        for i in 0..(line.len()/4+1) {
          let bit = &line[i*4..min(i*4+4,line.len())];
          println!("Bit {bit}");
          match STACKS_RE.captures(bit) {
            Some(cap) => { 
              let c = cap.get(1).unwrap().as_str().chars().nth(0).unwrap();;
              println!("{:?}",c);
              match stacks.get_mut(&sn) {
                Some(st) =>  {st.add_bottom(c);},
                None => {
                  let mut st =  Box::new(Stack::new()); 
                  st.add_bottom(c); 
                  stacks.insert(sn, *st); 
                }
              }
            },
            None => {}
          }   
          
          sn+=1;
        }
      } else {
        match MOVE_RE.captures(line) {
          Some(cap) => script.push(
            Move{
              count:cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
              from:cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
              to:cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            }),
          None => {}
        }
      }
    }
    
    
    /*let st = stacks
      .iter()
      .sorted_by(|a, b | Ord::cmp(&a.0, &b.0) )
      //.collect();
      .map(|k | k.1.clone())
      .collect();*/

    Crane { stacks:stacks
      .iter_mut()
      .sorted_by(|a, b | Ord::cmp(&a.0, &b.0) )
      //.collect();
      .map(|k | k.1.clone())
      .collect(),
      script:script 
    }


  }

  pub fn move_crates(&mut self,count:u32,from:usize,to:usize) {
    let v = self.stacks[from-1].pop(count);
    self.stacks[to-1].push(v);
  }
  pub fn read_tops(&mut self)  -> String {
    let mut chars:Vec<char> = Vec::new();
    for st in self.stacks.iter() {
      chars.push(*st.0.last().unwrap())
    }
    chars.iter().into_iter().collect()

  }
  pub fn run(&mut self) -> String {
    let i = self.script.iter();
    for step in i {
      //self.move_crates(step.count,step.from, step.to)
      let v = self.stacks[step.from-1].pop(step.count);
      self.stacks[step.to-1].push(v);
      println!("{:?}",self.stacks);
    }
    //"AA".to_string()
    self.read_tops()
  }
    
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reading () {
    let cr = Crane::new(
"    [D]    
[N] [C]    
[Z] [M] [P]
  1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
");
    println!("{:?}",cr);
    assert_eq!(cr.stacks[0].0,vec!['Z','N']);
    assert_eq!(cr.stacks[1].0,vec!['M','C','D']);
    assert_eq!(cr.stacks[2].0,vec!['P']);
    assert_eq!(cr.script[0],Move{count:1,from:2,to:1});
  }
  #[test]
  fn test_move () {
    let mut cr = Crane::new(
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
");
    println!("{:?}",cr);
    cr.move_crates(1,2,1);
    assert_eq!(cr.stacks[0].0,vec!['Z','N','D']);
    assert_eq!(cr.stacks[1].0,vec!['M','C']);
    assert_eq!(cr.read_tops(),"DCP");    

    cr.move_crates(3,1,3);
    assert_eq!(cr.stacks[0].0.len(),0);
    assert_eq!(cr.stacks[1].0,vec!['M','C']);
    assert_eq!(cr.stacks[2].0,vec!['P','Z','N','D']);
  }

  #[test]
  fn test_run () {
    let mut cr = Crane::new(
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
");
    println!("{:?}",cr);
    cr.run();
    println!("{:?}",cr);
    assert_eq!(cr.read_tops(),"MCD");
  }
}