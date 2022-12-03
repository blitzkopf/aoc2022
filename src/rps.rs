use std::ops::Add;

#[derive(PartialEq,Debug,Clone,Copy)]
enum RPS {
  Rock,
  Paper,
  Scissors
}
#[derive(PartialEq,Debug,Clone,Copy)]
enum Result {
  Loose,
  Draw,
  Win
}
#[derive(PartialEq,Debug)]
struct Round {
  a:RPS,
  b:RPS
}
#[derive(PartialEq,Debug)]
struct Round2 {
  a:RPS,
  b:Result
}

/*impl Add for (i32,i32) {
  type Output = (i32,i32);
  fn add(self, other: (i32,i32)) -> (i32,i32) {
    (self.0+other.0,self.1+other.1)
  }
}*/
impl Round {
  pub fn new(a:char,b:char) -> Round {
    Round{ 
      a: match a {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!("unknown play {}",a)
      },
      b: match b {
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => panic!("unknown play {}",b)
      },
    }
  }
  pub fn score(&self) -> (i32,i32) {
    match (self.a,self.b) { 
      (RPS::Rock,RPS::Rock) => (3+1,3+1),
      (RPS::Rock,RPS::Paper) => (0+1,6+2),
      (RPS::Rock,RPS::Scissors) => (6+1,0+3),
      (RPS::Paper,RPS::Rock) => (6+2,0+1),
      (RPS::Paper,RPS::Paper) => (3+2,3+2),
      (RPS::Paper,RPS::Scissors) => (0+2,6+3),
      (RPS::Scissors,RPS::Rock) => (0+3,6+1),
      (RPS::Scissors,RPS::Paper) => (6+3,0+2),
      (RPS::Scissors,RPS::Scissors) => (3+3,3+3),
    }

  }
}

impl Round2 {
  pub fn new(a:char,b:char) -> Round2 {
    Round2{ 
      a: match a {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!("unknown play {}",a)
      },
      b: match b {
        'X' => Result::Loose,
        'Y' => Result::Draw,
        'Z' => Result::Win,
        _ => panic!("unknown play {}",b)
      },
    }
  }
  pub fn score(&self) -> (i32,i32) {
    match (self.a,self.b) { 
      (RPS::Rock,Result::Loose,) => (6+1,0+3),
      (RPS::Rock,Result::Draw) => (3+1,3+1),
      (RPS::Rock,Result::Win) => (0+1,6+2),
      (RPS::Paper,Result::Loose,) => (6+2,0+1),
      (RPS::Paper,Result::Draw) => (3+2,3+2),
      (RPS::Paper,Result::Win) => (0+2,6+3),
      (RPS::Scissors,Result::Loose,) => (6+3,0+2),
      (RPS::Scissors,Result::Draw,) => (3+3,3+3),
      (RPS::Scissors,Result::Win) => (0+3,6+1),
    }

  }
}


pub struct Game {
  rounds:Vec<Round>,
  rounds2:Vec<Round2>
}

impl Game {
  pub fn new() -> Game {
    Game{rounds:Vec::new(),rounds2:Vec::new()}
  }
  pub fn read_input(&mut self,data:&str) {
    for line in data.split("\n") {
      if line.len() == 3 {
        self.rounds.push(Round::new(line.chars().nth(0).unwrap(),line.chars().nth(2).unwrap()));
        self.rounds2.push(Round2::new(line.chars().nth(0).unwrap(),line.chars().nth(2).unwrap()));
      }
    }
  }
  pub fn score(&self) ->  (i32,i32) {
    let mut res:(i32,i32)=(0,0);
    for round in self.rounds.iter() {
      let sc = round.score();
      res.0 += sc.0;
      res.1 += sc.1;
    }
    res
  }
  pub fn score2(&self) ->  (i32,i32) {
    let mut res:(i32,i32)=(0,0);
    for round in self.rounds2.iter() {
      let sc = round.score();
      res.0 += sc.0;
      res.1 += sc.1;
    }
    res
  }
}
#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;
  #[test]
  fn test_new_round() {
    let mut rnd = Round::new('A','X');
    assert_eq!(rnd,Round{a: RPS::Rock,b: RPS::Rock});
    assert_eq!(rnd.score(),(4,4));
  
    rnd = Round::new('B','Z');
    assert_eq!(rnd,Round{a: RPS::Paper,b: RPS::Scissors});
    assert_eq!(rnd.score(),(2,9));
    
    rnd = Round::new('C','Y');
    assert_eq!(rnd,Round{a: RPS::Scissors,b: RPS::Paper});
    assert_eq!(rnd.score(),(9,2));
    
  }
  #[test]
  fn test_read_input() {
    let mut game=Game::new();
    game.read_input("A Y
B X
C Z
");
    assert_eq!(game.rounds.len(),3);
    assert_eq!(game.score().1,15);
    assert_eq!(game.score2().1,12);

  }
}