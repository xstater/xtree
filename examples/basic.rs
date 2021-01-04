extern crate xtree;

use xtree::*;

fn main(){
    let mut tree =
        tr!(1)
            / (tr!(2)
                / tr!(5)
                / tr!(6))
            / (tr!(3)
                / tr!(7)
                / tr!(8))
            / tr!(4);

    let mut cursor = tree.cursor_mut();
    cursor.move_child(0);
    *cursor.current() = 10;
    cursor.move_parent();
    cursor.move_child(1);
    let sub_tree = cursor.remove().unwrap();
    for (depth,node) in sub_tree.df_iter().depth(){
        for _ in 0..depth {
            print!("-");
        }
        println!("{}",node);
    }

    let mut cursor = tree.cursor();
    println!("children of root:");
    for child in cursor.children() {
        print!("{} ",child)
    }
    println!();
    println!("root:{}",cursor.current());
    cursor.move_child(0);
    println!("first child:{}",cursor.current());
    cursor.move_parent();
    cursor.move_child(1);
    println!("second child:{}",cursor.current());

    for itr in tree.df_iter_mut() {
        *itr += 1;
        print!("{} ",itr);
    }
    println!();

    for (depth,node) in tree.df_iter().depth(){
        for _ in 0..depth {
            print!("-");
        }
        println!("{}",node);
    }

    for itr in tree.bf_iter(){
        print!("{} ",itr);
    }
}