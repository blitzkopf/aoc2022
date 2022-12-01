struct Elf {
  calories:u64,
}
impl Elf {
  pub fn new() -> Elf
  {
    Elf {calories:0}
  }

  pub fn add(&mut self,calories:u64) {
    self.calories += calories;
  }
}

pub struct CalorieCollector {
  elves: Vec<Elf>,
}

impl CalorieCollector {

  pub fn new() -> CalorieCollector {
    CalorieCollector { elves : Vec::new()}
  }

  pub fn read_input(&mut self,data:&str) {
    let mut elf = Elf::new();
    for line in data.split("\n") {
      if line == "" {
        self.elves.push(elf);
        elf = Elf::new();
      } else {
        elf.add(line.trim().parse::<u64>().unwrap());
      }
    }
    self.elves.push(elf);

  }

  pub fn get_biggest(&self) -> u64 {
    let mut biggest=0u64;
    for elf in self.elves.iter() {
      if elf.calories> biggest {
        biggest = elf.calories
      }
    }
    biggest
  }
  pub fn top_3(&mut self) -> (u64,u64,u64) {

    self.elves.sort_by(|a,b|b.calories.partial_cmp(&a.calories ).unwrap());

    (self.elves[0].calories,self.elves[1].calories,self.elves[2].calories)
  }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
  fn test_calories() {
    let mut cc = CalorieCollector::new();
    cc.read_input(
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
");
    assert_eq!(cc.get_biggest(),24000);
    assert_eq!(cc.top_3(),(24000,11000,10000));
  }
  
}