#[derive(Debug)]
struct Tree {
  height:i32,
  visible: bool,

}
#[derive(Debug)]
pub struct Forrest {
  trees:Vec<Vec<Tree>>
}

impl Forrest {
  pub fn new(input:&str) -> Forrest {
    let mut trees = Vec::new();
    for line in input.split("\n") {
      if(line == "") { continue }
      let mut treeline = Vec::new();
      for c in line.chars() {
        println!("c:{c}");
        let height= c.to_digit(10).unwrap() as i32;
        treeline.push(Tree{height:height,visible:false})
      }
      trees.push(treeline);
    } 
    Forrest{trees:trees}
  }
  pub fn check_visibility(&mut self) {
    let mut heights = vec![-1i32; self.trees[0].len()];
    for treeline in self.trees.iter_mut() {
      let mut lheight=-1;
      for (tree,height) in treeline.iter_mut().zip(heights.iter_mut()) {
        if tree.height>*height {
          tree.visible=true;
          *height = tree.height;
        }
        if tree.height>lheight {
          tree.visible=true;
          lheight = tree.height;
        }

      }
    }
    let mut heights = vec![-1i32; self.trees[0].len()];
    for treeline in self.trees.iter_mut().rev() {
      let mut lheight=-1;
      for (tree,height) in treeline.iter_mut().rev().zip(heights.iter_mut()) {
        if tree.height>*height {
          tree.visible=true;
          *height = tree.height;
        }
        if tree.height>lheight {
          tree.visible=true;
          lheight = tree.height;
        }
      }
    }
  }
  pub fn count_visable(&self ) -> u32 {
    let mut res=0;
    for tl in self.trees.iter() {
      for t in tl.iter() {
        if t.visible {res += 1}
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
  fn test_load_forrest() {
    let forrest=Forrest::new("30373
25512
65332
33549
35390
");
    assert_eq!(forrest.trees.len(),5);
    assert_eq!(forrest.trees[0][0].height,3);
    assert_eq!(forrest.trees[4][4].height,0);
  }
  #[test]
  fn test_visibilty() {
    let mut forrest=Forrest::new("30373
25512
65332
33549
35390
");
    forrest.check_visibility();
    println!("{forrest:?}");

    assert_eq!(forrest.trees[0][0].visible,true);
    assert_eq!(forrest.trees[4][4].visible,true);
    assert_eq!(forrest.count_visable(),21);
  }
}