use std::{fs};
use trees::*;

fn main() {

    let contents = fs::read_to_string("/home/gerry/Desktop/Rust/aoc/adventofcode/2022/day7/day7.txt").unwrap();
    let mut tree=tr("/")/tr("asd")/(tr("dsa")/tr("123")/tr("321"));
    let mut curr:String="root/".to_string();
    for line in contents.lines(){
        if line.contains("$") {
            if line.contains("cd"){
                let temp=line.split(" ").nth(2).unwrap();
                if temp.ne("..") && temp.ne("/"){
                    curr = format!("{}/{}", curr, temp);
                    
                }else{
                    curr = curr[..curr.rfind("/").unwrap()].to_string();
                }
                println!("{}", curr);
            }
        }else{

        }


    }

    println!("{}", tree);
    println!("{:?}", tree.root().data());
    println!("{}", tree.root().data().contains("123"));
    
}