use std::{fs};

fn main() {

    let contents = fs::read_to_string("/home/gerry/Desktop/Rust/aoc/adventofcode/2022/day8/day8.txt").unwrap();
    let mut max=0;
    let mut arr:Vec<Vec<i32>>=Vec::new();
    let mut up:i32;
    let mut down:i32;
    let mut left:i32;
    let mut right:i32;
    for line in contents.lines(){
        let mut temp:Vec<i32>=Vec::new();
        for c in line.chars() {
            temp.push(c as i32);
        }
        arr.push(temp);
    }

    for x in 0..arr.len(){
        for y in 0..arr[x].len() {
            up=0;
            down=0;
            left=0;
            right=0;
            if x > 0 && x < arr.len()-1 && y>0 && y < arr[x].len()-1{
                

                for i in (x+1)..arr.len() {
                    if arr[x][y] <= arr[i][y] {
                        down+=1;
                        break;
                    }else{
                        down+=1;
                    }
                }

                for i in (0..=(x-1) as usize).rev(){
                    if arr[x][y] <= arr[i][y] {
                        up+=1;
                        break;
                    }else{
                        up+=1;
                    }
                }

                for i in (y+1)..arr[x].len(){
                    if arr[x][y] <= arr[x][i] {
                        right+=1;
                        break;
                    }else{
                        right+=1;
                    } 
                }

                for i in (0..=(y-1)).rev() {
                    if arr[x][y] <= arr[x][i] {
                        left+=1;
                        break;
                    }else{
                        left+=1;
                    } 
                }

            }
            if (up*down*left*right) > max {
                max = up*down*left*right ;
                println!("x:{} y:{}, up:{}, down:{}, left:{}, right:{}",x, y, up, down, left, right);
            }
        }
    }
    
    println!("max: {}", max);
}