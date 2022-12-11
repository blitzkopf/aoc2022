use std::{collections::HashSet, ops::{Add, AddAssign, Sub}, fmt, cmp::{min,max}};
use num_traits::sign::{abs,signum};
use tuple::*;
use std::char::from_digit;

#[derive(Debug,Hash,Eq,PartialEq,Clone,Copy)]
pub struct  Pos  (i32,i32);

pub enum Direction {
  Up,
  Down,
  Left,
  Right
}

impl Add for Pos {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Pos  {
    Pos(self.0 + other.0, self.1 + other.1)
  }
}

impl Sub for Pos {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Pos  {
    Pos(self.0 - other.0, self.1 - other.1)
  }
}


impl AddAssign for Pos {
  fn add_assign(&mut self, other: Self) {
      *self = Self (self.0 + other.0,self.1 + other.1)
  }
}


impl Pos {
  pub fn follow(&mut self , target:&Self)  {
    let mut diff = *target - *self;
    println!("diff:{:?}",diff);
    if diff.0 == 0|| diff.1 == 0 {
      if abs(diff.0)>1 {
        diff.0 = signum(diff.0)
      } else {
        diff.0 = 0
      }
      if abs(diff.1)>1 {
        diff.1 = signum(diff.1)
      } else {
        diff.1 = 0
      }
    } else if  abs(diff.0)>1 ||  abs(diff.1)>1 {
      diff.0 = signum(diff.0);
      diff.1 = signum(diff.1);

    } else {
      diff.0 = 0;
      diff.1 = 0;
    }     
    println!("fixed:{:?}",diff);

    *self += diff;
    
  } 
}
pub struct RopeModel {
  head:Pos,
  tails:Vec<Pos>,
  positions:HashSet<Pos>,
  min:Pos,
  max:Pos,
}

impl RopeModel {
  pub fn new(tailcount:usize) -> RopeModel {
    let tails :Vec<Pos> = vec![Pos(0,0);tailcount]; 
    RopeModel { head: Pos(0,0), tails: tails, positions: HashSet::new(),min:Pos(0,0),max:Pos(0,0) }
  }

  pub fn move_head(&mut self,direction:Direction,steps:usize) {
    for i in 0..steps {
      self.head += match direction {
        Direction::Up => Pos(0,1),
        Direction::Down => Pos(0,-1),
        Direction::Right => Pos(1,0),
        Direction::Left => Pos(-1,0),
      };
      //self.min=min(self.min,self.head);
      self.min.0 = min(self.min.0,self.head.0);
      self.min.1 = min(self.min.1,self.head.1);
      self.max.0 = max(self.max.0,self.head.0);
      self.max.1 = max(self.max.1,self.head.1);

      let mut target = &self.head;
      for tail in self.tails.iter_mut() {
        tail.follow(target);
        target = tail;
      }
      self.positions.insert(*target);
    }
  }
  pub fn run_script(&mut self,script:&str) {

    for line in script.split("\n") {
      println!("\n{}",line);
      if line == ""  {continue};
      let linesplit:Vec<&str> = line.split(" ").collect();
      let steps = linesplit[1].parse::<usize>().unwrap();
      let dir = match linesplit[0] {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Unknown direction")
          
      };
      self.move_head(dir, steps);
      println!("{}",self);


    }
  }
  pub fn get_results(self) -> usize {
    self.positions.len()
  }
}

impl fmt::Display for RopeModel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let mut  output:Vec<String> = Vec::new();
      output.push(format!("min:{} {} max:{} {} ",self.min.0,self.min.1,self.max.0,self.max.1));
      for row in (self.min.1 .. self.max.1+1).rev() {
        let mut chars:Vec<char> = Vec::new();
        for col in self.min.0 .. self.max.0+1 {
          let p = Pos(col,row);
          let mut c = if(self.positions.contains(&p)){
            '#'
          } else {
            '.'
          };
          for (i,tail ) in self.tails.iter().enumerate() {
            if p == *tail { c=from_digit(i as u32,10).unwrap() };
          }
          if p == self.head { c='H' };
          chars.push(c);
        }
        output.push(chars.iter().collect::<String>());
      }
      write!(f,"{}",output.join("\n"))
    }
  }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut rm = RopeModel::new(1);
    rm.move_head(Direction::Right,4);
    assert_eq!(rm.tails[0],Pos(3,0))
  }
  #[test]
  fn test_script() {
    let mut rm = RopeModel::new(1);
    rm.run_script("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
    println!("{}",rm);
    //println!("{:?}",rm.positions);
    assert_eq!(rm.positions.len(),13)
  }

  #[test]
  fn test_script_9() {
    let mut rm = RopeModel::new(9);
    rm.run_script("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
    println!("{}",rm);
    //println!("{:?}",rm.positions);
    assert_eq!(rm.positions.len(),36)
  }
}