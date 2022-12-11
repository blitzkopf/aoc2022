use regex::Regex;
#[derive(Debug,Clone,Copy,PartialEq)]
enum Instruction {
  noop,
  addx(i32)
}

pub struct CPU {
  x: i32,
  pc: usize,
  ci: Instruction,
  cycles_remaining: u32,
  program: Vec<Instruction>,
  clock: i32,
  buffer: Vec<char>
}

impl CPU {
  pub fn new(input:&str) -> CPU {
    let prog_re=Regex::new(r"(\w{4})(\s+(-?\d+))?").unwrap();
    let mut program= Vec::new();
    for line in input.split("\n") {
      match prog_re.captures(line) {
        Some(cap) => { 
          let statement=cap.get(1).unwrap().as_str();
          let arg = match cap.get(3) {
            Some(num) => num.as_str().parse::<i32>().unwrap(),
            None => 0,
          };
          let instr = match statement {
            "noop" => Instruction::noop,
            "addx" => Instruction::addx(arg),
            _ => panic!() 
          };
          program.push(instr);
        },
        None => {} 
      }
    }
    CPU {x:1,pc:0,cycles_remaining:0,ci:Instruction::noop,program:program,clock:0,buffer:Vec::new()}
  }
  pub fn tick(&mut self)  {

    let pos = ( self.clock % 40 ) + 1 ;
    self.clock += 1;
    if self.x >= pos -2  && self.x <= pos + 0 {
      self.buffer.push('#');
    } else {
      self.buffer.push('.');
    }
    (self.ci,self.cycles_remaining) =  match self.cycles_remaining {
      0 => {
        self.pc += 1;
        match self.program.get(self.pc-1) {
          Some(instr) => match instr {
            Instruction::noop => {(Instruction::noop,0)},
            Instruction::addx(v) => {(Instruction::addx(*v),1)},
          },
          None => (Instruction::noop,0)
        }
      },
      1 => 
        match self.ci {
          Instruction::noop => {(Instruction::noop,0)},
          Instruction::addx(num) =>  { self.x += num; (Instruction::noop,0) },
        },
      _ => (self.ci,self.cycles_remaining-1)
    };
  }
  pub fn run(&mut self,ticks:u32) {
    for i in 0..ticks {
      self.tick();
    } 
  }
  pub fn get_results( &mut self ) -> i32{
    let mut result=0;
    self.run(19);
    println!(":20 * {} = {}",self.x,20*self.x);
    let mut result=20*self.x;
    for i in 0..5 {
      self.run(40);
      let mply = 20+40*(i+1);
      println!(":{} * {} = {}",mply,self.x,mply*self.x);
      result += mply*self.x
    }
    self.run(21);
    println!("{}\n{}\n{}\n{}\n{}\n{}",
      self.buffer[0..40].iter().collect::<String>(),
      self.buffer[40..80].iter().collect::<String>(),
      self.buffer[80..120].iter().collect::<String>(),
      self.buffer[120..160].iter().collect::<String>(),
      self.buffer[160..200].iter().collect::<String>(),
      self.buffer[200..240].iter().collect::<String>()
    );
    result
  }

}
#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;
  #[test]
  fn test_load() {
    let mut cpu = CPU::new("noop
addx 3
addx -5");
    assert_eq!(cpu.program, vec![Instruction::noop,Instruction::addx(3),Instruction::addx(-5)]);
    cpu.tick();
    assert_eq!(cpu.x,1);
    cpu.tick();
    assert_eq!(cpu.x,1);
    cpu.tick();
    assert_eq!(cpu.x,4);
    cpu.run(2);
    assert_eq!(cpu.x,-1);

  }

  #[test]
  fn test_run() {
    let mut cpu = CPU::new(
"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");
      assert_eq!(cpu.get_results(),13140);
      assert_eq!(cpu.buffer.iter().collect::<String>(),"##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....")
    }

      
}