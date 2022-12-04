use std::fs;

fn main() {

    let contents = fs::read_to_string("/home/gerry/Desktop/Rust/aoc/adventofcode/2022/day4/day4.txt").expect("");
    let mut sum=0;
    for line in contents.lines(){
        let mut pairs = line.split(",");
        let first = pairs.clone().nth(0).expect("").split("-");
        let second = pairs.nth(1).expect("").split("-");
        
        if first.clone().nth(0).expect("").parse::<i32>().unwrap() <= second.clone().nth(0).expect("").parse::<i32>().unwrap() && first.clone().nth(1).expect("").parse::<i32>().unwrap() >= second.clone().nth(1).expect("").parse::<i32>().unwrap() {
            sum+=1;
        }else if first.clone().nth(0).expect("").parse::<i32>().unwrap() >= second.clone().nth(0).expect("").parse::<i32>().unwrap() && first.clone().nth(1).expect("").parse::<i32>().unwrap() <= second.clone().nth(1).expect("").parse::<i32>().unwrap() {
            sum+=1;
        }
    }
    println!("sum: {}", sum);

}
