use std::{collections::HashSet, hash::Hash};

struct Compartnemt {
  contents:HashSet<char>,
}

struct Rucksack {
  c1:Compartnemt,
  c2:Compartnemt
}
pub struct RSCollection {
  rucksacks:Vec<Rucksack>,
}

impl Compartnemt {
  pub fn new(input:&str) -> Compartnemt {
    Compartnemt {
      contents: HashSet::from_iter(input.chars())
    }
  }
}
fn char_to_val(ch:&char) -> u32 {
  let c:u32 = (*ch).into();
  let A:u32 = 'A'.into();
  let a:u32 = 'a'.into();
  println!("{ch} : {c} {A} {a}");
  if c >= a {
    c-a+1
  } else {
    c-A+1+26
  }

}
impl Rucksack {
  pub fn new(input:&str) -> Rucksack {
    let len = input.len();
    Rucksack {
      c1:Compartnemt::new(&input[0..len/2]),
      c2:Compartnemt::new(&input[len/2..len]),
    }
  }
  pub fn get_dupls(&self) -> char {
    let inter = self.c1.contents.intersection(&self.c2.contents);
    *inter.into_iter().next().unwrap()
  }

  pub fn get_value(&self) -> u32 {
    char_to_val(&self.get_dupls())
  }

  pub fn get_contents(&self) -> HashSet<char> {
    //let res:HashSet<char> = HashSet::from_iter(self.c1.contents.union(&self.c2.contents))
    let res:HashSet<char> = self.c1.contents.union(&self.c2.contents)
      .map(|x| *x)
      .collect();
    res
  }
}


impl RSCollection {
  pub fn new() -> RSCollection {
    RSCollection {
      rucksacks: Vec::new()
    }
  }
  pub fn read_input(&mut self,data:&str) {
    for line in data.split("\n") {
      if line != "" {
        self.rucksacks.push(Rucksack::new(line))
      }
    }
  }
  pub fn get_value(&self) -> u32 {
    let mut res=0;
    for rs in self.rucksacks.iter() {
      let val = rs.get_value();
      res += val;
      println!("val:{val} res:{res}")
    }
    res
  }
  pub fn get_value2(&self) -> u32 {
    let mut res=0;
    let mut iter= self.rucksacks.chunks(3);
    for rss in  iter {
      let inters1 = 
        rss[0].get_contents()
        .intersection(&rss[1].get_contents())
        .map(|x| *x)
        .collect();
      let c = 
        *rss[2].get_contents()
        .intersection(&inters1)
        .next()
        .unwrap()
        /* .collect()*/;
    
      res += char_to_val(&c)  ;
      
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

// Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;
  #[test]
  fn test_compartment() {
    let comp=Compartnemt::new("vJrwpWtwJgWr");
    assert_eq!(comp.contents,HashSet::from_iter(vec!['v','J','r','w','p','W','t','g']));
  }
  #[test]
  fn test_rucksack() {
    let rucks=Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
    assert_eq!(rucks.get_dupls(),'p');
    assert_eq!(rucks.get_value(),16)
  }  
  #[test]
  fn test_rucksack2() {
    let rucks=Rucksack::new("ttgJtRGJQctTZtZT");
    assert_eq!(rucks.get_dupls(),'t');
    assert_eq!(rucks.get_value(),20)
  }
  #[test]
  fn test_coll() {
    let mut coll = RSCollection::new();
    coll.read_input("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
  assert_eq!(coll.get_value(),157);
  }
  #[test]
  fn test_coll2() {
    let mut coll = RSCollection::new();
    coll.read_input("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
  assert_eq!(coll.get_value2(),70);
  }
}