extern crate xtree;

use xtree::*;

fn main(){
    let tree =
        tr!(1)
            / (tr!(2)
                / tr!(5)
                / tr!(6))
            / tr!(3)
            / tr!(4);

    let mut cursor = tree.cursor();
    println!("root:{}",cursor.current());
    println!("first child:{}",cursor.move_child().unwrap());
    cursor.move_parent().unwrap();
    cursor.move_child_next();
    println!("second child:{}",cursor.move_child().unwrap());

    for itr in tree.dfs_iter() {
        print!("{} ",itr);
    }
    println!();

    for (depth,node) in tree.dfs_iter().depth(){
        for _ in 0..depth {
            print!("-");
        }
        println!("{}",node);
    }
}