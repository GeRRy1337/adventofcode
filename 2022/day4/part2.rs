use std::fs;

fn main() {

    let contents = fs::read_to_string("/home/gerry/Desktop/Rust/aoc/adventofcode/2022/day4/day4.txt").expect("");
    let mut sum=0;
    for line in contents.lines(){
        let mut pairs = line.split(",");
        let mut first = pairs.next().unwrap().split("-");
        let mut second = pairs.next().unwrap().split("-");
        let num1: i32 = first.next().unwrap().parse().unwrap();
        let num2: i32 = second.next().unwrap().parse().unwrap();
        let num3:i32 = first.next().unwrap().parse().unwrap();
        let num4:i32 = second.next().unwrap().parse().unwrap();

        if num1 == num2 || num3 == num4 || num1 == num4 || num2 == num3{
            sum += 1;
        }else if num1 < num2 && num3 > num2 {
            sum+=1;
        }else if num1 > num2 && num4 > num1 {
            sum+=1;
        }
    }
    println!("sum: {}", sum);

}
