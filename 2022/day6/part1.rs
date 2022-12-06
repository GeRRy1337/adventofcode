use std::{fs};

fn main() {

    let contents = fs::read_to_string("/home/gerry/Desktop/Rust/aoc/adventofcode/2022/day6/day6.txt").unwrap();
    let mut db=0;
    let mut curr:String="".to_string();
    let mut found=false;
    for c in contents.chars(){
        db+=1;
        if curr.len() < 3 {
            curr+=&c.to_string();
        }else{
            curr+=&c.to_string();
            println!("{}",curr);

            found=false;

            for i in  0..curr.len(){
                let ch=curr.clone().chars().nth(i).unwrap();
                for y in i+1..curr.len() {
                    if ch == curr.clone().chars().nth(y).unwrap() {
                        found=true;
                        break;
                    }
                }
            }
            
            curr.remove(0);

            if !found {
                break;
            }

        }

    }
    
    println!("db: {}",db)
}