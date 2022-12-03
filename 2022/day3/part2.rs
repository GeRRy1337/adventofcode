use std::{fs};

fn main() {
    
    let nums=1..=52;
    let chars=('a'..='z').chain('A'..='Z');
    let zipped = chars.into_iter().zip(nums.into_iter());

    let contents = fs::read_to_string("/home/gerry/Desktop/Rust/aoc/adventofcode/day3/day3.txt").expect("");

    let mut sum=0;
    let mut db=0;
    let mut ln1:&str=""; 
    let mut ln2:&str="";

    for line in contents.lines(){
        db += 1;
        println!("{}",line);
        if db == 1{
            ln1=line.clone();
        }else if db == 2 {
            ln2=line.clone();
        }else if db == 3{
            let mut badge='!';
            for c in line.chars() {
                if ln2.contains(c) && ln1.contains(c){
                    badge=c;
                }

            }

            for (c, i) in zipped.clone()
            {
                if badge == c {
                    sum+=i;
                }
            }
            db=0;
        }
    }
    println!("{}", sum);

}
