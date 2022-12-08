#[derive(Debug)]
struct Tree {
  height:i32,
  visible: bool,
  scenic_score: Option<i32>, 
}
#[derive(Debug)]
pub struct Forrest {
  trees:Vec<Vec<Tree>>
}

impl Forrest {
  pub fn new(input:&str) -> Forrest {
    let mut trees = Vec::new();
    for line in input.split("\n") {
      if line == "" { continue }
      let mut treeline = Vec::new();
      for c in line.chars() {
        println!("c:{c}");
        let height= c.to_digit(10).unwrap() as i32;
        treeline.push(Tree{height:height,visible:false,scenic_score:None})
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

  fn get_tree(&mut self,col:i32,row:i32) -> Option<&mut Tree> {
    if col<0 || row < 0  { 
      return None
    } 
    match self.trees.get_mut(row as usize) {
      None => None,
      Some(tl) => tl.get_mut(col as usize)
    }
  }

  pub fn calc_sceninc_score(&mut self, col:i32,row:i32) {
    /*let  my_tree = match self.get_tree(col,row) {
        Some(tr) => tr,
        None => return,
    };*/
    let my_height = self.trees[row as usize][col as usize ].height;
    let mut val=1;
    for (cd,rd) in [(0,1),(0,-1),(1,0),(-1,0)] {
      let mut i=1;
      let mul = loop {
        match self.get_tree(col+cd*i,row+rd*i) {
          None       => break i-1,
          Some(tree) => if tree.height >= my_height {
            break i
          } else {
            i += 1;
          }
        }
      };
      val *= mul
    }
    self.trees[row as usize][col as usize ].scenic_score=Some(val);
  }
  pub fn run_calc(&mut self)  {
    self.check_visibility();
    for row in 0..self.trees.len() {
      for col in 0..self.trees[row].len() {
        self.calc_sceninc_score(col as i32, row as i32)
      }
    }
  } 


  pub fn get_results(&self ) -> (u32,i32) {
    let mut res=0;
    for tl in self.trees.iter() {
      for t in tl.iter() {
        if t.visible {res += 1}
      }
    }
    let mut ssv:Vec<i32> = Vec::new();
    for tl in &self.trees {
      let mut satt = tl.iter()
      .map(|tr| tr.scenic_score.unwrap())
      .collect::<Vec<i32>>();
      ssv.append(&mut satt);
    }
    ssv.sort_by(|a ,b | b.cmp(a));
    let mult = ssv[0]/* *ssv[1]*ssv[2]*/;
    (res,mult)
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
    //assert_eq!(forrest.count_visable(),21);
  }
  #[test]
  fn test_scenic_score() {
    let mut forrest=Forrest::new("30373
25512
65332
33549
35390
");
    forrest.run_calc();
    println!("{forrest:?}");

    assert_eq!(forrest.trees[2][1].scenic_score,Some(6));
    assert_eq!(forrest.trees[3][2].scenic_score,Some(8));
    assert_eq!(forrest.get_results(),(21,8))
  }
}